# rstmt

[![license](https://img.shields.io/crates/l/rstmt.svg)](https://crates.io/crates/rstmt)
[![crates.io](https://img.shields.io/crates/v/rstmt.svg)](https://crates.io/crates/rstmt)
[![docs.rs](https://docs.rs/rstmt/badge.svg)](https://docs.rs/rstmt)

[![clippy](https://github.com/FL03/rstmt/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/rstmt/actions/workflows/clippy.yml)
[![rust](https://github.com/FL03/rstmt/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/rstmt/actions/workflows/rust.yml)

***

_**Warning: expect heavy changes to the API as the library is currently in the early stages of development and is not yet ready for production use.**

`rstmt` is a generalize music-theory toolkit written in Rust.

## Features

- [ ] American Standard Pitch Notation (ASPN)
- [ ] The Neo-Riemannian Theory

## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/rstmt.git
cd triad
```

```bash
cargo build --features full -r --workspace
```

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.rstmt]
features = ["full"]
version = "0.0.*"
```

### Examples

#### _`neo`: Apply a parallel transformation onto a c-major triad_

```rust
extern crate rstmt;

use rstmt::prelude::{Note, Triad};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a c-major triad
    let c_major = Triad::major(Note::from_pitch(0));
    // assert the parallel transformation of a c-major triad is a c-minor triad
    assert_eq!(c_major.parallel(), Triad::minor(Note::from_pitch(0)));
    // assert the invertability of transformations
    assert_eq!(c_major.parallel().parallel(), c_major);

    Ok(())
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
