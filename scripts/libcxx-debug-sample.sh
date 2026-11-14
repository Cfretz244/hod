#!/bin/sh
# A sample of a libc++ `-g` run's modules, regenerated as LLVM IR text for
# `cargo xtask debug trees` and `debug verify` (WP-210;
# docs/status/libcxx-hod-debug.md "Debug information"). The suite run
# deletes its objects; this takes the modules hod built (coverage lines
# `hod`), orders them by test path, keeps every k-th so that <count> are
# spread over the directories, and reruns each test's own compile command
# from args.tsv (the run's flags, `-g` among them) with `-S -emit-llvm`.
#
#   scripts/libcxx-debug-sample.sh <logdir> <outdir> <count>
#
# Writes <outdir>/<key>.ll and <outdir>/sample.tsv (key, test path).
# Environment: KILN (kiln++; for x86-64 the wrapper chain without hod,
# e.g. scripts/libcxx-x64-kiln.sh with HOD_X64_KILN unset), LLVM (default
# /usr/lib/llvm-22).
set -eu
logdir=$1 out=$2 count=$3
: "${KILN:?}"
LLVM=${LLVM:-/usr/lib/llvm-22}
mkdir -p "$out"
# key <TAB> test, for every compile of a test source whose module hod built
awk -F'\t' '/ -c -o / { n = split($2, a, " "); for (i = 1; i <= n; i++)
        if (index(a[i], "/libcxx/test/") && a[i] ~ /\.cpp$/) {
            print $1 "\t" substr(a[i], index(a[i], "/libcxx/test/") + 13); break } }' \
    "$logdir/args.tsv" | sort -t '	' -k2,2 -u | while IFS='	' read -r key test; do
        f="$logdir/cov/$key.tsv"
        if [ -f "$f" ] && grep -q '^hod	' "$f"; then printf '%s\t%s\n' "$key" "$test"; fi
    done > "$out/all.tsv"
total=$(wc -l < "$out/all.tsv")
step=$(( total / count )); [ "$step" -ge 1 ] || step=1
awk -v s="$step" -v c="$count" '(NR - 1) % s == 0 && n < c { print; n++ }' "$out/all.tsv" > "$out/sample.tsv"
while IFS='	' read -r key test; do
    argv=$(awk -F'\t' -v k="$key" '$1 == k && / -c -o / { print $2; exit }' "$logdir/args.tsv")
    ll="$out/$key.ll"
    # shellcheck disable=SC2086 # the recorded command line, split as recorded
    set -- $(printf '%s' "$argv" | sed "s| -c -o [^ ]*| -S -emit-llvm -o $ll|")
    KILN_LLVM_DIR=$LLVM "$KILN" "$@" > /dev/null 2>&1 || echo "regenerating $key ($test) failed" >&2
done < "$out/sample.tsv"
echo "$(wc -l < "$out/sample.tsv") of $total hod-built modules sampled into $out" >&2
