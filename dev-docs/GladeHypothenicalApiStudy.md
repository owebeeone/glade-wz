# Glade Hypothenical API Study

Status: study draft

Purpose: show one coherent hypothetical Glade API surface for the current
working scenarios so the boundary between `Glade`, `Glial`, and `Grip Share`
can be tested concretely.

This document is non-normative. It is intended to make the API boundary easier
to reason about, not to freeze names or exact signatures.

The current higher-level Glial environment model that should map onto this API
is described in
`/Users/owebeeone/limbo/glade-wz/dev-docs/glial/GlialEnvironmentModel.md`.

## Core Position

Glade SHOULD expose one small family of content and exchange primitives rather
than one generic replicated blob API.

For the scenarios below, the API surface needs to make these choices explicit:

- what `Share` a thing belongs to
- what `ContentShare` or exchange is being declared
- what `sync_model` is being used
- who is allowed to claim authority
- whether readers are following live state, replaying persisted state, or
  opening a sparse window into larger content

Glade SHOULD own:

- share declaration
- `ContentShare` declaration
- exchange bundle declaration
- authority claim and ownership term
- content heads, chunks, and snapshot references
- invalidation and replay cursors
- diagnostics and observation state

Glade SHOULD NOT own:

- database query semantics
- shell or PTY execution semantics
- terminal emulation semantics
- file parser or syntax tree semantics
- UI binding semantics
- transport selection details

## Common API Shape

The scenarios below assume one shared control-plane model with a few explicit
sync models:

- `exchange`
- `append-log`
- `authoritative-patch`
- `live-stream`
- `crdt` when truly needed

The working API idea is:

### TypeScript

```ts
type SyncModel =
  | 'exchange'
  | 'append-log'
  | 'authoritative-patch'
  | 'live-stream'
  | 'crdt';

type AuthorityMode =
  | 'provider-owned'
  | 'leased-writer'
  | 'multi-writer';

const glade = await Glade.connect({
  runtime: 'js',
  peerId: 'ui:session-42',
});

const share = await glade.openShare({
  shareId: 'project:alpha',
  scope: 'team:alpha',
});

const content = await share.content({
  contentId: 'file:report.md',
  syncModel: 'authoritative-patch',
  authorityMode: 'leased-writer',
});
```

### Python

```python
from glade import Glade

glade = await Glade.connect(
    runtime="py",
    peer_id="service:bugs-db",
)

share = await glade.open_share(
    share_id="project:alpha",
    scope="team:alpha",
)

content = await share.content(
    content_id="file:report.md",
    sync_model="authoritative-patch",
    authority_mode="leased-writer",
)
```

The remainder of the API is scenario-specific convenience over the same
underlying Glade concepts:

- exchange bundle records
- content heads
- chunk references
- replay cursors
- authority claims
- diagnostics

## Glial Mapping Over Glade

Glial SHOULD present environments, workspaces, facets, sessions, and
capabilities. Glade SHOULD remain the lower share/content substrate.

That means Glial can expose a higher API such as:

### TypeScript

```ts
const glial = await Glial.connect({
  sessionId: 'session:alice-browser',
  principalId: 'user:alice',
});

const env = await glial.openEnvironment({
  environmentId: 'env:alice-dev',
});

await env.mountWorkspace({
  workspaceId: 'workspace:repo-alpha',
  capability: repoCapability,
});

await env.mountWorkspace({
  workspaceId: 'workspace:bugs-alpha',
  capability: bugsCapability,
});

const editor = await env.mountFacet({
  facetId: 'facet:editor',
  kind: 'editor',
});

const ai = await env.attachAgentSession({
  sessionId: 'session:assistant-refactor',
  delegatedCapabilities: [
    env.delegateShare('workspace:repo-alpha', 'share:file:src/app.ts', ['read', 'write']),
    env.delegateShare('workspace:repo-alpha', 'share:selection:editor-main', ['read']),
    env.delegateShare('workspace:repo-alpha', 'share:terminal:dev-server', ['read']),
  ],
});

const file = await editor.openShare({
  workspaceId: 'workspace:repo-alpha',
  shareId: 'share:file:src/app.ts',
}).asMutableContent({
  syncModel: 'authoritative-patch',
});
```

