use crate::resources::Resources;
use anyhow::Result;
use crate::run;
use std::collections::BTreeMap;
use rayon::prelude::*;

const PADDING: &str = "     ";

pub fn run_all(resources: Resources) {
    for year in 2015..=2020 {
        println!("\n\n{}\n", year);
        (1_usize..=25)
            .into_par_iter()
            .map(|day| run_day(&resources, year, day))
            .collect::<BTreeMap<usize, String>>()
            .iter()
            .for_each(|(_, result)| println!("{}", result));
    }
}

fn run_day(resources: &Resources, year: usize, day: usize) -> (usize, String) {
    let results = run(&resources, year, day);
    let answer1 = resources.read_answer(year, day, 1);
    let answer2 = resources.read_answer(year, day, 2);

    if let Ok(results) = results {
        if let Some(results) = results {
            let answer1 = check_answer(results.0, answer1);
            let answer2 = check_answer(results.1, answer2);
            (day, format!("{: >2}{}[{}] [{}]", day, PADDING, answer1, answer2))
        } else {
            (day, format!("{: >2}{}Not implemented", day, PADDING))
        }
    } else {
        (day, format!("{: >2}{}Error running", day, PADDING))
    }
}

fn check_answer(result: String, answer: Result<Option<String>>) -> String {
    if let Ok(answer) = answer {
        if let Some(answer) = answer {
            if result == answer {
                String::from("✓")
            } else {
                String::from("✖")
            }
        } else {
            String::from("?")
        }
    } else {
        String::from("⚠")
    }
}