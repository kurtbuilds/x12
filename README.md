# X12

There are two related crates:

- `x12_alt`, which contains structs representing x12 transactions, segments, and elements.
- `serde_x12_alt`, which contains serde implementations to de/serialize to x12 format.

# Example

```rust
use serde_x12::{from_str, to_string};
use x12::Document;
use x12::release_5010::transactions::HealthCareClaim;

fn main() {
    let x12 = include_str!("../data/CHPW_Claimdata.txt");
    let doc = from_str::<Document<HealthCareClaim>>(&x12).unwrap();
    let r = to_string(&doc).unwrap();
    println!("{}", r);
}
```

This code showcases reading, but does not showcase byte for byte identical output, because separator information is lost.
(The input file format uses carriage line breaks, `\r\n`, as a visual separator.) Use this code for a byte-for-byte
identical round-trip:

```rust
use serde_x12::{detect_format, from_str};
use x12::Document;
use x12::release_5010::transactions::HealthCareClaim;

fn main() {
    let x12 = include_str!("../data/CHPW_Claimdata.txt");
    let f = detect_format(x12).unwrap();
    let doc = from_str::<Document<HealthCareClaim>>(&x12).unwrap();
    let r = f.to_string(&doc).unwrap();
    assert_eq!(x12, r);
}
```

# Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
x12_alt = "0.1"
serde_x12_alt = "0.1"
```

The libraries are used as `use x12::..` and `use serde_x12::..`, respectively. The `alt` only prevents a `crates.io` 
naming conflict.

# Discussion

The `x12` crate does not currently make an attempt to share structs between different versions (e.g. 5010 vs 8010). This
could be fixed in future versions.

It also does not make an attempt to unify them via an enum, deserializing based on detecting the version in the `ISA`
header. For now, that functionality is left to library users.

# Warnings

There is one single use of `unsafe` in `serde_x12`. To implement deserialization, the library needs to backtrack when
deserializing an optional, but the `serde` API does not appear to support that. The code passes `miri`, but to my
understanding, the invariant that guarantees the `unsafe` is safe cannot be enforced by the compiler. (Specifically,
the `unsafe` use requires a generic type paramter to be zero-sized, but the compiler can't put sized bounds on generics.)

Full details [here](https://github.com/serde-rs/serde/issues/2712).