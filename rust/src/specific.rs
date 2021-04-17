use crate::resources::Resources;
use anyhow::Result;
use crate::run;

pub fn run_specific(resources: Resources, year: usize, day: usize) {
    let results = run(&resources, year, day).unwrap();
    let answer1 = resources.read_answer(year, day, 1);
    let answer2 = resources.read_answer(year, day, 2);

    println!("Year {} Day {}", year, day);

    if let Some(results) = results {
        check_answer(1, answer1, &results.0);
        check_answer(2, answer2, &results.1);
    } else {
        println!("Not implemented");
    }
}

fn check_answer(part: usize, answer: Result<Option<String>>, result: &String) {
    match answer {
        Ok(answer) => match answer {
            None => println!("No answer provided for part {}, calculated was '{}'", part, result),
            Some(answer) => {
                if &answer == result {
                    println!("Provided answer and result match for part {}, both are {}", part, answer);
                } else {
                    println!("Provided answer and result mismatch for part {}, answer: '{}' result: '{}'", part, answer, result);
                }
            }
        }
        Err(err) => println!("Unable to open answer for part {}: {:?}", part, err)
    }
}