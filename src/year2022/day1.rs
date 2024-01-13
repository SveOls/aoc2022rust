use std::error::Error;
use super::*;

impl Run for Aoc<2022, 1> {
    fn parta(&self) -> Result<String, Box<dyn Error>> {
        let text = self.0.as_str();
        let res = text.split("\n\n").map(|x| x.split('\n').filter(|x| !x.is_empty()).map(|y| y.parse::<i64>().unwrap()).sum::<i64>()).max().unwrap();
        Ok(res.to_string())
    }
    fn partb(&self) -> Result<String, Box<dyn Error>> {
        unimplemented!()
    }
}