### Underlying Glade Shape

```ts
const glade = await Glade.connect({
  runtime: 'js',
  peerId: 'session:alice-browser',
});

const fileShare = await glade.openShare({
  shareId: 'share:file:src/app.ts',
  scope: 'workspace:repo-alpha',
});

const file = await fileShare.mutableContent({
  contentId: 'file:src/app.ts',
  syncModel: 'authoritative-patch',
  authorityMode: 'leased-writer',
});
```

The point is not that Glial hides Glade completely. The point is that Glial
maps:

- environment membership
- workspace mounting
- facet composition
- delegated capability issuance

onto Glade's lower-level share and content mechanisms.

## Scenario 1: Database Query Request / Response

This is bounded work. It fits the `exchange` model.

### What Glade Owns

- declaration of the exchange
- request identity and attempt identity
- provider claim and ownership term
- progressive publication of response events
- requester observation state
- diagnostics if publication or observation is uncertain

### What Glade Does Not Own

- query language
- query execution
- database connection management
- result shaping beyond payload transport

### TypeScript Requester

```ts
type BugsQueryRequest = {
  projectId: string;
  status: 'open' | 'closed';
  limit: number;
};

type BugsQueryResponse = {
  rows: Array<{ id: string; title: string; status: string }>;
  total: number;
};

const bugsQuery = await share.exchange<BugsQueryRequest, BugsQueryResponse>({
  exchangeId: 'BugsQuery',
  authorityMode: 'provider-owned',
  requestCodec: 'json',
  responseCodec: 'json',
  replyRetention: { ttlMs: 60_000 },
});

const run = await bugsQuery.request(
  { projectId: 'alpha', status: 'open', limit: 200 },
  { ttlMs: 15_000 },
);

for await (const progress of run.progress()) {
  console.log(progress.kind, progress.detail);
}

for await (const diag of run.diagnostics()) {
  console.warn(diag.kind, diag.detail);
}

const result = await run.final();
console.log(result.rows.length, result.total);
```

### Python Provider

```python
bugs_query = await share.exchange(
    exchange_id="BugsQuery",
    authority_mode="provider-owned",
    request_codec="json",
    response_codec="json",
)

@bugs_query.serve()
async def handle_bugs_query(job):
    await job.claim()
    await job.progress("running", {"stage": "db-query"})
    rows = await db.fetch_bugs(
        project_id=job.request["projectId"],
        status=job.request["status"],
        limit=job.request["limit"],
    )
    await job.publish_final({
        "rows": rows,
        "total": len(rows),
    })
```

### Boundary Notes

- `request()` is a convenience API over Glade exchange bundle records.
- `serve()` is a convenience API over claim, progress, publication, and
  diagnostics records.
- If the requester needs confirmation that it truly observed the final result,
  that remains a distinct Glade concern from provider publication.

## Scenario 2: CLI Command Output As A Persistent Growing Log

This is not bounded work in the same sense as the query. It fits the
`append-log` model, optionally with a paired control record for command exit
status and metadata.

### What Glade Owns

- declaration of the command output log
- append ordering
- durable chunk references
- replay cursor management
- tailing live output and resuming from a cursor
- metadata head for exit code and completion state

### What Glade Does Not Own

- process launch semantics
- shell environment
- PTY semantics
- interpretation of stdout vs stderr beyond tagged payload metadata

### Python Provider

```python
run_log = await share.log(
    content_id="cli:run-8842:output",
    authority_mode="provider-owned",
    chunk_codec="utf8",
    retention={"persist": True},
)

await run_log.open_head({
    "argv": ["pytest", "-q"],
    "started_at": now_ms(),
})

process = await spawn_command(["pytest", "-q"])

async for chunk in process.stdout_chunks():
    await run_log.append(chunk, stream="stdout")

async for chunk in process.stderr_chunks():
    await run_log.append(chunk, stream="stderr")

exit_code = await process.wait()
await run_log.close_head({
    "exit_code": exit_code,
    "completed_at": now_ms(),
})
```

### TypeScript Follower

