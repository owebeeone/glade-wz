#!/bin/sh
# Link, then sign the output with codesign (ad-hoc by default).
#
# The macOS firewall files its allow and deny decisions against a code
# signature. The linker's own ad-hoc signature ("linker-signed") is not one it
# keeps, so a freshly linked binary that listens or connects is flagged on
# every run. A codesign ad-hoc signature is kept, for that build.
# .cargo/config.toml at the workspace root routes every macOS link here, so
# glade-node, the other members' binaries and every test executable are signed.
#
# GLADE_CODESIGN_IDENTITY names a keychain code-signing identity to use instead
# of ad-hoc ("-"). A certificate keeps one identity across rebuilds; ad-hoc
# signatures change with every build.
cc "$@" || exit $?

out=""
prev=""
for arg in "$@"; do
    if [ "$prev" = "-o" ]; then
        out="$arg"
    fi
    case "$arg" in
        @*)
            # rustc passes a long command line as a response file, one
            # argument per line.
            file="${arg#@}"
            if [ -f "$file" ]; then
                found=$(awk 'take { print; exit } $0 == "-o" { take = 1 }' "$file")
                if [ -n "$found" ]; then
                    out="$found"
                fi
            fi
            ;;
    esac
    prev="$arg"
done

if [ -n "$out" ] && [ -f "$out" ]; then
    if ! codesign --force --sign "${GLADE_CODESIGN_IDENTITY:--}" "$out" 2>/dev/null; then
        echo "macos-link-sign: could not sign $out; it keeps the linker's signature" >&2
    fi
fi
exit 0
