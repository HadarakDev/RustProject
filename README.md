# Team

* Antoine TAVERNIER.
* Matthieu ALLEXANDRE.
* Nicolas ROCHE.
* Florian BERGERON.

# RustProject

Portable Pixel Map Project written in Rust.

## How to build library

```rust
cargo build
```

## How to run library

```rust
cargo run
```

## How to test library

```rust
cargo test
```

## Execute project

```rust
rustc main.rs
```

## How to build documentation for your library

```rust
cargo doc
```

## How to launch documentation

Launch ```/ppm/doc/docs/index.html``` on your web browser.

## How to use Dynamic ppm Library with python

* Build the dynamic library with `cargo build`
* Import the module ppm.py like this in your python script:
`from ppm import *`

An example file is available at : `/python/main.py`

## Used libraries

* ctypes : last version.
* image : last version.
* ansi_rgb : last version.
* rgb : v 0.8.
* libc : v 0.2.66
