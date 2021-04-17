mod day1;
mod day2;

use crate::resources::Resources;
use crate::y2015::day1::run_2015_1;
use crate::AoCResult;
use crate::y2015::day2::run_2015_2;

pub fn run_2015(resources: &Resources, day: usize) -> AoCResult {
    match day {
        1 => run_2015_1(resources.read_input(2015, 1)?),
        2 => run_2015_2(resources.read_input(2015, 2)?),
        3..=25 => Ok(None),
        _ => panic!("Invalid day {} for 2015", day)
    }
}