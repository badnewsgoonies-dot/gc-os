# BLUEPRINT v4: The General Contractor
## AI-Powered Project Building Platform — Desktop-First, Event-Sourced

**Codename:** GC (General Contractor)
**Architecture:** Event-sourced + CQRS. One write model, many read projections.

**Architectural laws:**
1. All durable product changes append events.
2. All UI reads projections.
3. All builds run against immutable snapshots.
4. All restores fork hidden branches.

**Two orderings in the event log:**
- `seq` (per branch) answers: "what happened in this branch?"
- `globalPosition` (BIGSERIAL across all branches) answers: "what should projectors consume next?"

**What is NOT an event (ephemeral only):**
Token deltas, typing indicators, scroll position, pane width, hover state, draft input. These live in Zustand only, never touch the event log. The log contains semantic facts, not UI twitching.

**Stack:**
- Frontend: Next.js 15 + React 19 + TypeScript + Tailwind v4 + shadcn/ui
- Canvas: tldraw SDK (DOM-based, iframe-embeddable, owns spatial state)
- AI: Vercel AI SDK v6 + Anthropic API + model routing
- Persistence: PostgreSQL (events + projections) + object storage (artifact blobs)
- Real-time: SSE for AI streaming + projection updates
- Execution: Sandboxed iframes (HTML), WebContainers (JS, right panel only), cloud WASM (Rust)

---

## CORE DATA MODEL

See `src/types/index.ts` for the full TypeScript definitions. Summary:

**Write model (append-only, the truth):**
- `Event` — every action in the system. Has `seq` (monotonic per branch), `kind`, `actorType`, `causationId`, `correlationId`, `payload`.
- `ArtifactVersion` — immutable snapshots of artifact content. Has `contentHash`, `storageRef`, `evidence`, `gatesPassed`, `derivedFromEventSeq`.
- `BuildRun` — a build attempt pinned to `inputSnapshotId`. Has `status`, `promotedAt`.
- `GateEvidence` — individual verification results per build. Has `gate`, `passed`, `evidenceRef`.
- `Snapshot` — a bookmark at a specific `atSeq` on a branch. For safe reset.

**Blob store (not in Postgres):**
- Artifact file contents (large)
- Poster/thumbnail images
- Gate evidence logs, screenshots, JUnit XML

**Canvas authority (tldraw owns this):**
- tldraw doc/store for shapes, positions, pages
- `CanvasBinding` maps tldraw shape IDs to artifact version IDs
- Never store `canvasPosition` on artifacts

**Read projections (materialized by projector, queried by UI):**
- `ChatFeedItem` — the chat stream, derived from events
- `CurrentPreviewProjection` — what the right panel shows
- `WorkspaceOverviewProjection` — project cards on landing page

**The UI never reads raw events. The UI reads projections.**

---

## LAYOUT

```
┌──────────────────────────────────────────────────────────────┐
│  [GC]    Workspace: My Website    [Canvas] [Preview] [Split] │  <- TopBar 48px
├────────────────────┬─────────────────────────────────────────┤
│                    │                                         │
│   CHAT PANEL       │   RIGHT PANEL                           │
│   (400px fixed)    │   (flex-1)                              │
│                    │                                         │
│   Reads from       │   Preview mode: live artifact render    │
│   ChatFeedItem     │   Canvas mode: tldraw + artifact shapes │
│   projection       │   Split mode: both stacked              │
│                    │                                         │
│   [Drop zone]      │   Compiled toggle →                     │
│   [Text input]     │     latestPromotedBuildRunId            │
├────────────────────┴─────────────────────────────────────────┤
│  Status: Building... (3/7 gates passed)                      │  <- StatusBar 32px
└──────────────────────────────────────────────────────────────┘
```

Preview grades:
1. **Poster** — always safe for chat feed (static thumbnail in ArtifactCard)
2. **Focused live** — right panel only (iframe / WebContainer / WASM canvas)
3. **Verified compiled** — `latestPromotedBuildRunId`, never ad-hoc preview state

---

## FILE STRUCTURE

