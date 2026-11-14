#!/bin/sh
# The modules a libc++ run's hod declined, regenerated with their detail
# (WP-210; docs/status/libcxx-hod-debug.md "Refusals"). The coverage log
# names only the stable reason (`debug:die-unsupported`); hod's message
# names the node and what is not written. For every coverage file with a
# `fallback` line this reruns the test's own compile command from
# args.tsv with `-S -emit-llvm` and runs hod on the module, strict.
#
#   scripts/libcxx-debug-refused.sh <logdir> <outdir>
#
# Writes <outdir>/<key>.ll, <outdir>/<key>.hod.txt (hod's standard
# error) and <outdir>/refused.tsv (key, test, reason, hod's message).
# Environment: KILN (kiln++, or the x86-64 wrapper without hod), HOD (the
# hod binary), LLVM (default /usr/lib/llvm-22), KEEP=0 to delete the
# modules after reading the message.
set -eu
logdir=$1 out=$2
: "${KILN:?}" "${HOD:?}"
LLVM=${LLVM:-/usr/lib/llvm-22}
mkdir -p "$out"
: > "$out/refused.tsv"
grep -l '^fallback	' "$logdir"/cov/*.tsv 2>/dev/null | while read -r f; do
    key=$(basename "$f" .tsv)
    reason=$(awk -F'\t' '$1 == "fallback" { print $2; exit }' "$f")
    argv=$(awk -F'\t' -v k="$key" '$1 == k && / -c -o / { print $2; exit }' "$logdir/args.tsv")
    test=$(printf '%s' "$argv" | tr ' ' '\n' | awk 'index($0, "/libcxx/test/") && /\.cpp$/ {
        print substr($0, index($0, "/libcxx/test/") + 13); exit }')
    ll="$out/$key.ll"
    # shellcheck disable=SC2086 # the recorded command line, split as recorded
    set -- $(printf '%s' "$argv" | sed "s| -c -o [^ ]*| -S -emit-llvm -o $ll|")
    if KILN_LLVM_DIR=$LLVM "$KILN" "$@" > /dev/null 2>&1 && [ -s "$ll" ]; then
        env -u HOD_COVERAGE_LOG -u HOD_CENSUS -u HOD_FALLBACK_LLC -u HOD_STRICT \
            "$HOD" --strict -O0 -filetype=obj -relocation-model=pic "$ll" -o /dev/null \
            > "$out/$key.hod.txt" 2>&1 || true
        msg=$(grep -m1 'error:' "$out/$key.hod.txt" | sed 's/^hod: error: //') || msg="built"
        [ -n "$msg" ] || msg="built"
    else
        msg="regeneration failed"
    fi
    [ "${KEEP:-1}" = 0 ] && rm -f "$ll"
    printf '%s\t%s\t%s\t%s\n' "$key" "$test" "$reason" "$msg" >> "$out/refused.tsv"
done
