# Glial Trust And Capability Model

Status: working draft

Purpose: define who may declare what in a Glial environment and how that maps
to p2p-first Glade validation.

## Core Claim

Glial owns policy and capability issuance for environments, workspaces,
sessions, facets, and agents.

Glade validates declarations mechanically. Glial decides which principals
should receive the capabilities that make those declarations valid.

## Trust Roots

Each environment or workspace MUST have a trust root.

The trust root SHOULD define:
- root principals
- accepted identity providers or certificate authorities
- accepted provider certificates
- capability issuers
- revocation authorities
- key distribution policy
- policy update authority

In a p2p-first model, peers MUST be able to validate trust roots from signed
genesis material rather than relying only on a connected server.

## Identity Classes

Glial SHOULD distinguish:

| Identity | Meaning |
| --- | --- |
| `Principal` | A permission-bearing actor such as a human, service, or agent. |
| `Session` | One runtime attachment for a principal. |
| `Facet` | A mounted app/tool projection operating within an environment. |
| `Provider` | A service principal that can claim Glade work or host content. |
| `TransportPeer` | A network peer identity such as a libp2p `PeerId`. |

`TransportPeer` MUST NOT be treated as the authorization identity unless it is
explicitly bound to a principal.

## Capability Shape

A capability SHOULD describe:
- issuer
- subject principal or session
- workspace scope
- share scope if narrower than workspace
- allowed actions
- expiry
- delegation rights
- revocation reference
- key material reference or wrapped key
- signature

Allowed actions SHOULD include:
- `mount-workspace`
- `open-share`
- `read`
- `write`
- `control`
- `host`
- `claim`
- `define`
- `instantiate`
- `delegate`
- `decrypt`

## Declaration Rights

Glial MUST distinguish rights to:
- define reusable contracts
- instantiate existing definitions
- claim provider authority
- write content
- observe content
- delegate access to another session
- update policy

Those rights SHOULD NOT be collapsed into one broad workspace permission.

Example:
- a UI session may instantiate `exchange:BugsQuery`
- a database service may claim `exchange:BugsQuery`
- the UI session may read responses
- neither may update the genesis policy

## Delegation

Delegation MUST be explicit and attributable.

A delegated capability SHOULD record:
- delegating principal
- receiving principal or session
- delegated actions
- target workspace/share/content ref
- expiry
- revocation path
- whether redelegation is allowed

Agent access SHOULD use delegated capabilities rather than whole-session
visibility by default.

## Encryption Domains

Workspaces SHOULD be the default encryption domains.

Shares MAY use narrower keys where needed.

The model SHOULD support:
- workspace keys
- share keys
- wrapped keys per recipient or group
- key rotation
- capability revocation
- relay-only peers without content keys

Revocation MUST define what happens to:
- future access
- cached ciphertext
- cached plaintext
- local replicas
- retained logs

Full deletion of previously decrypted local data MAY be impossible. The policy
MUST be honest about that.

## Provider Trust

A provider that claims work MUST present proof that it may claim that work.

Provider proof SHOULD bind:
- provider principal
- supported definitions or claim kinds
- workspace scope
- expiry or lease policy
- certificate or capability chain

Providers SHOULD claim work through Glade declarations. They SHOULD NOT be
trusted merely because a peer can route traffic to them.

## Facet Trust

Facets are not security boundaries by default.

A facet MAY request access to shares, but Glial MUST evaluate that request
against the session's capabilities and workspace policy.

Facet manifests SHOULD declare:
- consumed share types
- produced share types
- requested actions
- provider roles if any
- whether the facet can delegate to agents

## Audit And Attribution

Glial SHOULD make every important action attributable.

Records SHOULD preserve:
- principal
- session
- facet if relevant
- provider if relevant
- delegated capability chain
- timestamp
- affected workspace/share/content

This is especially important for AI sessions that may edit files, run
commands, or query external data sources.

## Revocation

Revocation SHOULD be modeled as declarations that invalidate future use of a
capability.

Revocation MUST be visible to peers validating new declarations.

Open questions include:
- how quickly revocation must propagate
- whether offline peers may continue operating temporarily
- how stale claims are detected after revocation
- what diagnostics appear when revoked access is attempted

## Relationship To Glade

Glial issues and governs capabilities.

Glade records bind to those capabilities and validate:
- this declaration was signed by this principal
- this principal had this capability
- this capability allowed this declaration kind
- this capability was valid under the referenced policy

This keeps Glade mechanically verifiable while keeping product and
environment policy in Glial.
