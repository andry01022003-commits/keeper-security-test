# Reverse proxy timeouts for OCI and Docker pushes

When a client pushes a layer, the final `PUT /v2/<name>/blobs/uploads/<id>?digest=`
does not finish when its body has arrived. The backend then assembles the
upload's parts, re-reads them to verify the digest, writes the verified object
and promotes it to its content-addressed key, and records the blob in the
database. For a layer of several hundred MB or more on S3, NFS or a networked
block volume that work takes tens of seconds, sometimes minutes, and the
backend sends no response bytes until it is done.

A reverse proxy whose response timeout is shorter than that answers the
client with 504 and resets the request to the backend. containerd 2.x (Docker
Engine with the containerd image store, BuildKit, nerdctl) pushes each layer
as one `PUT` carrying the whole layer and re-sends the identical `PUT` once
after a 504, so a short proxy timeout turns every large layer push into a
retry (#4902).

## Recommended settings

Give the `/v2/` routes a response timeout longer than the slowest layer
completion you expect. `3600s` is a safe value for multi-GB layers; disabling
the timeout for `/v2/` is also fine.

### Envoy Gateway (Gateway API `HTTPRoute`)

Envoy's route timeout defaults to **15 s** and is measured from the end of
the request body to the end of the response, so upload time does not count
but the completion work does. Envoy Gateway keeps that default unless the
route sets one:

```yaml
apiVersion: gateway.networking.k8s.io/v1
kind: HTTPRoute
metadata:
  name: artifact-keeper
spec:
  rules:
    - matches:
        - path: { type: PathPrefix, value: /v2/ }
      timeouts:
        request: 3600s          # "0s" disables the timeout
        # backendRequest, if you set it, must not be shorter than request.
      backendRefs:
        - name: artifact-keeper-backend
          port: 8080
```

A `BackendTrafficPolicy` with `spec.timeout.http.requestTimeout` achieves the
same for every route of a gateway. Istio (`VirtualService` `timeout`) and
Contour (`HTTPProxy` `timeoutPolicy.response`) have equivalent settings.

### nginx

`proxy_read_timeout` defaults to **60 s**: the longest gap nginx waits between
two reads from the backend, which during a completion is the whole completion.

```nginx
location /v2/ {
    proxy_pass              http://artifact-keeper-backend:8080;
    proxy_read_timeout      3600s;
    proxy_send_timeout      3600s;
    client_max_body_size    0;      # layer size is enforced by MAX_UPLOAD_SIZE
    proxy_request_buffering off;    # stream layers instead of spooling them
}
```

For the ingress-nginx controller the same values are the
`nginx.ingress.kubernetes.io/proxy-read-timeout`, `proxy-send-timeout` and
`proxy-body-size` annotations.

## What the backend does when the proxy cuts a completion anyway

Since 1.11.7 a completion runs to its end even when the request that started
it is dropped by the proxy or the client:

- A retry of the same `PUT` (same session, same digest) that arrives while the
  completion is still running waits for it, up to
  `OCI_UPLOAD_COMPLETION_RETRY_WAIT_SECS`, and is answered as the completion
  was: `201 Created` with `Location` and `Docker-Content-Digest`, or the same
  error. If the completion is still running when the wait is over, the retry
  gets `503` with `Retry-After: 5`.
- A retry that arrives after the completion finished gets `201 Created` when
  the blob is in the repository.
- A retry with a different digest is still refused with `409`
  (`upload session is already being modified`).
- A completion whose replica died (crash, pod kill) holds the session for at
  most five minutes, after which a retry can take it over. Before 1.11.7 that
  was six hours.

When replicas sit behind a load balancer, a retry that lands on a different
replica than the running completion polls the session instead of waiting on
the completion directly; it settles the same way except that it cannot tell a
different digest apart while the first completion is running.

| Variable | Default | Meaning |
|---|---|---|
| `OCI_UPLOAD_COMPLETION_RETRY_WAIT_SECS` | `30` | How long a retried completion `PUT` waits for the completion it duplicates before answering `503` with `Retry-After`. `0` answers `503` at once. Capped at 3600. |

These behaviours make a cut completion recoverable, but each cut still costs
the client a retry and the layer's upload time again on the next attempt.
Setting the proxy timeouts above is the fix; the retry handling is a safety
net.
