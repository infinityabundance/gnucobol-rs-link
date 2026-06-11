# gnucobol-rs-link

gnucobol-rs inter-program linkage: CALL/CANCEL, BY REFERENCE/CONTENT/VALUE parameter passing, and dynamic program loading.

A faithful-port satellite of the **gnucobol-rs** ecosystem -- an oracle-first Rust compatibility court for
GnuCOBOL 3.2 (byte-exact vs the real cobc/libcob, fail-closed, receipt-backed). This crate ports: CALL/linkage + dynamic loading (libcob/call.c).

## Profile
Intended: **std (dynamic ABI); unsafe only at the dlopen/dlsym boundary**.

## Ecosystem
- gnucobol-rs (core) = oracle-proven data-division primitives (PIC, layout, COMP-3, MOVE, VALUE, arithmetic).
- gnucobol-rs-exec / -io / -intrinsics / -link / -tui = the modular runtime satellites (this is one).
- gnucobol-rs-* MAY depend on the gnucobol-rs core; the core MUST NOT depend on a satellite.
- kobold-* (Apache-2.0, separate repos) = the forensic-intelligence layer ABOVE the ecosystem.

## License
**LGPL-3.0-or-later** (faithful derivative of GnuCOBOL/libcob; FSF copyright retained). See COPYING.LESSER + COPYING.

## Status
Scaffold only -- repo initialized, no implementation yet. Implementation follows the split/planning pass.
