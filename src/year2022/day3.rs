use super::*;
use std::error::Error;

impl Run for Aoc<2022, 3> {
    fn parta(&self) -> Result<AocResult, Box<dyn Error>> {
        Ok(self
            .lines()
            .map(|x| x.split_at(x.char_indices().nth(x.chars().count() / 2).unwrap().0))
            .map(|(a, b)| a.chars().find(|&o| b.chars().any(|p| p == o)).unwrap())
            .map(|c| (c as u8 - c.is_ascii_lowercase().then_some(96).unwrap_or(38)) as i64)
            .sum::<i64>()
            .into())
    }
    fn partb(&self) -> Result<AocResult, Box<dyn Error>> {
        Ok(self
            .lines()
            .array_chunks::<3>()
            .map(|[x, y, z]| {
                x.chars()
                    .find(|&a| y.chars().any(|b| b == a) && z.chars().any(|c| c == a))
                    .unwrap()
            })
            .map(|c| (c as u8 - c.is_ascii_lowercase().then_some(96).unwrap_or(38)) as i64)
            .sum::<i64>()
            .into())
    }
}
