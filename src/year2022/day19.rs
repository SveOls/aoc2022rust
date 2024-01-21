use std::rc::Rc;

use super::*;

#[derive(Debug, Clone)]
struct Blueprint {
    num: i64,
    ore: [i64; 3],
    clay: [i64; 3],
    obsidian: [i64; 3],
    geode: [i64; 3],
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        let value = value.strip_prefix("Blueprint ").unwrap();
        // dbg!(value);
        let mut value = value
            .split(|x: char| !x.is_ascii_digit())
            .filter(|x| !x.is_empty());
        let num = value.next().unwrap().parse().unwrap();
        let ore = [value.next().unwrap().parse().unwrap(), 0, 0];
        let clay = [value.next().unwrap().parse().unwrap(), 0, 0];
        let obsidian = [
            value.next().unwrap().parse().unwrap(),
            value.next().unwrap().parse().unwrap(),
            0,
        ];
        let geode = [
            value.next().unwrap().parse().unwrap(),
            0,
            value.next().unwrap().parse().unwrap(),
        ];
        Self {
            num,
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    time: usize,
    blueprint: Rc<Blueprint>,
    ore: [i64; 2],
    clay: [i64; 2],
    obsidian: [i64; 2],
    geode: [i64; 2],
    save_for: Option<u8>,
}

impl From<Blueprint> for Game {
    fn from(value: Blueprint) -> Self {
        Self {
            time: 0,
            blueprint: Rc::new(value),
            ore: [1, 0],
            clay: [0, 0],
            obsidian: [0, 0],
            geode: [0, 0],
            save_for: None,
        }
    }
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.ore[1] += self.ore[0];
        self.clay[1] += self.clay[0];
        self.obsidian[1] += self.obsidian[0];
        self.geode[1] += self.geode[0];
        self.time += 1;
        Some(self.time)
    }
}

impl Game {
    fn set_next(mut self, inp: Option<u8>) -> Self {
        self.save_for = inp;
        self
    }
    fn next_don(mut self, depth: usize, memo: i64) -> i64 {
        if self.save_for.is_none() {
            let mut ret = 0;
            for i in (0..=3).rev() {
                ret = ret.max(
                    self.clone()
                        .set_next(Some(i))
                        .next_don(depth, ret.max(memo)),
                );
            }
            return ret;
        }
        let mut test = self.clone();
        while test.next().is_some_and(|x| x < depth) {
            if test.blueprint.geode[2] <= test.obsidian[1] {
                // print!("g");
                test.obsidian[1] -= test.blueprint.geode[2];
                test.geode[0] += 1;
            } else if test.blueprint.obsidian[1] <= test.clay[1] {
                // print!("o");
                test.clay[1] -= test.blueprint.obsidian[1];
                test.obsidian[0] += 1;
            } else if test.blueprint.clay[0] <= test.ore[1] {
                // print!("c");
                test.ore[1] -= test.blueprint.ore[0];
                test.clay[0] += 1;
            } else {
                test.ore[0] += 1;
            }
        }
        // dbg!(&memo);
        if memo > test.geode[1] {
            // dbg!(memo);
            return 0;
        }
        while self.next().is_some_and(|x| x < depth) {
            let material = [self.ore[1], self.clay[1], self.obsidian[1]];
            let cost = match self.save_for.unwrap() {
                0 => self.blueprint.ore,
                1 => self.blueprint.clay,
                2 => self.blueprint.obsidian,
                3 => self.blueprint.geode,
                _ => panic!(),
            };
            if cost
                .into_iter()
                .zip(material.into_iter())
                .all(|(x, y)| x <= y)
            {
                self.ore[1] -= cost[0];
                self.clay[1] -= cost[1];
                self.obsidian[1] -= cost[2];
                match self.save_for.take().unwrap() {
                    0 => {
                        self.ore[0] += 1;
                        self.ore[1] -= 1
                    }
                    1 => {
                        self.clay[0] += 1;
                        self.clay[1] -= 1
                    }
                    2 => {
                        self.obsidian[0] += 1;
                        self.obsidian[1] -= 1
                    }
                    3 => {
                        self.geode[0] += 1;
                        self.geode[1] -= 1
                    }
                    _ => panic!(),
                }
                return self.set_next(None).next_don(depth, memo);
                // *memo = ret.max(*memo);
                // return ret;
            }
        }
        self.geode[1]
    }
}
// 25 20 15 10  5  0  1  3  6  4  9  8  7
//  g  g  g  g  g  o  o  o  g  o  g  g  g -> 66
// 25 26 22 18 14 10  6  2  4  7  5  8  7
//  o  g  g  g  g  g  g  o  o  g  g  o  g -> 65
// 25 20 21 17 13  9  5  1  3  6  4  8  7
//  g  o  g  g  g  g  g  o  o  g  o  g  g -> 65
//
// 5  1  3  6  4  8 (7)
// g  o  o  g  o  g (10)
// 5  7  4  7  5  3 (7)
// o  g  o  g  g  o (9)
//
// 5  2  5  3  7  6 (5)
// g  o  g  o  g  g (12)
// 5  8  6  4  8  7 (6)
// o  g  g  o  g  g (11)
//
// if afford(geode): build geode
// else if afford(obsidian): build obsidian
// else: build(clay)
//

impl Run for Aoc<2022, 19> {
    fn parta(&self) -> Result<AocResult, AocError> {
        Ok(self
            .lines()
            .map(Blueprint::from)
            .map(Game::from)
            .map(|x| x.blueprint.num * x.next_don(24, 0))
            .sum::<i64>()
            .into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        Ok(self
            .lines()
            .take(3)
            .map(Blueprint::from)
            .map(Game::from)
            .map(|x| x.next_don(32, 0))
            .product::<i64>()
            .into())
    }
}