```
gc-project/
├── package.json
├── next.config.ts
├── tailwind.config.ts
├── tsconfig.json
├── BLUEPRINT.md
├── prisma/
│   └── schema.prisma              # Events, projections, artifacts, builds
├── src/
│   ├── app/
│   │   ├── layout.tsx
│   │   ├── globals.css
│   │   ├── page.tsx               # Main workspace (MVP: single workspace)
│   │   └── api/
│   │       ├── events/
│   │       │   └── route.ts       # POST: append event. GET: stream projections via SSE
│   │       ├── chat/
│   │       │   └── route.ts       # AI chat endpoint (wraps event appender + AI SDK)
│   │       ├── artifacts/
│   │       │   └── [id]/
│   │       │       └── route.ts   # GET: serve artifact version files from blob store
│   │       └── builds/
│   │           └── route.ts       # POST: queue build. GET: build status
│   │
│   ├── components/
│   │   ├── layout/
│   │   │   ├── TopBar.tsx
│   │   │   ├── StatusBar.tsx
│   │   │   └── ProjectShell.tsx   # Three-panel with react-resizable-panels
│   │   ├── chat/
│   │   │   ├── ChatPanel.tsx      # Reads from ChatFeedItem projection
│   │   │   ├── ChatInput.tsx      # Text + image drop
│   │   │   ├── MessageRenderer.tsx # Dispatches on ChatFeedItem.kind
│   │   │   ├── TextBubble.tsx
│   │   │   ├── ArtifactCard.tsx   # Poster grade — click opens right panel
│   │   │   ├── DecisionCard.tsx
│   │   │   ├── StatusPill.tsx
│   │   │   ├── DiffView.tsx
│   │   │   ├── ImageBubble.tsx
│   │   │   └── PlanCard.tsx       # Contract/plan approval UI
│   │   ├── canvas/
│   │   │   ├── CanvasPanel.tsx    # tldraw wrapper
│   │   │   └── ArtifactShape.tsx  # Custom tldraw shape → CanvasBinding
│   │   └── preview/
│   │       └── PreviewPanel.tsx   # Reads from CurrentPreviewProjection
│   │
│   ├── lib/
│   │   ├── kernel/
│   │   │   ├── event-store.ts     # Server: append event, increment seq, update branch head
│   │   │   ├── projector.ts       # Server: fold events into projection tables
│   │   │   └── blob-store.ts      # Server: object storage abstraction (local FS for MVP)
│   │   ├── ai/
│   │   │   ├── chat-handler.ts    # AI SDK streamText + tool calls → events
│   │   │   ├── tools.ts           # Tool definitions (generate_artifact, propose_plan, etc.)
│   │   │   ├── model-router.ts    # Haiku for brainstorm, Sonnet for code, Opus for architecture
│   │   │   └── prompts.ts         # GC system prompt
│   │   ├── build/
│   │   │   ├── dispatcher.ts      # Queue BuildRun against snapshot
│   │   │   ├── verifier.ts        # Run gates, record GateEvidence
│   │   │   └── promoter.ts        # Promote verified build → update workspace pointer
│   │   ├── execution/
│   │   │   ├── iframe-sandbox.ts  # Sandboxed iframe for HTML artifacts
│   │   │   └── webcontainer.ts    # WebContainer for JS (right panel only)
│   │   ├── db/
│   │   │   ├── prisma.ts          # Prisma client singleton
│   │   │   └── queries.ts         # Typed query functions for projections
│   │   └── store/
│   │       └── index.ts           # Client stores: projection readers, UI state
│   │
│   └── types/
│       └── index.ts               # Full v4 type definitions (298 lines)
```

---

## BUILD WAVES

### WAVE 0A: Kernel (1 worker, 4-5 hours)
**Scope:** `prisma/schema.prisma`, `src/lib/kernel/*`, `src/app/api/events/*`
**This is the foundation. Nothing else works without it.**

**Prisma schema:** See `prisma/schema.prisma` (already in repo, 243 lines). Includes all write model tables, projector infrastructure (Outbox, ProjectionOffset), and all read projection tables.

**Definition of done — six things must work:**

**1. Transactional append.**
Appending an event must atomically: allocate branch seq, write the event, insert outbox entry, update Branch.nextSeq. One transaction, all or nothing.

