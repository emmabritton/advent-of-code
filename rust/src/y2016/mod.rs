mod day1;

use crate::resources::Resources;
use crate::y2016::day1::run_2016_1;
use crate::AoCResult;

pub fn run_2016(resources: &Resources, day: usize) -> AoCResult {
    match day {
        1 => run_2016_1(resources.read_input(2016, 1)?),
        2..=25 => Ok(None),
        _ => panic!("Invalid day {} for 2016", day)
    }
}