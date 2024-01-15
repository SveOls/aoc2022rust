use super::*;
use std::error::Error;

impl Run for Aoc<2022, 6> {
    fn parta(&self) -> Result<AocResult, Box<dyn Error>> {
        Ok(self
            .as_bytes()
            .array_windows::<4>()
            .position(|x| {
                x.iter()
                    .flat_map(|y| std::iter::repeat(y).zip(x.iter()))
                    .filter(|x| x.0 == x.1)
                    .count()
                    == 4
            })
            .map(|x| x + 4)
            .unwrap()
            .into())
    }
    fn partb(&self) -> Result<AocResult, Box<dyn Error>> {
        Ok(self
            .as_bytes()
            .array_windows::<14>()
            .position(|x| {
                x.iter()
                    .flat_map(|y| std::iter::repeat(y).zip(x.iter()))
                    .filter(|x| x.0 == x.1)
                    .count()
                    == 14
            })
            .map(|x| x + 14)
            .unwrap()
            .into())
    }
}
