// No crate-level `forbid(unsafe_code)`: the CALL byte-semantics below are 100% safe, but the planned
// dynamic-loading feature (dlopen/dlsym) will scope audited `unsafe` at that boundary only.
//! # gnucobol-rs-link
//!
//! Inter-program linkage parameter-passing **byte semantics** — `CALL ... USING BY REFERENCE / BY CONTENT /
//! BY VALUE` — layered over the **oracle-proven** [`gnucobol-rs`] core. cobc's behavior here was mapped by
//! the `GNURUST.CALL.LAYOUT.ATLAS.1` court; this crate reproduces it as a safe Rust model:
//!
//! - **BY REFERENCE** passes the caller's *address*; a LINKAGE item overlays the caller's storage, and a
//!   callee write is visible to the caller. A LINKAGE item *larger* than the field reads/writes into
//!   adjacent storage (no re-bounds-check at the field) — exactly as cobc does.
//! - **BY CONTENT** passes a sized *copy*; mutating it never touches the caller.
//!
//! Dynamic program loading (`dlopen`/`dlsym` for `CALL` by runtime name) is a deferred feature; it will be
//! the only place audited `unsafe` appears. Faithful-port satellite; LGPL-3.0-or-later.

use gnucobol_rs::{build_field, Decimal, PicError, Usage};

/// The COBOL argument-passing conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassBy {
    /// `BY REFERENCE` — the callee shares the caller's storage (an overlay view).
    Reference,
    /// `BY CONTENT` — the callee gets a private copy.
    Content,
    /// `BY VALUE` — the callee gets the value (here, a copy of the bytes).
    Value,
}

/// `BY REFERENCE`: return a mutable overlay view of `len` bytes of the caller's storage at `offset`. A
/// write through the returned slice mutates the caller; a `len` exceeding the nominal field extends into
/// adjacent storage, per `GNURUST.CALL.LAYOUT.ATLAS.1`.
pub fn bind_reference(caller: &mut [u8], offset: usize, len: usize) -> &mut [u8] {
    &mut caller[offset..offset + len]
}

/// `BY CONTENT` / `BY VALUE`: return a sized private copy (`len` bytes, space-padded if the source is
/// shorter). Mutating the copy never touches the caller.
pub fn bind_content(caller: &[u8], offset: usize, len: usize) -> Vec<u8> {
    let end = (offset + len).min(caller.len());
    let mut v = caller[offset..end].to_vec();
    v.resize(len, b' ');
    v
}

/// Overlay a numeric DISPLAY LINKAGE item described by `pic` over the caller's storage at `offset`
/// (BY REFERENCE) and decode the overlaid bytes via the oracle-proven core. A PIC *narrower* than the
/// caller's field decodes only the LEADING bytes (the proven numeric length-mismatch overlay).
pub fn bind_numeric_reference(caller: &[u8], offset: usize, pic: &str) -> Result<Decimal, PicError> {
    let field = build_field(pic, Usage::Display, false, false)?;
    let end = (offset + field.size).min(caller.len());
    Ok(Decimal::from_display(&caller[offset..end], &field.attr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn by_reference_overlays_adjacent_and_writes_through() {
        // caller PA='ABC' at 0 with adjacent storage 'XY'; callee X(5) over PA sees 'ABCXY'.
        let mut caller = b"ABCXY".to_vec();
        {
            let view = bind_reference(&mut caller, 0, 5);
            assert_eq!(view, b"ABCXY"); // overlay reads into the adjacent field
            view[0] = b'Z'; // callee MOVE 'Z' TO L(1:1)
        }
        assert_eq!(&caller, b"ZBCXY"); // write visible to the caller ('ABC' -> 'ZBC')
    }

    #[test]
    fn by_content_is_a_clean_untouched_copy() {
        let caller = b"DEF".to_vec();
        let mut copy = bind_content(&caller, 0, 3);
        assert_eq!(copy, b"DEF"); // clean sized copy
        copy[0] = b'Z'; // callee writes its copy
        assert_eq!(&caller, b"DEF"); // caller UNCHANGED
    }

    #[test]
    fn numeric_narrower_overlays_leading_bytes() {
        // caller 9(4)=1234 (DISPLAY "1234"); a LINKAGE 9(2) overlays the leading "12" -> 12.
        let caller = b"1234".to_vec();
        let d = bind_numeric_reference(&caller, 0, "9(2)").unwrap();
        assert_eq!(d.unscaled_i128(), Some(12));
    }
}
