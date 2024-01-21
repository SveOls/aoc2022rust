use super::*;
use core::num::ParseIntError;
use core::str::FromStr;
use std::collections::HashSet;

#[derive(Debug)]
struct Sensor {
    position: [i64; 2],
    beacon: [i64; 2],
}

impl Sensor {
    fn range(&self) -> i64 {
        (self.position[0].abs_diff(self.beacon[0]) + self.position[1].abs_diff(self.beacon[1]))
            as i64
    }
    fn illegal_x_at(&self, y: i64, between: Option<[i64; 2]>) -> Option<[i64; 2]> {
        let between = between.unwrap_or([i64::MIN, i64::MAX]);
        if self.range() - y.abs_diff(self.position[1]) as i64 >= 0 {
            let ret = [
                (self.position[0] + y.abs_diff(self.position[1]) as i64 - self.range())
                    .max(between[0]),
                (self.position[0] - y.abs_diff(self.position[1]) as i64 + self.range())
                    .min(between[1]),
            ];
            if ret[0] > ret[1] {
                None
            } else {
                Some(ret)
            }
        } else {
            None
        }
    }
}

impl FromStr for Sensor {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut a = s
            .split(|x: char| !x.is_ascii_digit() && x != '-')
            .filter(|x| !x.is_empty())
            .array_chunks::<2>();
        Ok(Self {
            position: a
                .next()
                .map(|x| x[0].parse().and_then(|x0| x[1].parse().map(|x1| [x0, x1])))
                .unwrap()?,
            beacon: a
                .next()
                .map(|x| x[0].parse().and_then(|x0| x[1].parse().map(|x1| [x0, x1])))
                .unwrap()?,
        })
    }
}

impl Run for Aoc<2022, 15> {
    fn parta(&self) -> Result<AocResult, AocError> {
        let data = self.lines().map(Sensor::from_str).try_collect::<Vec<_>>()?;
        let yv = 2000000;
        let ret: Vec<[i64; 2]> = data
            .iter()
            .filter_map(|x| x.illegal_x_at(yv, None))
            .collect();
        let ret = {
            let mut acc = ret;
            acc.sort_by_key(|x| x[0]);

            // if acc.len() < 2 {
            //     return acc;
            // }

            let mut result: Vec<[i64; 2]> = vec![];

            let mut acce = acc[0];
            for i in 1..acc.len() {
                let curr = acc[i];

                if acce[1] >= (curr[0] - 1) {
                    acce[1] = i64::max(acce[1], curr[1]); // Extend acc
                } else {
                    result.push(acce);
                    acce = curr;
                }
            }

            result.push(acce);

            result
        };
        // dbg!(a);
        Ok((ret.iter().map(|x| x[1] - x[0] + 1).sum::<i64>() as usize
            - data
                .iter()
                .filter(|x| x.beacon[1] == yv)
                .map(|x| x.beacon)
                .collect::<HashSet<_>>()
                .len()
            - data
                .iter()
                .filter(|x| x.position[1] == yv)
                .map(|x| x.position)
                .collect::<HashSet<_>>()
                .len())
        .into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        let data = self.lines().map(Sensor::from_str).try_collect::<Vec<_>>()?;
        let yv = 4000000;
        let xv = 4000000;
        for y in 0..yv {
            let ret: Vec<[i64; 2]> = data
                .iter()
                .filter_map(|x| x.illegal_x_at(y, Some([0, xv])))
                .collect();
            let mut acc = ret;
            acc.sort_by_key(|x| x[0]);
            // if acc.len() < 2 {
            //     return acc;
            // }

            let mut result: Vec<[i64; 2]> = vec![];

            let mut acce = acc[0];
            for i in 1..acc.len() {
                let curr = acc[i];

                if acce[1] >= (curr[0] - 1) {
                    acce[1] = i64::max(acce[1], curr[1]); // Extend acc
                } else {
                    result.push(acce);
                    acce = curr;
                }
            }

            result.push(acce);
            if result.iter().map(|x| x[1] - x[0] + 1).sum::<i64>() <= xv {
                return Ok((y + xv * (result[1][0] - 1)).into());
            }
        }
        unimplemented!()
    }
}
