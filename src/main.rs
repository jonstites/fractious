extern crate clap;

use clap::{App, Arg};

fn cli() -> (String, String) {
    let matches = App::new("My fractal program")
        .version("0.1")
        .author("Voldemort")
        .about("something fractally")
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("image")
                .default_value("100x100")
                .help("Dimensions of the image, defaults to 100x100")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bounds")
                .short("b")
                .long("bounds")
                .default_value("-2+1i,-1-1i")
                .help("Upper left and lower right coordinates, defaults to -2+1i,-1-1i")
                .takes_value(true),
        )
        .get_matches();

    let dimensions = matches.value_of("image").unwrap().to_string();
    let bounds = matches.value_of("bounds").unwrap().to_string();
    (dimensions, bounds)
}

fn main() {
    let dimensions = cli();
    println!("{:?}", dimensions);
}
