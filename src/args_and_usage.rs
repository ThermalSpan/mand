use clap::{App, Arg};

// Programmer defined constants
static PROGRAM_NAME: &'static str = "mand";

// Derived constants
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Args {
    pub win_width: u32,
    pub win_height: u32,
}

pub fn parse_args() -> Args {
    let args = App::new(PROGRAM_NAME)
        .version(VERSION)
        .author("Russell W. Bentley <russell.w.bentley@icloud.com>")
        .about("A simple graphical mandelbrot set viewer")
        .arg(
            Arg::with_name("WIN_WIDTH")
                .help("The desired window width")
                .long("win-width")
                .value_name("w")
                .default_value("2048")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("WIN_HEIGHT")
                .help("The desired window height")
                .long("win-height")
                .value_name("h")
                .default_value("2048")
                .required(true),
        )
        .get_matches();

    Args {
        win_width: str::parse(args.value_of("WIN_WIDTH").unwrap()).unwrap(),
        win_height: str::parse(args.value_of("WIN_HEIGHT").unwrap()).unwrap(),
    }
}