```sql
BEGIN;
SELECT next_seq FROM branches WHERE id = $branch_id FOR UPDATE;
INSERT INTO events (global_position, workspace_id, branch_id, seq, kind,
  actor_type, causation_id, correlation_id, idempotency_key, payload)
VALUES (DEFAULT, $workspace_id, $branch_id, $next_seq, $kind,
  $actor_type, $causation_id, $correlation_id, $idempotency_key, $payload::jsonb);
UPDATE branches SET next_seq = next_seq + 1 WHERE id = $branch_id;
INSERT INTO outbox (event_id, global_position) VALUES ($event_id, $global_position);
COMMIT;
```

**2. Idempotency.**
The append API accepts an `idempotencyKey`. Same key → same result, no duplicate event. Unique constraint on `Event.idempotencyKey` enforces this.

**3. Projector with checkpoints.**
Projector reads events where `globalPosition > offset`, folds each into projection tables, then atomically advances `ProjectionOffset.lastGlobalPosition`. If the projector crashes mid-batch, it replays from the last checkpoint. Outbox table is the wake-up source (NOTIFY is a hint, not the truth).

Projector fold logic (switch on event.kind):
- `user.message.posted` → insert ChatFeedItem { role: 'user', kind: 'text' }
- `assistant.message.posted` → insert ChatFeedItem { role: 'assistant', kind: 'text' }
- `assistant.decision.requested` → insert ChatFeedItem { role: 'assistant', kind: 'decision' }
- `artifact.created` → insert ChatFeedItem { kind: 'artifact' }, upsert ArtifactIndex
- `artifact.version.created` → insert ChatFeedItem { kind: 'artifact' }, update ArtifactIndex
- `build.gate.recorded` → insert ChatFeedItem { kind: 'gate' }
- `build.run.promoted` → update CurrentPreview, update WorkspaceOverview
- `plan.proposed` → insert ChatFeedItem { kind: 'plan' }, upsert ActivePlan
- `plan.frozen` → update ActivePlan, update WorkspaceOverview.status = 'building'
- `workspace.active_branch_changed` → update WorkspaceOverview.activeBranchId

**4. Rebuildability.**
A command that truncates all projection tables, resets all ProjectionOffsets to 0, and replays the entire event log through the projector. If this doesn't produce identical projections, the event model is broken. Test this explicitly.

**5. Durable fan-out.**
Outbox table, not NOTIFY, is the source of truth for downstream consumers. NOTIFY is fine as a wake-up signal, but consumers must poll outbox rows on startup in case they missed signals. SSE endpoint reads from outbox / events by globalPosition.

**6. Immutable blob refs.**
Blob store (`src/lib/kernel/blob-store.ts`): content-addressed local filesystem at `./data/blobs/{sha256hash}`. `put(content)` → returns sha256 hash as storageRef. `get(storageRef)` → returns content. Even if crude (local FS), the key is hash-based from day one. ArtifactVersion.storageRef and posterRef both use this.

**Deliverables:**
- `prisma/schema.prisma` — already in repo (copy to project)
- `src/lib/kernel/event-store.ts` — transactional append with idempotency
- `src/lib/kernel/projector.ts` — fold events into projection tables with checkpoints
- `src/lib/kernel/blob-store.ts` — content-addressed local FS
- `src/lib/kernel/rebuild.ts` — truncate projections + replay from events
- `src/app/api/events/route.ts` — POST: append event. GET: SSE stream by globalPosition
- `src/lib/db/prisma.ts` — Prisma client singleton
- `prisma/seed.ts` — creates default workspace + main branch + initial projections

**Test (all must pass):**
- Append 10 events of different kinds → projections have correct entries
- Append with same idempotencyKey twice → only one event created, no error
- globalPosition is strictly monotonic across all events (even across branches)
- seq is monotonic within branch, unique constraint enforced
- Rebuild projections from scratch → identical to incremental
- Blob store put/get round-trip: content matches, hash is deterministic
- Projector crash simulation: kill mid-batch, restart, verify no missed/duplicate projections

---

### WAVE 0B: Canvas Boundary (1 worker, 1-2 hours)
**Scope:** `src/components/canvas/*`
**Contract:** tldraw owns spatial state. CanvasBinding connects artifact versions to shapes.

