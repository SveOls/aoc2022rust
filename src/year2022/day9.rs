use std::collections::HashSet;

use super::*;

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    Easth,
    Westh,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'D' => Direction::South,
            'U' => Direction::North,
            'L' => Direction::Westh,
            'R' => Direction::Easth,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Heading(Direction, u8);

#[derive(Debug)]
struct Rope<const N: usize> {
    tail: [[i64; 2]; N],
    visited: HashSet<[i64; 2]>,
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Self {
            tail: [[0; 2]; N],
            visited: HashSet::new(),
        }
    }
    fn translate(mut self, rhs: Heading) -> Self {
        for _ in 0..rhs.1 {
            match rhs.0 {
                Direction::North => self.tail[0][0] -= 1,
                Direction::South => self.tail[0][0] += 1,
                Direction::Easth => self.tail[0][1] += 1,
                Direction::Westh => self.tail[0][1] -= 1,
            }
            for i in 1..N {
                match (
                    self.tail[i - 1][0].abs_diff(self.tail[i][0]),
                    self.tail[i - 1][1].abs_diff(self.tail[i][1]),
                ) {
                    (2, 0) => {
                        self.tail[i][0] += (self.tail[i - 1][0] - self.tail[i][0]).signum();
                    }
                    (0, 2) => {
                        self.tail[i][1] += (self.tail[i - 1][1] - self.tail[i][1]).signum();
                    }
                    (a, b) if a + b > 2 => {
                        self.tail[i][0] += (self.tail[i - 1][0] - self.tail[i][0]).signum();
                        self.tail[i][1] += (self.tail[i - 1][1] - self.tail[i][1]).signum();
                    }
                    (1, 0) | (0, 1) | (0, 0) | (1, 1) => {}
                    a => panic!("{a:?}"),
                }
            }
            self.visited.insert(*self.tail.last().unwrap());
        }
        self
    }
}

impl From<&str> for Heading {
    fn from(value: &str) -> Self {
        value
            .split_once(char::is_whitespace)
            .map(|(x, y)| Heading(x.chars().next().unwrap().into(), y.parse().unwrap()))
            .unwrap()
    }
}

impl Run for Aoc<2022, 9> {
    fn parta(&self) -> Result<AocResult, AocError> {
        Ok(self
            .lines()
            .map(Heading::from)
            .fold(Rope::<2>::new(), |acc, x| acc.translate(x))
            .visited
            .len()
            .into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        Ok(self
            .lines()
            .map(Heading::from)
            .fold(Rope::<10>::new(), |acc, x| acc.translate(x))
            .visited
            .len()
            .into())
    }
}
