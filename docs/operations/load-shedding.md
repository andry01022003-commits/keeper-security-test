# Overload, load shedding and CPU sizing

How the backend behaves when it gets more work than its CPU can serve, and
how to size a replica so overload shows up as a status code a client retries
instead of a hung or failed connection (#4647).

## The three places a request can wait

1. **Kernel accept queue.** The kernel holds completed TCP handshakes until
   the process accepts them. Its length is `HTTP_LISTEN_BACKLOG`; unset (the
   default) asks for the OS maximum, which Linux caps at
   `net.core.somaxconn` (4096 on current kernels; 128 on kernels before
   5.4). The value the kernel grants is logged at startup ("HTTP listener
   accept backlog"); `ss -ltn` shows it in the `Send-Q` column. When this
   queue is full, Linux **drops** new SYNs (with the default
   `net.ipv4.tcp_abort_on_overflow=0`), so the client's connect stalls and
   retransmits. It does not get a fast refusal.
2. **Global concurrency limit.** `GLOBAL_MAX_CONCURRENCY` (default 512) is the
   number of requests served at once, excluding health and readiness probes.
   A request over the limit is answered **immediately** with `503` and
   `Retry-After`; it never queues. This is the outermost application layer,
   so a shed request does no authentication, database or storage work.
3. **In process.** Below the limit, every accepted request runs. When the CPU
   limit is the bottleneck, they all slow down together.

## Measured behaviour

Debug build on one host, the API listener under a systemd `CPUQuota=20%`.
Each client opens a new connection per request (`Connection: close`) and
sends `GET /api/v1/repositories` for 20 s.

| Clients | `GLOBAL_MAX_CONCURRENCY` | `HTTP_LISTEN_BACKLOG` | Result |
|---|---|---|---|
| 200 | 512 | default (4096) | 100% answered, p50 0.98 s, p99 6.3 s; no connect failures |
| 200 | 16 | default (4096) | 96% `503` + `Retry-After` (p50 0.30 s), 4% served; no connect failures |
| 1000 | 16 | default (4096) | 96% `503` + `Retry-After` (p50 1.6 s), 4% served; no connect failures |
| 1000 | 512 | default (4096) | 78% `503` + `Retry-After` (p50 1.1 s), 22% served; no connect failures |
| 1000 | 512 | 8 | 23% connect timeouts (5 s), 26% response timeouts, 51% served |

The backend's own listener did not refuse a connection in any run. With the
default backlog, overload became `503` + `Retry-After` once concurrency
passed the limit. Below the limit it showed up as latency. A deliberately tiny
backlog shows what a full accept queue looks like: connect **timeouts**, not
refusals.

So a connection that is refused within milliseconds (the symptom in #4647) is
not the backend's accept queue overflowing. Look at what sits between the
client and the pod: an ingress or gateway with its own connection or
pending-request limits, a `kubectl port-forward`, or a load balancer whose
health check briefly removed the endpoint.

## Sizing guidance

- **CPU.** For mixed CI-fleet load (many formats publishing and fetching at
  once), give each backend replica at least 8 cores, or set a CPU *request*
  without a CPU *limit*. In the #4647 release-gate run, 42 concurrent
  mixed-format clients saturated a 4-core limit, and the same load on 8 cores
  peaked below 3 cores of use. CFS throttling at a hard limit stalls the whole
  process, including the accept loop, for the rest of each period.
- **`GLOBAL_MAX_CONCURRENCY`.** The default of 512 is sized so it never
  throttles legitimate parallel traffic on a well-provisioned replica. On a
  CPU-limited replica it is rarely reached, so overload shows up as latency
  and then as client or gateway timeouts. If you would rather clients see a
  fast `503` + `Retry-After`, lower it toward the concurrency the replica
  serves at acceptable latency, for example 32 to 64 per core. Every
  byte-transfer request also holds a permit, so leave room for your
  concurrent uploads and downloads.
- **`HTTP_LISTEN_BACKLOG`.** Leave it unset unless you have a reason. If
  `net.core.somaxconn` in the pod's network namespace is low (128 on older
  kernels), raise the sysctl (for example via the pod `securityContext.sysctls`
  where your cluster allows it) and then, if you want an explicit value, set
  `HTTP_LISTEN_BACKLOG` to at most that.
- **Gateway in front.** Make the gateway's upstream timeout longer than the
  backend's `GLOBAL_REQUEST_TIMEOUT_SECS` (120 s), and make sure it passes the
  backend's `503` and `Retry-After` through rather than turning upstream
  overload into a connection error.

## Buffered metadata memory budget

Proxied metadata that has to be held in memory before it is answered (npm
packuments, the PyPI simple index, Debian `dists`, Helm indexes, VS Code
gallery queries, conda virtual merges) reserves bytes from one process-wide
budget, `AK_PROXY_METADATA_BUDGET_BYTES` (default 1 GiB). When the budget is
held by in-flight requests, a new request waits for room; the callers that
bound their wait (conda virtual merges, VS Code gallery queries) answer `503`
with `Retry-After` instead.

The budget is accounted in 4 KiB units: a reservation is rounded up to a
whole unit, and a single request larger than the whole budget holds all of it
rather than waiting forever. Any value the platform can address is honoured.
Before 1.11.5, a value above 4 GiB was silently clamped to 4294967295 bytes
(#4881), which also capped the default `CONDA_VIRTUAL_MAX_CONCURRENT_MERGES`
at 3. A blank, non-numeric or zero value falls back to the default; that, or
a value too large to represent, logs a warning at startup naming the
configured and effective values. The effective value is in the
`conda virtual merge gate configured` log line (`budget_bytes`), printed when
the first virtual conda merge runs. Sizing against the container memory limit
is in [conda virtual merge memory](conda-virtual-merge-memory.md).

## Database connection budget

Every replica opens its own pool, so size the fleet against the server
(#4549): `DATABASE_MAX_CONNECTIONS` x replicas, plus the single-flight
lock/wait connections held outside the pool (capped per replica by
`PROXY_SINGLEFLIGHT_MAX_LOCK_WAITERS`), plus one rolling-update surge
replica, must stay below Postgres `max_connections`. The defaults
(3 x 50 = 150) do not fit a stock server's 100. Every pool connection arms
server-side TCP keepalives (15 s idle, 5 s interval, 3 probes) with a query
right after connect, so sessions left behind by a force-killed pod are
reaped in about 30 s. `DATABASE_TCP_KEEPALIVES=false` turns this off. Behind
a pooler in transaction mode (PgBouncer, RDS Proxy) the setting may not
reach the server session that serves later queries; configure keepalives
on the pooler or in `postgresql.conf` instead. A failure to set them is
logged once at warn and never fails the connection.

A database that stays down at startup fails the process after at most five
connect attempts (each bounded by `DATABASE_ACQUIRE_TIMEOUT_SECS`) and no new
attempt after about 60 s, roughly 40 s with the defaults.

`ak_db_pool_connection_failures_total{stage}` counts pool connection
failures:

| `stage` | Counted when |
|---|---|
| `startup` | An initial connect attempt failed at boot. Startup retries up to five times with a jittered 1 to 8 s backoff, so a refusal storm shows several increments before the process exits or recovers. |
| `request` | A pool acquire timed out while serving a request and the client was answered `503 POOL_EXHAUSTED`. |

The `startup` series is exported once the replica has started serving, so it
shows how many connect attempts a replica that did come up needed; a replica
that gave up exits before it can be scraped, and its attempts are in its log.
Before 1.11.5 the `startup` series was never exported, because the attempts
were counted before the Prometheus recorder was installed (#4828).

A sustained non-zero `rate(ak_db_pool_connection_failures_total{stage="request"}[5m])`
means the pool is too small for the load, or the server is refusing new
sessions; the server's `FATAL: sorry, too many clients already` lines appear
only in the Postgres log.
