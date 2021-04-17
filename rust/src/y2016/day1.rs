use crate::AoCResult;
use anyhow::{Result, Error};
use std::collections::HashSet;
use std::ops::Range;

pub(super) fn run_2016_1(input: String) -> AoCResult {
    let steps = input.split(",")
        .map(|step| parse(step.trim()).unwrap())
        .collect();

    let (distance, first_repeat) = travel(steps);

    Ok(Some((distance.to_string(), first_repeat.to_string())))
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Coord { x, y }
    }
}

impl Coord {
    fn dist(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    fn walk(&mut self, dir: &Direction, mov: Move) -> Vec<Coord> {
        let mut walked = vec![];
        match dir {
            Direction::North => {
                walked = (1..mov.dist())
                    .map(|val| Coord::new(self.x, self.y + val))
                    .collect();
                self.y += mov.dist();
            },
            Direction::East => {
                walked = (1..mov.dist())
                    .map(|val| Coord::new(self.x + val, self.y))
                    .collect();
                self.x += mov.dist();
            },
            Direction::West => {
                walked = (1..mov.dist())
                    .map(|val| Coord::new(self.x - val, self.y))
                    .collect();
                self.x -= mov.dist();
            },
            Direction::South => {
                walked = (1..mov.dist())
                    .map(|val| Coord::new(self.x, self.y - val))
                    .collect();
                self.y -= mov.dist();
            },
        }
        walked
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Move {
    Left(usize),
    Right(usize),
}

impl Move {
    fn dist(&self) -> isize {
        (match self {
            Move::Left(dist) => *dist,
            Move::Right(dist) => *dist
        }) as isize
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    fn update(&self, mov: &Move) -> Direction {
        match self {
            Direction::North => if let Move::Left(_) = mov { Direction::West } else { Direction::East }
            Direction::East => if let Move::Left(_) = mov { Direction::North } else { Direction::South }
            Direction::West => if let Move::Left(_) = mov { Direction::South } else { Direction::North }
            Direction::South => if let Move::Left(_) = mov { Direction::East } else { Direction::West }
        }
    }
}

fn parse(input: &str) -> Result<Move> {
    let mut chars = input.chars();
    if input.len() == 1 { return Err(Error::msg(format!("Too short: {}", input))); }
    let dir = chars.next().unwrap().to_ascii_uppercase();
    let dist = chars.collect::<String>().parse::<usize>()?;
    match dir {
        'R' => Ok(Move::Right(dist)),
        'L' => Ok(Move::Left(dist)),
        _ => Err(Error::msg(format!("Invalid letter: {}", input)))
    }
}

fn travel(moves: Vec<Move>) -> (usize, usize) {
    let mut visited = HashSet::new();
    let mut pos = Coord::new(0, 0);
    let mut dir = Direction::North;
    let mut first_repeat: Option<Coord> = None;
    for mov in moves {
        dir = dir.update(&mov);
        let walked = pos.walk(&dir, mov);
        for step in walked {
            if first_repeat.is_none() {
                if visited.contains(&step) {
                    first_repeat = Some(step.clone());
                }
                visited.insert(step);
            }
        }
    }
    (pos.dist(), first_repeat.and_then(|coord| Some(coord.dist())).unwrap_or(0))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let r12 = parse("R12");
        let l1 = parse("L1");
        let r298 = parse("R298");

        assert_eq!(r12.unwrap(), Move::Right(12));
        assert_eq!(l1.unwrap(), Move::Left(1));
        assert_eq!(r298.unwrap(), Move::Right(298));

        let err1 = parse("RR");
        let err2 = parse("R");
        let err3 = parse("K");

        assert!(err1.is_err());
        assert!(err2.is_err());
        assert!(err3.is_err());
    }

    #[test]
    fn test_part_1_samples() {
        let dist_5 = run_2016_1(String::from("R2, L3"));
        let dist_2 = run_2016_1(String::from("R2, R2, R2"));
        let dist_12 = run_2016_1(String::from("R5, L5, R5, R3"));

        assert_eq!(dist_5.unwrap().unwrap().0, "5");
        assert_eq!(dist_2.unwrap().unwrap().0, "2");
        assert_eq!(dist_12.unwrap().unwrap().0, "12");
    }

    #[test]
    fn test_part_2_samples() {
        let dist_4 = run_2016_1(String::from("R8, R4, R4, R8"));

        assert_eq!(dist_4.unwrap().unwrap().1, "4");
    }
}