```ts
const runLog = await share.openLog({
  contentId: 'cli:run-8842:output',
  localCache: true,
});

const head = await runLog.head();
console.log(head.argv, head.started_at);

for await (const chunk of runLog.tail({ from: 'live-or-last-local-cursor' })) {
  terminal.write(chunk.data);
}

const finalHead = await runLog.waitForClose();
console.log(finalHead.exit_code);
```

### Boundary Notes

- This model can support many command runs without introducing a second file
  sync subsystem.
- The log head is mutable control state. The chunks are durable content.
- Glade does not need CRDT semantics here because append order is owned by one
  authority.

## Scenario 3: Shared Mutable File With Sparse Windows

This is a mutable object with replay and sparse fetch. It fits the
`authoritative-patch` model.

### What Glade Owns

- declaration of the file content share
- canonical file head and version
- patch submission and accepted patch publication
- line-range or byte-range chunk references
- sparse window fetch
- invalidation events for cached windows
- local cache persistence rules

### What Glade Does Not Own

- file format semantics
- syntax tree indexing
- editor layout decisions
- merge policy beyond the declared sync model

### TypeScript Viewer / Editor

```ts
const reportFile = await share.mutableContent({
  contentId: 'file:reports/current.md',
  syncModel: 'authoritative-patch',
  authorityMode: 'leased-writer',
  chunking: { kind: 'line-range', linesPerChunk: 200 },
  staleReads: 'allowed',
  localCache: true,
});

const window3000 = await reportFile.openWindow({
  lineStart: 3000,
  lineCount: 250,
  preferCached: true,
  allowStale: true,
});

render(window3000.text);

window3000.onInvalidated(async (info) => {
  if (!info.overlapsCurrentWindow) return;
  const refreshed = await window3000.refresh({ background: true });
  render(refreshed.text);
});

await reportFile.submitPatch({
  baseVersion: window3000.version,
  hunks: [
    {
      startLine: 3012,
      deleteCount: 1,
      insertLines: ['Rewritten line from editor'],
    },
  ],
});
```

### Python Authority

```python
report_file = await share.mutable_content(
    content_id="file:reports/current.md",
    sync_model="authoritative-patch",
    authority_mode="leased-writer",
    chunking={"kind": "line-range", "lines_per_chunk": 200},
)

@report_file.serve_patches()
async def apply_patch(job):
    await job.claim()
    current = await filesystem.read_text("reports/current.md")
    updated = apply_hunks(current, job.patch["hunks"], job.patch["baseVersion"])
    await filesystem.write_text("reports/current.md", updated)
    await job.publish_version(
        version=job.next_version(),
        invalidations=job.invalidation_ranges(),
    )
```

### Boundary Notes

- The viewer does not need the whole file.
- A stale cached window MAY remain visible briefly; Glade only needs to make
  invalidation and refetch deterministic.
- If later the same content type truly needs concurrent multi-writer merge,
  this scenario could move to a `crdt` sync model without changing the higher
  `Share` identity model.

## Scenario 4: Interactive Terminal Stream

This is a live channel with optional persisted replay. It fits the
`live-stream` model plus a paired append log for history and reattach.

### What Glade Owns

- declaration of the live channel
- writer or controller authority claim
- output stream identity and replay cursor
- optional persisted output log
- presence and diagnostics

### What Glade Does Not Own

- PTY behavior
- terminal emulation
- shell interpretation
- frame rendering policy

### Python PTY Host

```python
terminal = await share.live_channel(
    channel_id="pty:session-8842",
    authority_mode="provider-owned",
    input_codec="utf8",
    output_codec="bytes",
    persisted_log_content_id="pty:session-8842:output",
)

@terminal.host()
async def host_terminal(session):
    pty = await open_pty(cols=120, rows=30)

    async def pump_input():
        async for frame in session.input_frames():
            if frame.kind == "stdin":
                await pty.write(frame.data)
            elif frame.kind == "resize":
                await pty.resize(frame.cols, frame.rows)

    async def pump_output():
        async for chunk in pty.output_chunks():
            await session.publish_output(chunk)

    await run_concurrently(pump_input(), pump_output())
```

### TypeScript Viewer / Controller

