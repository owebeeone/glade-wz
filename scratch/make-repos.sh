#!/usr/bin/env bash
#
# Create the glade GitHub repos (the gwz root + one per NEW workspace member), each with a description, AND wire
# each local repo to its new remote:
#   1. gh repo create                        — make the empty remote
#   2. git -C <path> remote add origin <url> — point the local repo's origin at it
#   3. gwz repo sync <path>                  — let gwz adopt the member's remote   (members only)
#
# Requires: gh (authenticated — `gh auth status`), git, gwz. Idempotent: skips a repo that already exists and an
# origin that's already set, so it's safe to re-run.
#
# The root (glade-wz) is the gwz CONTAINER, not a member — it gets a remote but no `gwz repo sync`; push it
# directly when ready:  git -C <root> push -u origin <branch>
#
# NOT covered here (pre-existing remotes, pending pin/migration as members via gwz, not creation):
#   glade, grip-core, grip-react, taut, taut-shape(+ts/rs/py)
#
# LICENSE: intentionally NO --license flag. (a) gh's --license takes a single template, so it can't express a
#   dual license; (b) --license auto-creates an initial commit on the remote, which would diverge from each
#   member's existing local history and break the first push. Add licenses as repo CONTENT instead.
#
set -euo pipefail

OWNER="owebeeone"     # GitHub org to create under; empty = your personal account (resolved to your login below)
VIS="--public"        # visibility: --private | --public | --internal
REMOTE_PROTO="ssh"    # protocol for the origin remote URL: ssh | https

# Locate the gwz root (this script lives in scratch/) and run from there so member paths + gwz are in context.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# Pre-flight: gh must be authenticated.
if ! gh auth status >/dev/null 2>&1; then
  echo "Error: You are not authenticated with the GitHub CLI ('gh'). Please run 'gh auth login' first." >&2
  exit 1
fi

# Resolve OWNER to your login when blank, so every target is an unambiguous owner/name.
OWNER="${OWNER:-$(gh api user --jq .login)}"

# Create the empty remote (idempotent), point the local repo's origin at it (idempotent), and — for gwz members
# — let gwz adopt the remote.  $3 = local path, $4 = "sync" | "nosync".
ensure() {
  local name="$1" desc="$2" path="$3" mode="$4"
  local target="$OWNER/$name"

  if gh repo view "$target" >/dev/null 2>&1; then
    echo "== $target already exists"
  else
    echo "== creating $target"
    # --clone=false: don't let gh interactively offer to clone the new (empty) repo.
    gh repo create "$target" $VIS --description "$desc" --clone=false
  fi

  local url
  if [ "$REMOTE_PROTO" = ssh ]; then
    url="$(gh repo view "$target" --json sshUrl --jq .sshUrl)"
  else
    url="$(gh repo view "$target" --json url --jq .url).git"
  fi

  if git -C "$path" remote get-url origin >/dev/null 2>&1; then
    echo "   origin already set: $(git -C "$path" remote get-url origin)"
  else
    echo "   git -C $path remote add origin $url"
    git -C "$path" remote add origin "$url"
  fi

  if [ "$mode" = sync ]; then
    echo "   gwz repo sync $path"
    gwz repo sync "$path"
  fi
}

# A gwz member lives in a subdir named after the repo; the root (glade-wz) is the gwz container itself.
mk()   { ensure "$1" "$2" "$1" sync; }
root() { ensure "$1" "$2" "."  nosync; }

root glade-wz      "glade root — a gwz multi-repo WORKSPACE (clone with gwz, not git): the glade/glial design corpus (dev-docs), plan-docs, and member manifest. See AGENTS_GWZ.md"

mk glade-decl      "glade declaration-surface CONTRACT — taut schema + exported IR + golden corpus for GladeId/Shape/Domain/Zone/BindingDecl/ChangeEvent; zero language code, oracle mandatory (role: contract)"
mk glade-decl-ts   "glade-decl TS rendering — @owebeeone/glade-decl: committed generated types + thin index, corpus-gated against the pinned contract (role: types)"
mk glade-decl-rs   "glade-decl Rust rendering — the glade-decl crate: committed generated types, corpus-gated against the pinned contract (role: types)"
mk glade-decl-py   "glade-decl Python rendering — glade_decl: committed generated types, corpus-gated against the pinned contract (role: types)"
mk ggg-viz         "glade trace atlas — interactive protocol scenario explorer (grip-react, no react state hooks): 25+ traces, invariants, state-anchored comment loop; the executable design spec for glade/glial/grazel"

echo "Done."
