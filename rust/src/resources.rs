use anyhow::{Result, Error, Context};
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

pub struct Resources {
    answer_dir: PathBuf,
    input_dir: PathBuf,
}

impl Resources {
    pub fn new() -> Result<Self> {
        let working_dir = std::env::current_dir().context("Can't get working dir").unwrap();
        let root = working_dir.parent().unwrap();
        Ok(Resources {
            answer_dir: root.join("resources").join("answers"),
            input_dir: root.join("resources").join("inputs"),
        })
    }
}

impl Resources {
    pub fn read_input(&self, year: usize, day: usize) -> Result<String> {
        let path = self.input_dir.join(year.to_string()).join(format!("day{}", day));
        if path.exists() {
            let mut output = String::new();
            File::open(path)?.read_to_string(&mut output)?;
            Ok(output)
        } else {
            Err(Error::msg(format!("No input file found for {}/{}", year, day)))
        }
    }

    pub fn read_answer(&self, year: usize, day: usize, part: usize) -> Result<Option<String>> {
        let path = self.answer_dir.join(year.to_string()).join(format!("day{}p{}", day, part));
        if path.exists() {
            let mut output = String::new();
            File::open(path)?.read_to_string(&mut output)?;
            Ok(Some(output.trim().to_string()))
        } else {
            Ok(None)
        }
    }
}