```ts
const terminal = await share.liveChannel({
  channelId: 'pty:session-8842',
  authorityMode: 'provider-owned',
  persistedLogContentId: 'pty:session-8842:output',
});

const attached = await terminal.attach({
  role: 'writer',
  from: 'live',
});

for await (const chunk of attached.output()) {
  xterm.write(chunk.text);
}

xterm.onData((data) => {
  void attached.sendInput({ kind: 'stdin', data });
});

xterm.onResize(({ cols, rows }) => {
  void attached.sendInput({ kind: 'resize', cols, rows });
});
```

### Boundary Notes

- This is not a replicated document and should not be forced into that shape.
- The hot path is a live input/output channel.
- Replay and late attach can be provided by the paired persisted log without
  polluting the live path.
- If multiple writers are allowed, Glade only needs to transport tagged input
  frames and preserve the host's accepted ordering.

## Scenario 5: Collaborative File Editing With Concurrent Writers

This is the first scenario in this study where a `crdt` sync model becomes a
serious candidate.

This scenario is different from Scenario 3. Scenario 3 assumes one accepted
writer path at a time and sparse patch invalidation around a canonical file
head. This scenario assumes multiple participants MAY edit the same logical
document concurrently and the system SHOULD merge those edits without requiring
an edit lease for each keystroke.

This section is a boundary study only. It does not imply that Glade SHOULD
implement CRDT first.

### What Glade Owns

- declaration of the collaborative content share
- declared `crdt` sync model
- replica identity and operation envelopes
- snapshot and compaction boundaries
- local operation staging and later convergence
- observation, replay, and diagnostics state

### What Glade Does Not Own

- editor semantics
- syntax-aware transforms
- filesystem projection policy beyond the declared adapter contract
- UI conflict presentation

### TypeScript Editor

```ts
const sharedDoc = await share.mutableContent({
  contentId: 'doc:reports/live-edit',
  syncModel: 'crdt',
  authorityMode: 'multi-writer',
  localCache: true,
  compaction: { snapshotEveryOps: 5_000 },
});

const replica = await sharedDoc.attachReplica({
  replicaId: 'ui:session-42',
});

const view = await replica.openTextView({
  path: '/',
});

editor.onLocalInsert(({ at, text }) => {
  void view.insert(at, text);
});

editor.onLocalDelete(({ at, count }) => {
  void view.delete(at, count);
});

for await (const update of view.remoteUpdates()) {
  editor.applyRemoteUpdate(update);
}
```

### Python Peer Or Service

```python
shared_doc = await share.mutable_content(
    content_id="doc:reports/live-edit",
    sync_model="crdt",
    authority_mode="multi-writer",
)

replica = await shared_doc.attach_replica(
    replica_id="service:ai-editor",
)

view = await replica.open_text_view(path="/")

await view.insert(120, "Suggested rewrite")
await view.delete(240, 18)

async for diagnostic in replica.diagnostics():
    logger.warning("crdt diagnostic: %s %s", diagnostic.kind, diagnostic.detail)
```

### Boundary Notes

- This is the case where Glade would pay the CRDT tax because concurrent
  writers are part of the contract, not an accident.
- The stable API decision is not "use CRDT everywhere". The stable API
  decision is that `mutableContent()` can declare different sync models
  explicitly.
- `Grip Lab` file editing SHOULD start with `authoritative-patch` unless
  concurrent multi-writer editing is a real requirement.
- If `Grip Lab` later offers a true collaborative editing mode, this scenario
  is the model that would justify it.

## Boundary Summary

The main API lesson from these scenarios is that Glade SHOULD expose a small
set of explicit models instead of trying to hide every case behind one generic
mutation API.

The useful working set appears to be:

- `exchange()` for bounded request/reply work
- `log()` for durable growing output
- `mutableContent()` for authoritative patchable content with sparse windows
- `liveChannel()` for low-latency interactive streams
- `mutableContent()` with `syncModel: 'crdt'` for true concurrent collaborative
  editing

All of those sit on the same lower Glade substrate:

- `Share`
- `ContentShare`
- authority claim and ownership term
- head records
- chunk and snapshot references
- diagnostics
- observation and replay cursors

`Glial` can map product and session concerns onto these primitives.
`Grip Share` can map local Grip state onto them.
Neither layer should need to redefine what the Glade content models mean.
