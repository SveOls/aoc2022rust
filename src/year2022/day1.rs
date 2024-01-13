use super::*;
use std::error::Error;

impl Run for Aoc<2022, 1> {
    fn parta(&self) -> Result<AocResult, Box<dyn Error>> {
        Ok(self
            .split("\n\n")
            .map(|x| {
                x.split('\n')
                    .filter(|x| !x.is_empty())
                    .map(|y| y.parse::<i64>().unwrap())
                    .sum::<i64>()
            })
            .max()
            .unwrap()
            .into())
    }
    // fn partb(&self) -> Result<AocResult, Box<dyn Error>> {
    //     unimplemented!()
    // }
}
