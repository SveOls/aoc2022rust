use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use super::*;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Wall {
    rocks: Vec<[i64; 2]>,
}

impl Wall {
    fn contains(&self, pos: [i64; 2]) -> bool {
        self.rocks.array_windows::<2>().any(|[x, y]| {
            pos[0] == pos[0].clamp(x[0].min(y[0]), x[0].max(y[0]))
                && pos[1] == pos[1].clamp(x[1].min(y[1]), x[1].max(y[1]))
        })
    }
}

impl FromStr for Wall {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rocks: s
                .split(" -> ")
                .map(|x| {
                    x.split_once(",")
                        .map(|x| x.0.parse().and_then(|x0| x.1.parse().map(|x1| [x0, x1])))
                        .unwrap()
                })
                .try_collect::<Vec<_>>()?,
        })
    }
}

impl Run for Aoc<2022, 14> {
    fn parta(&self) -> Result<AocResult, AocError> {
        let walls: HashSet<_> = self.lines().map(Wall::from_str).try_collect()?;
        let lowest_point = walls
            .iter()
            .flat_map(|x| x.rocks.iter().map(|x| x[1]))
            .max()
            .unwrap();
        let mut visited = HashSet::<[i64; 2]>::new();
        let mut position = [500, 0];
        let mut return_to = Vec::new();
        while position[1] < lowest_point {
            if !visited.contains(&[position[0], position[1] + 1])
                && !walls
                    .iter()
                    .any(|x| x.contains([position[0], position[1] + 1]))
            {
                return_to.push(position);
                position[1] += 1;
                continue;
            }
            if !visited.contains(&[position[0] - 1, position[1] + 1])
                && !walls
                    .iter()
                    .any(|x| x.contains([position[0] - 1, position[1] + 1]))
            {
                return_to.push(position);
                position[1] += 1;
                position[0] -= 1;
                continue;
            }
            if !visited.contains(&[position[0] + 1, position[1] + 1])
                && !walls
                    .iter()
                    .any(|x| x.contains([position[0] + 1, position[1] + 1]))
            {
                return_to.push(position);
                position[1] += 1;
                position[0] += 1;
                continue;
            }
            visited.insert(position);
            position = return_to.pop().unwrap();
        }
        Ok(visited.len().into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        let walls: HashSet<_> = self.lines().map(Wall::from_str).try_collect()?;
        let lowest_point = walls
            .iter()
            .flat_map(|x| x.rocks.iter().map(|x| x[1]))
            .max()
            .unwrap();
        let mut visited = HashSet::<[i64; 2]>::new();
        for i in -lowest_point - 5..=lowest_point + 5 {
            visited.insert([500 + i, lowest_point + 2]);
        }
        let mut position = [500, 0];
        let mut return_to = Vec::new();
        while !visited.contains(&[500, 0]) {
            if !visited.contains(&[position[0], position[1] + 1])
                && !walls
                    .iter()
                    .any(|x| x.contains([position[0], position[1] + 1]))
            {
                return_to.push(position);
                position[1] += 1;
                continue;
            }
            if !visited.contains(&[position[0] - 1, position[1] + 1])
                && !walls
                    .iter()
                    .any(|x| x.contains([position[0] - 1, position[1] + 1]))
            {
                return_to.push(position);
                position[1] += 1;
                position[0] -= 1;
                continue;
            }
            if !visited.contains(&[position[0] + 1, position[1] + 1])
                && !walls
                    .iter()
                    .any(|x| x.contains([position[0] + 1, position[1] + 1]))
            {
                return_to.push(position);
                position[1] += 1;
                position[0] += 1;
                continue;
            }
            visited.insert(position);
            position = return_to.pop().unwrap_or(position);
        }
        let mut a = visited
            .iter()
            .filter(|x| x[1] < lowest_point + 2)
            .copied()
            .collect::<Vec<_>>();
        a.sort();
        // dbg!(&a);
        Ok(a.len().into())
    }
}
