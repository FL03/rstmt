# rstmt

[![crates.io](https://img.shields.io/crates/v/rstmt?style=for-the-badge&logo=rust)](https://crates.io/crates/rstmt)
[![docs.rs](https://img.shields.io/docsrs/rstmt?style=for-the-badge&logo=docs.rs)](https://docs.rs/rstmt)
[![GitHub License](https://img.shields.io/github/license/FL03/rstmt?style=for-the-badge&logo=github)](https://github.com/FL03/rstmt/blob/main/LICENSE)

***

_**Warning: expect heavy changes to the API as the library is currently in the early stages of development and is not yet ready for production use.**

`rstmt` is a generalize music-theory toolkit written in Rust.

## Features

- [ ] American Standard Pitch Notation (ASPN)
- [ ] The Neo-Riemannian Theory

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.rstmt]
features = ["full"]
version = "0.0.x"
```

### Examples

#### _Example #1:_ Using the `Triad` implementation

```rust
    extern crate rstmt;

    use rstmt::Note;
    use rstmt::nrt::Triad;

    fn main() -> Result<(), Box<dyn core::error::Error + Send + Sync + 'static>> {
        let root = Note::from_pitch(0);
        // initialize a c-major triad
        let triad = dbg!(Triad::major(root));
        // test the root of the triad
        assert_eq!(triad.root(), root);
        // test the parallel transformation
        assert_eq!(triad.parallel(), Triad::minor(root));
        // assert the invertibility of the transformations
        assert_eq!(triad.parallel().parallel(), triad);
        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Getting Started

For more information on getting an environment setup to develop the crate view the [QUICKSTART](QUICKSTART.md) file.
