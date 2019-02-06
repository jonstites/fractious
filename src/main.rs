#[macro_use]
extern crate clap;

use clap::{App, Arg};

fn cli() -> (u32, u32) {
    let matches = App::new("My fractal program")
        .version("0.1")
        .author("Voldemort")
        .about("something fractally")
        .arg(Arg::with_name("height")
             .short("h")
             .long("height")
             .default_value("100")
             .help("Height of the fractal image, defaults to 100")
             .takes_value(true))
        .arg(Arg::with_name("width")
             .short("w")
             .long("width")
             .default_value("100")
             .help("Width of the fractal image, defaults to 100")
             .takes_value(true))
        .get_matches();

    // TODO don't actually unwrap, do something nicer
    let height = value_t!(matches, "height", u32).unwrap();
    let width = value_t!(matches, "width", u32).unwrap();

    (height, width)
        
}

fn main() {
    let dimensions = cli();
    println!("{:?}", dimensions);
}