**Deliverables:**
- `CanvasPanel.tsx` — tldraw editor instance, dark grid, zoom/pan
- `ArtifactShape.tsx` — custom tldraw shape type that:
  - Renders artifact poster (thumbnail) inside the shape
  - Shows title + evidence badge
  - Double-click → setActiveArtifact + switch to preview mode
  - References `artifactVersionId` via CanvasBinding, NOT by embedding files
- Canvas state stored in tldraw's own store (not in Postgres events)
- CanvasBinding CRUD: when AI places artifact on canvas, create binding row

**Test:** Mount tldraw. Create an artifact shape referencing a mock artifact version. Move it around. Double-click to trigger preview. Verify no `canvasPosition` exists on any artifact row.

---

### WAVE 1: Shell + Projections (1 worker, 2-3 hours)
**Scope:** `src/components/layout/*`, `src/components/chat/*` (already mostly built)
**Contract:** All UI reads from projection tables via API calls, not from raw events.

**Deliverables:**
- Wire ChatPanel to `GET /api/chat-feed?workspaceId=X` (reads ChatFeedItem projection)
- Wire StatusBar to `GET /api/build-status?workspaceId=X` (reads latest BuildRun)
- Wire TopBar workspace name from WorkspaceOverview projection
- SSE connection on page load: `GET /api/events?workspaceId=X&after=LAST_SEQ` → append new ChatFeedItems to store in real-time
- Add PlanCard component (renders plan.proposed events as approval UI)

**Test:** Seed 20 events via API. Page loads, chat shows all projected items. New events via SSE appear live.

---

### WAVE 2: Chat Streaming (1 worker, 3-4 hours)
**Scope:** `src/app/api/chat/*`, `src/lib/ai/*`
**Contract:** User message → event → AI response → event(s). Everything flows through the event log.

**Deliverables:**

**1. Chat API** (`src/app/api/chat/route.ts`):
```typescript
// POST /api/chat
// Body: { workspaceId, content }
//
// Flow:
// 1. Append event: user.message.posted
// 2. Load recent ChatFeedItems as conversation context
// 3. Call AI SDK streamText with tools
// 4. As AI streams: buffer response
// 5. On complete: append event: assistant.message.posted
// 6. On tool call: append appropriate event (artifact.created, plan.proposed, etc.)
// 7. Return streamed response to client
```

**2. System prompt** (`src/lib/ai/prompts.ts`):
```
You are a General Contractor (GC) for software projects. Users come to you
with ideas — websites, apps, games, tools — and you help them build it.

YOUR ROLE:
- Partner, not servant. You have opinions and share them.
- Ask clarifying questions before building. 2 max per message.
- Push back on bad ideas gently. Suggest alternatives.
- Explain trade-offs in plain language.

YOUR WORKFLOW:
1. DISCOVER — understand what they need (3-5 exchanges minimum)
2. SPECIFY — propose a plan (structured deliverables list)
3. BUILD — dispatch work, show progress, deliver previews
4. ITERATE — user sees output, requests changes, you refine

YOUR TONE:
- Friendly but direct. Like a contractor you'd refer to friends.
- Brief. Don't monologue. One question at a time.
- Use the user's language. If they say "website" don't say "SPA."
- Say "we" not "I" — you and the user are a team.

TOOLS:
- generate_artifact: create a live preview (HTML/React/code)
- update_artifact: modify an existing artifact version
- propose_plan: present structured requirements for approval
- show_options: give user 2-4 choices when a decision is needed

NEVER:
- Generate code in chat text — always use generate_artifact
- Start building without conversation first (3+ exchanges minimum)
- Use bullet points in conversational messages
```

