use crate::AoCResult;

pub(super) fn run_2015_1(input: String) -> AoCResult {
    let mut floor = 0;
    let mut basement = None;

    input.chars().enumerate().for_each(|(idx, chr)| {
        match chr {
            '(' => floor+=1,
            ')' => {
                floor-=1;
                if floor < 0 && basement.is_none() {
                    basement = Some(idx + 1)
                }
            },
            _ => {}
        }
    });

    Ok(Some((floor.to_string(), basement.unwrap_or(0).to_string())))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_samples() {
        let floor0_1 = run_2015_1(String::from("(())"));
        let floor0_2 = run_2015_1(String::from("()()"));
        let floor3_1 = run_2015_1(String::from("((("));
        let floor3_2 = run_2015_1(String::from("(()(()("));
        let floor3_3 = run_2015_1(String::from("))((((("));
        let floor_neg1_1 = run_2015_1(String::from("())"));
        let floor_neg1_2 = run_2015_1(String::from("))("));
        let floor_neg3_1 = run_2015_1(String::from(")))"));
        let floor_neg3_2 = run_2015_1(String::from(")())())"));

        assert_eq!(floor0_1.unwrap().unwrap().0, "0");
        assert_eq!(floor0_2.unwrap().unwrap().0, "0");
        assert_eq!(floor3_1.unwrap().unwrap().0, "3");
        assert_eq!(floor3_2.unwrap().unwrap().0, "3");
        assert_eq!(floor3_3.unwrap().unwrap().0, "3");
        assert_eq!(floor_neg1_1.unwrap().unwrap().0, "-1");
        assert_eq!(floor_neg1_2.unwrap().unwrap().0, "-1");
        assert_eq!(floor_neg3_1.unwrap().unwrap().0, "-3");
        assert_eq!(floor_neg3_2.unwrap().unwrap().0, "-3");
    }

    #[test]
    fn test_part_2_samples() {
        let basement_1 = run_2015_1(String::from(")"));
        let basement_5 = run_2015_1(String::from("()())"));

        assert_eq!(basement_1.unwrap().unwrap().1, "1");
        assert_eq!(basement_5.unwrap().unwrap().1, "5");
    }
}