# Fractious

`fractious` is a command-line program for generating images of the Mandelbrot set.

[![Build Status](https://travis-ci.com/jonstites/fractious.svg?branch=master)](https://travis-ci.com/jonstites/fractious)
[![Cargo Version](https://img.shields.io/crates/v/fractious.svg)](https://crates.io/crates/fractious)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/fractious.svg)](#license)

## Usage

With the default options, `fractious` produces the following image:

```bash
$ fractious -o mandelbrot.jpeg
```

![An image of the Mandelbrot set created by fractious](https://www.jonstites.com/assets/images/mandelbrot.jpeg)

The region of complex numbers to display can be specified, as can the dimensions of the image:

```bash
$ fractious -o mandelbrot_corner.jpeg -r "-2+1i,0" -p 600x300
```

![An image of a corner of the Mandelbrot set created by fractious](https://www.jonstites.com/assets/images/mandelbrot_corner.jpeg)

## Installation

[Precompiled binaries are available for Linux, macOS, and Windows](https://github.com/jonstites/fractious/releases).

Linux and macOS binaries are static, so nothing else needs to be installed.
Windows binaries require Microsoft Visual C++ (MSVC) Redistributable for Visual Studio 2017.


For Rust programmers, `fractious` can be installed with `cargo`:

```bash
$ cargo install fractious
```

## Build

If you have Rust installed, you can build from source:

```bash
$ git clone git@github.com:jonstites/fractious.git
$ cd fractious
$ cargo build --release
$ ./target/release/fractious --version
fractious 0.1.1
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.