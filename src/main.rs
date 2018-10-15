extern crate clap;
extern crate svg;
use clap::{App, Arg};

fn main() {
    let args = App::new("SVG curves tool")
        .version("0.1.0")
        .author("Patrick Elsen <pelsen@xfbs.net>")
        .about("Transforms SVG quadratic and cubic (bézier) curves.")
        .arg(Arg::with_name("INPUT")
             .help("Which file to process")
             .required(true)
             .index(1))
        .arg(Arg::with_name("verbose")
             .help("Enable verbose output")
             .short("v")
             .multiple(true))
        .get_matches();

    let input_file_name = args.value_of("INPUT").unwrap();

    process(input_file_name)
}

fn process(file_name: &str) {
}
