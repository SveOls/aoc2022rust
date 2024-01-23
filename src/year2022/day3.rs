use super::*;

impl Run for Aoc<2022, 3> {
    fn parta(&self) -> Result<AocResult, AocError> {
        Ok(self
            .lines()
            .map(|x| x.split_at(x.char_indices().nth(x.chars().count() / 2).unwrap().0))
            .map(|(a, b)| a.chars().find(|&o| b.chars().any(|p| p == o)).unwrap())
            .map(|c| (c as u8 - if c.is_ascii_lowercase() { 96 } else { 38 }) as i64)
            .sum::<i64>()
            .into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        Ok(self
            .lines()
            .array_chunks::<3>()
            .map(|[x, y, z]| {
                x.chars()
                    .find(|&a| y.chars().any(|b| b == a) && z.chars().any(|c| c == a))
                    .unwrap()
            })
            .map(|c| (c as u8 - if c.is_ascii_lowercase() { 96 } else { 38 }) as i64)
            .sum::<i64>()
            .into())
    }
}
