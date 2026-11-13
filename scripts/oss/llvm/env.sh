# Sourced by the WP-178 scripts: where things are. Every path can be
# overridden from the environment.
_here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
_wt="$(cd "$_here/../../.." && pwd)"
_main="$(cd "$(git -C "$_wt" rev-parse --path-format=absolute --git-common-dir)/.." && pwd)"
case "$_main" in */.git/modules*) _main="${_main%%/.git/modules*}" ;; esac
# The workspace that holds hod (a submodule) and kiln beside it.
_ws="$(cd "$_main/.." && pwd)"
export HOD_OSS_ROOT="${HOD_OSS_ROOT:-$_ws/.build/oss}"
export HOD_OSS_KILN="${HOD_OSS_KILN:-$_ws/.build/target-kiln/release}"
export HOD_OSS_KILN_SCRIPTS="${HOD_OSS_KILN_SCRIPTS:-$_ws/kiln/scripts/oss}"
export HOD_OSS_SCRIPTS="$_here"
export HOD_OSS_SRC="${HOD_OSS_SRC:-$HOD_OSS_ROOT/llvm-src}"
export HOD_OSS_FINDINGS="${HOD_OSS_FINDINGS:-$_ws/.build/findings/oss}"
LLVM_TAG=llvmorg-19.1.7
unset _here _wt _main _ws
