# gnucobol-rs-link

Inter-program linkage parameter-passing **byte semantics** (`CALL ... USING BY REFERENCE / BY CONTENT /
BY VALUE`) over the **oracle-proven** [`gnucobol-rs`](https://github.com/infinityabundance/gnucobol-rs) core.

cobc's behavior was mapped by the `GNURUST.CALL.LAYOUT.ATLAS.1` court; this crate reproduces it as a safe
Rust model:

- **BY REFERENCE** — the callee shares the caller's storage (overlay view); a callee write is visible to the
  caller, and an item larger than the field reads/writes into adjacent storage, exactly as cobc does.
- **BY CONTENT** / **BY VALUE** — the callee gets a private sized copy; mutating it never touches the caller.

```rust
use gnucobol_rs_link::{bind_reference, bind_content, bind_numeric_reference};

let mut caller = b"ABCXY".to_vec();
{ let v = bind_reference(&mut caller, 0, 5); v[0] = b'Z'; } // BY REFERENCE write-through
assert_eq!(&caller, b"ZBCXY");

// a narrower numeric LINKAGE item overlays the leading bytes, decoded via the proven core:
assert_eq!(bind_numeric_reference(b"1234", 0, "9(2)").unwrap().unscaled_i128(), Some(12));
```

Dynamic program loading (`dlopen`/`dlsym` for `CALL` by runtime name) is a deferred, separately-gated feature —
the only place audited `unsafe` will appear.

## License
LGPL-3.0-or-later — a faithful derivative of GnuCOBOL/libcob (FSF copyright retained). See COPYING.LESSER (+ COPYING).