**3. Tool definitions** (`src/lib/ai/tools.ts`):
```typescript
const tools = {
  generate_artifact: {
    description: 'Create a live preview artifact',
    parameters: z.object({
      logicalName: z.string(),
      kind: z.enum(['html', 'react', 'code', 'diagram']),
      title: z.string(),
      description: z.string(),
      files: z.record(z.string()),  // path -> content
      entryPoint: z.string(),
    }),
    // execute: creates Artifact + ArtifactVersion, appends artifact.created event
  },

  update_artifact: {
    description: 'Update an existing artifact with changes',
    parameters: z.object({
      artifactId: z.string(),
      changeDescription: z.string(),
      files: z.record(z.string()),
      entryPoint: z.string(),
    }),
    // execute: creates new ArtifactVersion, appends artifact.version.created event
  },

  propose_plan: {
    description: 'Present a structured build plan for user approval',
    parameters: z.object({
      title: z.string(),
      sections: z.array(z.object({
        title: z.string(),
        requirements: z.array(z.string()),
        deliverables: z.array(z.string()),
        acceptanceCriteria: z.array(z.string()),
      })),
    }),
    // execute: creates Plan, appends plan.proposed event
  },

  show_options: {
    description: 'Present 2-4 choices for the user to decide',
    parameters: z.object({
      prompt: z.string(),
      options: z.array(z.object({
        label: z.string(),
        description: z.string(),
        value: z.string(),
      })).min(2).max(4),
    }),
    // execute: appends assistant.decision.requested event
  },
}
```

**4. Model router** (`src/lib/ai/model-router.ts`):
- Default: Sonnet for everything (MVP)
- Future: Haiku for brainstorm/clarification, Sonnet for code gen, Opus for architecture

**Wiring:**
- ChatPanel `handleSend` → POST /api/chat → events appended → projector → SSE → ChatFeedItem → UI updates
- Tool calls during streaming → events appended → ArtifactCard / PlanCard / DecisionCard appear in feed

**Test:** Type "I want to build a landing page for a bakery." AI responds conversationally. After 3 exchanges, AI calls `show_options` with style choices. User picks one. AI calls `generate_artifact` with HTML. ArtifactCard appears in chat. Click it → preview shows live page in right panel.

---

### WAVE 3: Artifact Versions + Preview (1 worker, 2-3 hours)
**Scope:** `src/components/preview/*`, `src/lib/kernel/blob-store.ts`, `src/app/api/artifacts/*`
**Contract:** Artifacts are immutable versions stored in blob store. Preview reads from latest version.

**Deliverables:**
- `blob-store.ts` — content-addressed local filesystem store
- `POST /api/artifacts` — create artifact + version, store files in blob store
- `GET /api/artifacts/[id]` — return latest version's files
- `GET /api/artifacts/[id]/versions` — return version history
- Poster generation: on artifact creation, generate a 400x300 thumbnail (for HTML: headless screenshot; for code: syntax-highlighted snippet image; for MVP: just use first 20 lines of code as text poster)
- PreviewPanel reads from `useArtifactIndexStore.getLatestVersion(activeId)`
- For HTML artifacts: render in sandboxed iframe via `srcDoc`
- For React artifacts (future): render via Sandpack
- Version indicator in preview header: "v3" with dropdown to see history

**Test:** Create artifact via API with 3 versions. PreviewPanel shows latest. Version dropdown shows all 3. Each version has unique contentHash. Blob store returns correct content for each storageRef.

---

### WAVE 4: Plan Freeze (1 worker, 2-3 hours)
**Scope:** `src/components/chat/PlanCard.tsx`, plan-related API endpoints
**Contract:** GC proposes plan via tool call. User approves or requests changes. Approval freezes the plan.

**Deliverables:**
- `PlanCard.tsx` — renders plan sections as structured card in chat:
  - Section titles with expandable requirements/deliverables/criteria
  - "Approve & Build" button → emits `plan.frozen` event with checksum
  - "Request Changes" button → sends user message back to AI
  - Approved state: green border, locked, shows checksum
- `POST /api/plans/freeze` — compute checksum (sha256 of canonical JSON), update Plan.status, emit event
- Only one active frozen plan per workspace (`workspace.activePlanId`)
- Freezing a new plan supersedes the old one (old plan status → 'superseded')

**Test:** AI proposes plan with 3 sections. PlanCard renders in chat. Click "Approve & Build." Plan status changes to frozen. Checksum is computed. WorkspaceOverview.status changes to 'building'. Old plan (if any) becomes superseded.

---

### WAVE 5: Build Runs + Gates (1 worker, 3-4 hours)
**Scope:** `src/lib/build/*`, `src/app/api/builds/*`
**Contract:** Frozen plan triggers build. Build runs against a snapshot. Gates produce evidence. Verified builds get promoted.

