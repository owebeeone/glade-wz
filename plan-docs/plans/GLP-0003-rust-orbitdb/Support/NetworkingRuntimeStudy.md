# Networking Runtime Study

Plan: `GLP-0003`

## Scope Boundary

rust-orbitdb itself is not a TCP server. Its semantic crates MUST NOT perform
direct OS/network I/O. This document is downstream host/adapter guidance for the
point where a real process decides to expose rust-orbitdb over TCP or libp2p.
The real downstream workload is likely a libp2p swarm or host process, not a
raw socket echo server. Any C10k result is only a lower-bound runtime signal
unless it includes libp2p protocol state, stream muxing, discovery/routing,
connection management, buffers, and application backpressure.

## Question

Design a downstream server/adapter path for 10,000+ concurrent TCP connections
while balancing:

- platform uniformity across Linux, macOS, and Windows
- near-optimal readiness behavior without legacy O(N) polling
- bounded memory per connection
- good async ecosystem fit for libp2p and bindings

Primary candidate: Tokio using mio underneath.

## Recommendation

Tokio SHOULD be the default runtime candidate for a downstream real server and
adapter path. It is the best starting point because it is cross-platform,
mature, widely used, works with rust-libp2p ecosystem expectations, and uses mio
for OS-backed I/O readiness/completion.

The plan SHOULD still benchmark and document edge cases where Glommio or Smol
would be better for a narrow workload.

## mio I/O Abstraction

mio `Poll` is backed by the operating system selector. Its docs list epoll on
Linux, kqueue on macOS/BSD-style platforms, and IOCP on Windows:
`https://docs.rs/mio/latest/mio/struct.Poll.html`.

This matters for C10k because readiness is not discovered by scanning every
registered socket in user space. The server waits for the OS selector to report
ready/completed events and then services the returned event set. The practical
target is work proportional to ready events and protocol work, not O(total
connections) polling.

Important nuance: Windows IOCP is completion-based, not readiness-based. mio
adapts IOCP into its API. The docs call this bridge non-trivial and note
intermediate buffer costs for reads/writes. The C10k benchmark MUST therefore
run on Windows, not only Linux/macOS.

## Tokio Scheduler Mechanics

Tokio docs describe the multi-thread scheduler as a worker thread pool using
work stealing, with a worker thread per CPU core by default:
`https://docs.rs/tokio/latest/tokio/runtime/index.html`.

Tokio tasks are async tasks, not OS threads. Tokio's spawning tutorial describes
tasks as lightweight, requiring one allocation and 64 bytes of task metadata in
the documented baseline: `https://tokio.rs/tokio/tutorial/spawning`. This
number MUST NOT be used as a per-connection cost estimate. Real per-connection
cost includes task state, buffers, protocol objects, stream muxer state,
libp2p behavior state, queues, and application data.

For 10,000 sockets, the expected shape is:

- one accept loop
- one lightweight task per connection or per connection direction where useful
- bounded buffers per connection
- backpressure through channels/semaphores/read-write readiness
- no blocking work on runtime worker threads
- `spawn_blocking` or dedicated pools only for unavoidable blocking work

Compared with traditional OS threads, this avoids one kernel thread stack and
scheduler entity per connection. Context switching is cooperative at `.await`
points rather than preemptive thread switching for every idle socket. The memory
risk moves from thread stacks to per-task state, buffers, channel queues, and
protocol state. The benchmark MUST measure those directly.

Tokio fairness has assumptions: tasks must not block worker threads, and the
number of tasks and poll duration must stay bounded. The runtime study MUST
therefore include tests for accidental blocking, long poll loops, unfair
connections, and queue buildup.

## C10k Benchmark Shape

The downstream host/server benchmark SHOULD measure:

- accepted connections
- active connections
- idle connections
- memory per connection
- read/write latency percentiles
- queue depth
- backpressure events
- cancellation time
- reconnect storms
- slow-reader behavior
- Windows IOCP behavior separately from Linux/macOS readiness behavior

The benchmark MUST include at least:

- idle 10k sockets
- 10k sockets with sparse writes
- mixed slow/fast readers
- churn: connect/disconnect/reconnect
- partition-like pauses at the application layer
- overload and admission-control behavior
- libp2p-host profile with stream muxing, protocol state, discovery/routing
  state where applicable, and rust-orbitdb adapter backpressure

## Glommio Trade-Off

Glommio is a Linux `io_uring` thread-per-core runtime:
`https://docs.rs/glommio/latest/glommio/`.

It can be architecturally superior when:

- Linux-only is acceptable
- thread-per-core ownership is natural
- workload partitioning avoids cross-core coordination
- ultra-low context switching and cache locality matter more than platform
  uniformity

It is not the default here because the requirement explicitly includes Linux,
macOS, and Windows without platform-specific forks. Glommio's own docs state it
depends on Linux `io_uring`, with kernel version constraints. It is a useful
comparison point for Linux-only high-performance experiments, not the default
cross-platform architecture.

## Smol Trade-Off

Smol is a small async runtime and re-export of smaller async crates:
`https://github.com/smol-rs/smol`.

It can be superior when:

- minimal runtime footprint is more important than ecosystem compatibility
- the service avoids Tokio-dependent libraries
- compile-time and dependency size dominate
- the architecture is simpler than a libp2p-heavy server

It is not the default here because rust-libp2p, common networking libraries, and
server tooling are more naturally aligned with Tokio. Smol remains a useful
comparison point for minimal components or test utilities.

## Acceptance Criteria

The runtime choice is accepted when:

- downstream Tokio/mio server skeleton runs on Linux, macOS, and Windows
- no public core/sync/document API depends on Tokio
- C10k benchmark records memory, latency, throughput, and cancellation behavior
- Windows IOCP behavior is explicitly tested or documented as blocked
- Glommio and Smol are evaluated with a concrete reason for accepting or
  rejecting them for this workload
- no benchmark relies on legacy O(N) scanning of connection readiness

## References

- Tokio runtime docs: `https://docs.rs/tokio/latest/tokio/runtime/index.html`
- Tokio spawning tutorial: `https://tokio.rs/tokio/tutorial/spawning`
- mio Poll docs: `https://docs.rs/mio/latest/mio/struct.Poll.html`
- Glommio docs: `https://docs.rs/glommio/latest/glommio/`
- Smol README: `https://github.com/smol-rs/smol`
