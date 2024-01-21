use super::*;

impl Run for Aoc<2022, 4> {
    fn parta(&self) -> Result<AocResult, AocError> {
        Ok(self
            .lines()
            .map(|line| line.split_once(',').unwrap())
            .map(|(a, b)| {
                [
                    a.split_once('-')
                        .map(|(x, y)| [x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()])
                        .unwrap(),
                    b.split_once('-')
                        .map(|(x, y)| [x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()])
                        .unwrap(),
                ]
            })
            .filter(|[a, b]| a[0] >= b[0] && a[1] <= b[1] || b[0] >= a[0] && b[1] <= a[1])
            .count()
            .into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        Ok(self
            .lines()
            .map(|line| line.split_once(',').unwrap())
            .map(|(a, b)| {
                [
                    a.split_once('-')
                        .map(|(x, y)| [x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()])
                        .unwrap(),
                    b.split_once('-')
                        .map(|(x, y)| [x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()])
                        .unwrap(),
                ]
            })
            .filter(|[a, b]| {
                (a[0] == a[0].clamp(b[0], b[1]))
                    || (a[1] == a[1].clamp(b[0], b[1]))
                    || (b[0] == b[0].clamp(a[0], a[1]))
                    || (b[1] == b[1].clamp(a[0], a[1]))
            })
            .count()
            .into())
    }
}
