use crate::AoCResult;
use std::str::FromStr;
use anyhow::{Result, Error};
use crate::util::index_of_max;

pub(super) fn run_2015_2(input: String) -> AoCResult {
    let result = input.lines()
        .map(|line| parse(line))
        .map(|results| calc(results.unwrap()))
        .reduce(|state, item| (state.0 + item.0, state.1 + item.1))
        .unwrap();
    Ok(Some((result.0.to_string(), result.1.to_string())))
}

fn parse(input: &str) -> Result<[usize; 3]> {
    let numbers: Vec<usize> = input.split("x").filter_map(|num| usize::from_str(num).ok()).collect();
    return if numbers.len() != 3 {
        Err(Error::msg(format!("Invalid number of sides: {}", input)))
    } else {
        Ok([numbers[0], numbers[1], numbers[2]])
    }
}

fn calc(input: [usize; 3]) -> (usize, usize) {
    let idx = index_of_max(&input).unwrap();
    let mut smaller_sides = input.to_vec();
    smaller_sides.remove(idx);

    let paper = (2 * input[0] * input[1]) + (2 * input[1] * input[2]) + (2 * input[0] * input[2]) + (smaller_sides[0] * smaller_sides[1]);
    let ribbon = (smaller_sides[0] + smaller_sides[0] + smaller_sides[1] + smaller_sides[1]) + (input[0] * input[1] * input[2]);

    (paper, ribbon)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let output1 = parse("2x3x4");
        let output2 = parse("1x1x10");

        assert_eq!(output1.unwrap(), [2,3,4]);
        assert_eq!(output2.unwrap(), [1,1,10]);
    }

    #[test]
    fn test_calc() {
        let output1 = calc([2,3,4]);
        let output2 = calc([1,1,10]);

        assert_eq!(output1, (58, 34));
        assert_eq!(output2, (43, 14));
    }

    #[test]
    fn test_part_1_samples() {
        let answer_58 = run_2015_2(String::from("2x3x4"));
        let answer_43 = run_2015_2(String::from("1x1x10"));

        assert_eq!(answer_58.unwrap().unwrap().0, "58");
        assert_eq!(answer_43.unwrap().unwrap().0, "43");
    }

    #[test]
    fn test_part_2_samples() {
        let answer_34 = run_2015_2(String::from("2x3x4"));
        let answer_14 = run_2015_2(String::from("1x1x10"));

        assert_eq!(answer_34.unwrap().unwrap().1, "34");
        assert_eq!(answer_14.unwrap().unwrap().1, "14");
    }
}