**Deliverables:**

**1. Build dispatcher** (`src/lib/build/dispatcher.ts`):
- On `plan.frozen`: create Snapshot at current seq, create BuildRun with `inputSnapshotId`
- Decompose plan sections into tasks
- For each task: call AI to generate/update artifact (via same tool pipeline)
- Status updates emit events: `build.run.started`, status updates

**2. Verifier** (`src/lib/build/verifier.ts`):
- For each artifact version produced by the build:
  - HTML: validate structure (no broken refs, valid HTML)
  - TypeScript/React: run typecheck (future: WebContainer `tsc --noEmit`)
  - Rust: compile to WASM via cloud (future)
  - Run any tests specified in plan acceptance criteria
- Each gate: emit `build.gate.recorded` event with pass/fail + evidenceRef
- All gates pass → emit `build.run.completed { status: 'verified' }`
- Any gate fails → attempt fix (re-call AI with error), up to 3 rounds
- After 3 failed rounds → emit `build.run.completed { status: 'failed' }`

**3. Promoter** (`src/lib/build/promoter.ts`):
- On verified build: emit `build.run.promoted`
- Update `workspace.latestPromotedBuildRunId`
- Update `CurrentPreview` projection with promoted preview URL
- Create output Snapshot

**4. Build API** (`src/app/api/builds/route.ts`):
- `POST /api/builds` — queue build for frozen plan
- `GET /api/builds/[id]` — build status + gate evidence

**5. StatusBar wiring:**
- During build: show "Building... (2/5 gates passed)" with progress
- On complete: show "Verified ✓" or "Failed ✗"

**Test:** Freeze a plan. Build dispatches. AI generates artifacts. Gates run (HTML validation for MVP). All pass → build verified → promoted. StatusBar updates. CurrentPreview points to promoted build. Compiled toggle in TopBar shows the promoted preview.

---

### WAVE 6: Snapshot / Restore (1 worker, 2-3 hours)
**Scope:** `src/lib/kernel/snapshot.ts`, restore API
**Contract:** Snapshots capture all state boundaries. Restore forks a hidden branch — never rewinds.

**Why fork, not rewind:**
If a branch has events 1..100 and you "restore" to snapshot-at-60, you cannot append seq 61 again without colliding with old history. So restore creates a new hidden branch based on the snapshot. The old branch is untouched. Append-only truth preserved.

**Restore flow:**
1. `snapshot.created` (if not already exists for that point)
2. `branch.forked_from_snapshot` — new Branch with `baseSnapshotId = snapshot.id`, `nextSeq = snapshot.atSeq + 1`, `hiddenInUi = true`
3. `workspace.active_branch_changed` — Workspace.activeBranchId points to new branch
4. Rebuild projections: truncate projection tables, replay events from new branch's base snapshot through its events
5. Restore canvas state from `snapshot.canvasSnapshotRef` into tldraw store

The user experiences "reset." The system experiences "fork."

**Snapshot structure** (captures ALL state boundaries, not just event position):
- `sourceBranchId` + `atSeq` — event stream position
- `canvasSnapshotRef` — serialized tldraw state blob (so spatial layout is restored)
- `activePlanId` — which plan was active
- `latestPromotedBuildRunId` — which build was promoted
- `projectionStateRef` — optional optimization for fast restore without replay

**Deliverables:**
- `src/lib/kernel/snapshot.ts`:
  - `createSnapshot(workspaceId, summary)` — captures current branch atSeq, serializes tldraw canvas, records active plan and promoted build. Stores canvas state in blob store.
  - `restoreSnapshot(workspaceId, snapshotId)` — creates new branch from snapshot, switches workspace active branch, rebuilds projections, restores canvas state
- Auto-snapshot triggers: plan freeze, build promotion
- `GET /api/snapshots?workspaceId=X` — list snapshots with summaries
- `POST /api/snapshots/restore` — executes restore flow

**Test:**
- Create 5 events on main branch. Take snapshot S1. Create 5 more events (seq 6-10).
- Restore to S1. New hidden branch created. Active branch switched.
- ChatFeedItem projection shows only events 1-5 from original branch.
- New events append to new branch starting at seq 6 (no collision with old branch).
- Old branch events 1-10 still exist in events table, untouched.
- Canvas state matches what was captured in S1.
- Take another snapshot S2 on new branch. Restore to S2. Works recursively.

