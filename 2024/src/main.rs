extern crate aoc;
use aoc::*;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("AOC Runner")
        .version("1.0")
        .author("Joshua Scina<joshscina@gmail.com>")
        .about("Runs Advent of Code solutions")
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .value_name("DAY")
                .help("Specifies which day's solution to run")
                .required(true),
        )
        .get_matches();

    match matches.get_one::<String>("day").map(|s| s.as_str()) {
        Some("1") => day1::run(),
        Some("2") => day2::run(),
        Some("3") => day3::run(),
        _ => eprintln!("Invalid day specified. Please choose 1 or 2."),
    }
}
