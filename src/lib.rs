// unsafe permitted only at the dlopen/dlsym boundary; audited per-function -- NO crate-level forbid.
//! # gnucobol-rs-link
//!
//! gnucobol-rs inter-program linkage: CALL/CANCEL, BY REFERENCE/CONTENT/VALUE parameter passing, and dynamic program loading.
//!
//! A faithful-port satellite of the **gnucobol-rs** ecosystem (an oracle-first Rust compatibility court
//! for GnuCOBOL 3.2). Ports: CALL/linkage + dynamic loading (libcob/call.c). Intended profile: std (dynamic ABI); unsafe only at the dlopen/dlsym boundary.
//!
//! LICENSE: LGPL-3.0-or-later (faithful derivative of GnuCOBOL/libcob; FSF copyright retained). See
//! COPYING.LESSER (+ COPYING). Ecosystem rule: gnucobol-rs-* depend on the gnucobol-rs core; the core does
//! not depend on the satellites; kobold-* (Apache-2.0, separate) is the forensic-intelligence layer above.
//!
//! Status: SCAFFOLD. Implementation follows the split/planning pass, statement-by-statement against the
//! admitted GnuCOBOL 3.2 oracle (byte-exact, fail-closed), with the existing court/receipt discipline.

/// Crate scaffold marker; replace with the real public API as the implementation lands.
pub const GNUCOBOL_RS_SATELLITE: &str = "gnucobol-rs-link";
