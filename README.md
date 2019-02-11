# Fractious

[![Build Status](https://travis-ci.com/jonstites/fractals.svg?branch=master)](https://travis-ci.com/jonstites/fractals)

## About

A simple command-line program for generating images of the Mandelbrot set.

Run with:
`./fractious  -o mandelbrot.jpeg  -p 12000x8000`

There are options for changing the region and the size of the image:
```
USAGE:
    fractious [OPTIONS] --output <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>                        File to display the image
    -p, --pixel_dimensions <pixel_dimensions>    Dimensions of the image, defaults to 600x400 [default: 600x400]
    -r, --region <region>
            Upper left and lower right coordinates, defaults to -2+1i,1-1i [default: -2+1i,1-1i]
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