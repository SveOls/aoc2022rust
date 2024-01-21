use super::*;

impl Run for Aoc<2022, 1> {
    fn parta(&self) -> Result<AocResult, AocError> {
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
    fn partb(&self) -> Result<AocResult, AocError> {
        Ok(self
            .split("\n\n")
            .map(|x| {
                x.split('\n')
                    .filter(|x| !x.is_empty())
                    .map(|y| y.parse::<i64>().unwrap())
                    .sum::<i64>()
            })
            .fold([0, 0, 0], |acc, x| {
                [
                    acc[0].max(x.min(acc[1].min(acc[2]))),
                    acc[1].max(x.min(acc[2])),
                    acc[2].max(x),
                ]
            })
            .iter()
            .sum::<i64>()
            .into())
    }
}
