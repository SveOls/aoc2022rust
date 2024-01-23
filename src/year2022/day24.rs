use core::panic;
use std::collections::{HashSet, VecDeque};

use super::*;
use num::{self, Integer};

#[derive(Debug)]
struct Maze {
    dimensions: [usize; 2],
    blizzards: HashSet<[usize; 3]>,
}

impl Maze {
    fn resolve(&self, partb: bool) -> usize {
        let start_pos = [0, 1];
        let mut data = VecDeque::new();
        let mut memo = HashSet::new();
        let goals = [[self.dimensions[0] - 2, self.dimensions[1] - 2], [1, 1]];
        // let mut direction = 0;
        data.push_back((start_pos, 0, 0));
        while let Some((a, direction, i)) = data.pop_front() {
            if self.is_clear(a, i).iter().any(|&x| x) {
                continue;
            } else if a == goals[direction % 2] {
                let mut next = goals[direction % 2];
                if direction % 2 == 0 {
                    next[0] += 1;
                } else {
                    next[0] -= 1;
                }
                data.push_back((next, direction + 1, i + 1));
                if !(partb && direction < 2) {
                    return i + 1;
                }
            } else if memo.insert((
                a,
                direction,
                i % self.dimensions[0].lcm(&self.dimensions[1]),
            )) {
                data.push_back((a, direction, i + 1));
                if a[0] < self.dimensions[0] - 2 && a[1] == a[1].clamp(1, self.dimensions[1] - 2) {
                    data.push_back(([a[0] + 1, a[1]], direction, i + 1));
                }
                if a[1] < self.dimensions[1] - 2 && a[0] == a[0].clamp(1, self.dimensions[0] - 2) {
                    data.push_back(([a[0], a[1] + 1], direction, i + 1));
                }
                if a[0] > 1 && a[1] == a[1].clamp(1, self.dimensions[1] - 2) {
                    data.push_back(([a[0] - 1, a[1]], direction, i + 1));
                }
                if a[1] > 1 && a[0] == a[0].clamp(1, self.dimensions[0] - 2) {
                    data.push_back(([a[0], a[1] - 1], direction, i + 1));
                }
            }
        }
        panic!()
    }
    fn is_clear(&self, [y, x]: [usize; 2], time: usize) -> [bool; 4] {
        let y_time = time % (self.dimensions[0] - 2);
        let x_time = time % (self.dimensions[1] - 2);
        [
            self.blizzards
                .contains(&[1 + (y - 1 + y_time) % (self.dimensions[0] - 2), x, 0]),
            self.blizzards.contains(&[
                y,
                1 + (x - 1 + self.dimensions[1] - 2 - x_time) % (self.dimensions[1] - 2),
                1,
            ]),
            self.blizzards.contains(&[
                1 + (y - 1 + self.dimensions[0] - 2 - y_time) % (self.dimensions[0] - 2),
                x,
                2,
            ]),
            self.blizzards
                .contains(&[y, 1 + (x - 1 + x_time) % (self.dimensions[1] - 2), 3]),
        ]
    }
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let y = value.lines().count();
        let x = value.lines().next().unwrap().chars().count();
        let mut blizzards = HashSet::new();
        for (i, y) in value.lines().enumerate() {
            for (j, x) in y.chars().enumerate() {
                match x {
                    '^' => {
                        blizzards.insert([i, j, 0]);
                    }
                    '>' => {
                        blizzards.insert([i, j, 1]);
                    }
                    'v' => {
                        blizzards.insert([i, j, 2]);
                    }
                    '<' => {
                        blizzards.insert([i, j, 3]);
                    }
                    _ => {}
                }
            }
        }
        Self {
            blizzards,
            dimensions: [y, x],
        }
    }
}

impl Run for Aoc<2022, 24> {
    fn parta(&self) -> Result<AocResult, AocError> {
        //         let test = "#.######
        // #>>.<^<#
        // #.<..<<#
        // #>v.><>#
        // #<^v^^>#
        // ######.#";
        let map = Maze::from(self.to_string().as_str());
        // println!("{}", test);
        Ok(map.resolve(false).into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        //         let test = "#.######
        // #>>.<^<#
        // #.<..<<#
        // #>v.><>#
        // #<^v^^>#
        // ######.#";
        let map = Maze::from(self.to_string().as_str());
        // println!("{}", test);
        Ok(map.resolve(true).into())
    }
}
