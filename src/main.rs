extern crate clap;

use clap::{App, Arg};

mod day1;
mod day2;

fn main() {
    let matches = App::new("Advent of Code 2018")
        .author("Spencer Judd <spencercjudd@gmail.com>")
        .version("0.1.0")
        .arg(
            Arg::with_name("day")
                .help("day number, 1-25")
                .required(true)
                .index(1),
        ).get_matches();

    match matches.value_of("day").unwrap().parse::<u8>().unwrap() {
        1 => day1::solve(),
        2 => day2::solve(),
        3...25 => eprintln!("not yet solved!"),
        _ => eprintln!("day must be between 1 and 25"),
    }
}
