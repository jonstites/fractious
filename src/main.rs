extern crate clap;
extern crate image;
extern crate num;

use clap::{App, Arg};
use fractious::escape_times_region;
use num::Complex;
use std::str::FromStr;

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(i) => match (T::from_str(&s[..i]), T::from_str(&s[i + 1..])) {
            (Ok(left), Ok(right)) => Some((left, right)),
            _ => None,
        },
    }
}

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("fractious")
        .version("0.1.1")
        .author("Jonathan Stites")
        .about("Command-line program for generating images of the Mandelbrot set")
        .arg(
            Arg::with_name("pixel_dimensions")
                .short("p")
                .long("pixel_dimensions")
                .default_value("600x400")
                .help("Dimensions of the image, defaults to 600x400")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("region")
                .short("r")
                .long("region")
                .default_value("-2+1i,1-1i")
                .help("Upper left and lower right coordinates, defaults to -2+1i,1-1i")
                .allow_hyphen_values(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("File to display the image")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pixel_dimensions = matches.value_of("pixel_dimensions").unwrap();
    let pixel_dimensions = parse_pair::<usize>(&pixel_dimensions, 'x')
        .unwrap_or_else(|| panic!("Could not parse dimensions: {:?}", pixel_dimensions));

    let region = matches.value_of("region").unwrap();
    let region = parse_pair::<Complex<f64>>(&region, ',')
        .unwrap_or_else(|| panic!("Could not parse region: {:?}", region));

    let output = matches.value_of("output").unwrap();

    let limit = 255_u32;

    let escape_times = escape_times_region(pixel_dimensions, region, limit);

    let image = escape_times
        .into_iter()
        .map(|i| i.unwrap_or(255_u32))
        .map(|i| 255_u8 - i as u8)
        .collect::<Vec<u8>>();

    image::save_buffer(
        output,
        &image,
        pixel_dimensions.0 as u32,
        pixel_dimensions.1 as u32,
        image::ColorType::Gray(8),
    )
}
