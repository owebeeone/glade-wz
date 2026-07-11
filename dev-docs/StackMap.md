# G* Stack Map

Status: working draft

Purpose: define the current document ownership split for the G* stack.

## Framing

`/Users/owebeeone/limbo/glial-dev` remains the dev root for the G* libraries.
The documentation in this folder is split by architectural ownership rather
than by repository or package.

This split exists to keep the stable collaboration substrate smaller than the
full product story, and to keep Grip integration replaceable.

## Stack Split

| Layer | Responsibility | Does Not Own |
| --- | --- | --- |
| `Glade` | share identity, share scope, interest declaration, primary ownership, projection envelope, resync, migration, delegated capability boundaries | app composition, UI runtime internals, transport framing |
| `Grip Share` | thin adapter mapping grip tap declarations onto Glial bindings (declaration plumbing only — no direct tap→Glade coupling) | persistence, assembly, Glade kernel semantics, app/session policy |
| `Glial` | client runtime: local persistence FIRST (browser store), taut-shape-aware assembly, rich incremental change events to taps, optional Glade connectivity mediation; plus environment composition, workspace mounting, session and agent attachment policy, capability issuance, service mounts, product-facing facet composition | wire/replication mechanics (Glade's), Grip runtime internals |
| `Grip/Grok` | local graph execution, taps, drips, local value propagation | distributed ownership, routing, session policy |

## Authoritative Documents

| Topic | Document |
| --- | --- |
| stack split and doc ownership | `/Users/owebeeone/limbo/glade-wz/dev-docs/StackMap.md` |
| program status / stage tracking | `/Users/owebeeone/limbo/glade-wz/dev-docs/GladeProgramStatus.md` |
| open architectural decisions | `/Users/owebeeone/limbo/glade-wz/dev-docs/DecisionLog.md` |
| proposed documentation structure | `/Users/owebeeone/limbo/glade-wz/dev-docs/DocStructurePlan.md` |
| root plan document area | `/Users/owebeeone/limbo/glade-wz/plan-docs/README.md` |
| G* development plan | `/Users/owebeeone/limbo/glade-wz/dev-docs/GLDevPlan.md` |
| Phase 1 libp2p test | `/Users/owebeeone/limbo/glade-wz/dev-docs/Phase1Libp2pTest.md` |
| Glade kernel | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeKernel.md` |
| Glade core requirements | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeRequirements.md` |
| Glade bootstrap model | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeBootstrapModel.md` |
| Glade declaration model | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDeclarationModel.md` |
| Glade declaration package | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDeclarationPackage.md` |
| Glade definition schema strategy | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDefinitionSchemaStrategy.md` |
| Glade declaration DSL | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDeclarationDSL.md` |
| Glade bounded-work exchange semantics | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeExchangeSemantics.md` |
| Glade p2p-first topology | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeP2PFirstTopology.md` |
| Glade provider placement and work assignment | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeProviderPlacement.md` |
| Glade provisioning model | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeProvisioningModel.md` |
| Glade distributed control plane | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDistributedControlPlane.md` |
| Glade control-plane console | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeControlPlaneConsole.md` |
| Glade record envelope | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeRecordEnvelope.md` |
| Glade instance lifecycle | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeInstanceLifecycle.md` |
| Glade scale modes | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeScaleModes.md` |
| Glade workspace directory | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeWorkspaceDirectory.md` |
| Glade authorization model | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeAuthzModel.md` |
| Glade declaration surface (glade-decl) | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeDeclSurface.md` |
| Glade supplier model | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeSupplierModel.md` |
| Glade system-data seam | `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeSystemDataSeam.md` |
| Glial client runtime | `/Users/owebeeone/limbo/glade-wz/dev-docs/glial/GlialClientRuntime.md` |
| Glade API boundary study | `/Users/owebeeone/limbo/glade-wz/dev-docs/GladeHypothenicalApiStudy.md` |
| Grip to Glade mapping | `/Users/owebeeone/limbo/glade-wz/dev-docs/grip-share/GripShareAdapter.md` |
| Grip share advertisement | `/Users/owebeeone/limbo/glade-wz/dev-docs/grip-share/GripShareAdvertisement.md` |
| Glial orchestration layer | `/Users/owebeeone/limbo/glade-wz/dev-docs/glial/GlialOrchestration.md` |
| Glial environment model | `/Users/owebeeone/limbo/glade-wz/dev-docs/glial/GlialEnvironmentModel.md` |
| Glial application manager | `/Users/owebeeone/limbo/glade-wz/dev-docs/glial/GlialApplicationManager.md` |
| Glial trust and capability model | `/Users/owebeeone/limbo/glade-wz/dev-docs/glial/GlialTrustAndCapabilityModel.md` |
| Glial application definition model | `/Users/owebeeone/limbo/glade-wz/dev-docs/glial/GlialApplicationDefinitionModel.md` |
| GripLab declaration sketch | `/Users/owebeeone/limbo/glade-wz/dev-docs/examples/GripLab.glade` |
| GripLab declaration review | `/Users/owebeeone/limbo/glade-wz/dev-docs/examples/GripLabDeclarationReview.md` |
| rapid development environment requirements | `/Users/owebeeone/limbo/glade-wz/dev-docs/requirements/RapidDevEnvironment.md` |
| Glade developer golden path | `/Users/owebeeone/limbo/glade-wz/dev-docs/requirements/GladeDeveloperGoldenPath.md` |
| Glial frankenapp silver path | `/Users/owebeeone/limbo/glade-wz/dev-docs/requirements/GlialFrankenappSilverPath.md` |
| harsh reality triage | `/Users/owebeeone/limbo/glade-wz/dev-docs/requirements/HarshRealityTriage.md` |

## Transitional Documents

The following documents remain useful source material, but they are now
transitional rather than authoritative:

- `/Users/owebeeone/limbo/glade-wz/dev-docs/GlialGlossary.md`
- `/Users/owebeeone/limbo/glade-wz/dev-docs/GlialTopology.md`
- `/Users/owebeeone/limbo/glade-wz/dev-docs/GlialRequirements.md`

They should move to `history/` only after the new scoped documents fully cover
their authoritative content.

## Current Architectural Direction

Current working direction:

- `Glade` is the stable share substrate.
- `Grip Share` is the adapter layer above `Glade`.
- `Glial` is the environment, policy, and composition layer above `Glade`.
- `Grip/Grok` remains a local execution runtime, not the definition of the
  distributed model.

## Rapid Development Constraint

Rapid development is a first-class requirement of the stack.

The target workflow is:

1. build a useful UI or agent-facing mock quickly with Grip
2. run share behavior in single-process in-memory mode with no mandatory mesh
   or hosted server
3. promote the same app to multi-session and source-backed behavior by adding
   `Grip Share` mappings and `Glade` ownership rules

The detailed requirements for this are recorded in
`/Users/owebeeone/limbo/glade-wz/dev-docs/requirements/RapidDevEnvironment.md`.
