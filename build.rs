use core::fmt;
use std::{env, fs::File, io::Write, path::Path};

use bitvec::{order::Lsb0, vec::BitVec};
const HIGHEST: u64 = 1024;
fn uints() -> impl Iterator<Item = u64> {
    let first2: u32 = 11; // (highest as f64).log(2.0).round() as u32 + 1;
    let first10: u32 = 4; // (highest as f64).log(10.0) as u32 + 1;
    (0..(HIGHEST + 1))
        .chain((first2..64).map(|i| 2u64.pow(i)))
        .chain((first10..20).map(|i| 10u64.pow(i)))
}

pub enum LitCode {
    U0,
    U1,
    ZeroBit(Box<LitCode>),
    OneBit(Box<LitCode>),
}

impl fmt::Display for LitCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LitCode::ZeroBit(ref bits) => write!(f, "B<{}, _0>", bits),
            LitCode::OneBit(ref bits) => write!(f, "B<{}, _1>", bits),
            LitCode::U0 => write!(f, "_0"),
            LitCode::U1 => write!(f, "_1"),
        }
    }
}

pub fn gen_uint(u: u64) -> LitCode {
    let bits: BitVec<u64, Lsb0> = BitVec::from_element(u);
    let mut bits = bits.into_iter().take(64 - u.leading_zeros() as usize).rev();
    if bits.next().is_none() {
        return LitCode::U0;
    }
    let mut current = LitCode::U1;
    for is_bit in bits {
        current = if is_bit {
            LitCode::OneBit(Box::new(current))
        } else {
            LitCode::ZeroBit(Box::new(current))
        };
    }
    current
}

fn main() {
    println!("cargo:rerun-if-changed=build/main.rs"); // Allow caching the generation if `src/*` files change.

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("consts.rs");

    let mut f = File::create(&dest).unwrap();

    // Header stuff here!
    write!(
        f,
        "
/**
Type aliases for many constants.

This file is generated by `type_eval`'s build script.

We define aliases for

- Numbers 0 through {highest}
- Powers of 2 below `u64::MAX`
- Powers of 10 below `u64::MAX`

These alias definitions look like this:

```rust
use type_eval::prelude::*;

# #[allow(dead_code)]
type U6 = B<B<_1, _1>, _0>;
```

*/
use crate::prelude::*;
",
        highest = HIGHEST,
    )
    .unwrap();

    for u in uints() {
        writeln!(f, "    pub type U{} = {};", u, gen_uint(u)).unwrap();
    }
}