---

### WAVE 7: Git Export (1 worker, 2 hours, dormant for MVP)
**Scope:** `src/lib/git/*`, GitBinding schema
**Contract:** Export promoted code state to Git repo. Not the core truth path.

**Deliverables:**
- `POST /api/git/export` — take latest promoted BuildRun → collect all artifact version files → commit to Git repo
- GitBinding CRUD: link workspace to GitHub/GitLab repo
- Export only: code files, not conversation/canvas/provenance (those stay in the event store)
- Mode: 'export' (one-way push) for MVP. 'mirror' (bidirectional) for future.

**Test:** Create workspace with verified build. Export to Git. Verify all artifact files are in the repo. Verify conversation history is NOT in the repo.

---

## DISPATCH ORDER

| Wave | Workers | Dependencies | Time Est | What |
|------|---------|-------------|----------|------|
| 0A | 1 | None | 4-5 hrs | Kernel: schema, transactional append, projector w/ checkpoints, blob store, rebuild, outbox, event API, seed |
| 0B | 1 | None | 1-2 hrs | Canvas: tldraw setup, CanvasBinding, ArtifactShape |
| 1 | 1 | 0A | 2-3 hrs | Shell wired to projections, SSE live updates, PlanCard |
| 2 | 1 | 0A + 1 | 3-4 hrs | AI chat: streaming, tools → events, GC personality |
| 3 | 1 | 0A | 2-3 hrs | Artifact versions, blob store, poster gen, preview routing |
| 4 | 1 | 2 | 2-3 hrs | Plan proposal, freeze, checksum, approval UI |
| 5 | 1 | 3 + 4 | 3-4 hrs | Build runs (pinned to snapshot), gate evidence, promotion |
| 6 | 1 | 5 | 2-3 hrs | Snapshots (captures canvas + plan + build state), fork-based restore |
| 7 | 1 | 5 | 2 hrs | Git export (dormant) |
| 4 | 1 | 2 | 2-3 hrs | Plan proposal, freeze, checksum, approval UI |
| 5 | 1 | 3 + 4 | 3-4 hrs | Build runs, gate evidence, verification, promotion |
| 6 | 1 | 5 | 2 hrs | Snapshots, restore |
| 7 | 1 | 5 | 2 hrs | Git export (dormant) |

**Parallelism:** 0A and 0B run in parallel. After 0A completes, Waves 1 and 3 can run in parallel. Wave 2 needs Wave 1. Wave 4 needs Wave 2. Wave 5 needs Waves 3 + 4. Waves 6 + 7 are sequential after 5.

**Critical path:** 0A → 1 → 2 → 4 → 5. Total: ~14-18 hours on critical path.

---

## GC SYSTEM PROMPT

(See Wave 2 deliverables above for full prompt.)

Summary: friendly but direct general contractor. Asks questions first. Pushes back on unclear requirements. Proposes structured plans. Uses tools to create artifacts. Says "we" not "I." Never generates code in chat text.

---

## ENVIRONMENT VARIABLES

```env
DATABASE_URL="postgresql://user:pass@localhost:5432/gc"
ANTHROPIC_API_KEY="sk-ant-..."
BLOB_STORE_PATH="./data/blobs"

# Model routing (defaults)
GC_MODEL_BRAINSTORM="claude-haiku-4-5-20251001"
GC_MODEL_BUILD="claude-sonnet-4-6-20260321"
GC_MODEL_ARCHITECT="claude-opus-4-6-20260321"

# WebContainers (Wave 7+)
NEXT_PUBLIC_ENABLE_WEBCONTAINERS=false
```

---

## WHAT THIS BLUEPRINT DOES NOT COVER

- Mobile-specific UI
- User auth / accounts
- Payments / billing
- Team collaboration
- WASM compilation pipeline
- Production deployment
- Custom domains
- Version history UI (undo/redo)
- File export (zip download)
- Voice input
- Image generation
- Analytics
- Rate limiting

These are real but they're not MVP. Ship the core loop first:
**Conversation → Plan → Freeze → Build → Verify → Promote → Iterate.**
