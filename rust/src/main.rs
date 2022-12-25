mod resources;
mod runner;
mod y2015;
mod specific;
mod util;
mod y2016;

use clap::{App, Arg};
use anyhow::{Result, Context};
use crate::resources::Resources;
use crate::runner::run_all;
use crate::y2015::run_2015;
use crate::y2016::run_2016;
use crate::specific::run_specific;

pub type AoCResult = Result<Option<(String, String)>>;

fn main() {
    let resources = Resources::new().context("Reading resources").unwrap();
    match get_mode() {
        Mode::All => run_all(resources),
        Mode::Specific { year, day } => run_specific(resources, year, day)
    }
}

enum Mode {
    All,
    Specific { year: usize, day: usize },
}

fn get_mode() -> Mode {
    let matches = App::new("Advent of code")
        .arg(Arg::with_name("year")
            .takes_value(true)
            .multiple(false)
            .validator(|value| value.parse::<usize>().and_then(|num| Ok(num >= 2015 && num <= 2022)).map(|_| ()).map_err(|_| String::from("Must be 2015..=2022")))
            .requires("day"))
        .arg(Arg::with_name("day")
            .takes_value(true)
            .multiple(false)
            .validator(|value| value.parse::<usize>().and_then(|num| Ok(num >= 1 && num <= 25)).map(|_| ()).map_err(|_| String::from("Must be 1..=25")))
            .requires("year"))
        .get_matches();

    if matches.is_present("year") {
        Mode::Specific {
            year: matches.value_of("year").unwrap().parse::<usize>().unwrap(),
            day: matches.value_of("day").unwrap().parse::<usize>().unwrap(),
        }
    } else {
        Mode::All
    }
}

pub fn run(resources: &Resources, year: usize, day: usize) -> AoCResult {
    match year {
        2015 => run_2015(resources, day),
        2016 => run_2016(resources, day),
        2015..=2022 => Ok(None),
        _ => unimplemented!()
    }
}

