# Open-source projects built by kiln with hod

Each row is a project built from source by kiln with hod as its back end
(`KILN_BACKEND=hod`: kiln hands hod every `-O0` module it can take and
falls back to `llc` when hod declines), against a control built from the
same tree with the same flags by kiln with its LLVM back end. **TUs**:
translation units compiled; **built by hod**: their modules hod wrote the
object of; **fell back**: declined by hod (by refusal name) or never
offered to it by kiln; **strict build**: the build again with no fallback
allowed; **tests against control**: the project's tests in both trees.
The record of each row is the package named.

| project | flavour | TUs | built by hod | fell back | strict build | tests against control |
|---|---|---:|---:|---|---|---|
| LLVM 19.1.7 (WP-178) | -O0, assertions, AArch64+X86 back ends, static, aarch64 Linux host | 2779 | 2777 | 2: 1 declined, `attr:disable-tail-calls` (googletest's `gtest-all.cc`), 1 not offered by kiln (`EntryExitInstrumenter.cpp`, a kiln trigger false positive) | every tool of `check-llvm` and the 68 unit-test binaries: passes, but for the two TUs (built outside strict) | check-llvm at parity: lit 36338 tests (30390 pass, 2 fail in both), unit tests 68 binaries, 8007 tests, identical per test |
