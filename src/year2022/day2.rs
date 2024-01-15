use super::*;
use std::error::Error;

enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn fight(self, rhs: Self) -> i64 {
        use Play::*;
        match (self, rhs) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 0,
            (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => 6,
        }
    }
    fn fighte(self, rhs: char) -> i64 {
        use Play::*;
        match (self, rhs) {
            (Rock, 'Z') => 8,
            (Paper, 'Z') => 9,
            (Scissors, 'Z') => 7,
            (Rock, 'Y') => 4,
            (Paper, 'Y') => 5,
            (Scissors, 'Y') => 6,
            (Rock, 'X') => 3,
            (Paper, 'X') => 1,
            (Scissors, 'X') => 2,
            _ => panic!(),
        }
    }
}

impl From<Play> for i64 {
    fn from(value: Play) -> Self {
        match value {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}

impl From<char> for Play {
    fn from(value: char) -> Self {
        use Play::*;
        match value {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!(),
        }
    }
}

impl Run for Aoc<2022, 2> {
    fn parta(&self) -> Result<AocResult, Box<dyn Error>> {
        Ok(self
            .lines()
            .map(|x| {
                x.split_once(' ')
                    .map(|y| {
                        i64::from(Play::from(y.1.chars().next().unwrap()))
                            + Play::from(y.1.chars().next().unwrap())
                                .fight(y.0.chars().next().unwrap().into())
                    })
                    .unwrap()
            })
            .sum::<i64>()
            .into())
    }
    fn partb(&self) -> Result<AocResult, Box<dyn Error>> {
        Ok(self
            .lines()
            .map(|x| {
                x.split_once(' ')
                    .map(|y| {
                        Play::from(y.0.chars().next().unwrap()).fighte(y.1.chars().next().unwrap())
                    })
                    .unwrap()
            })
            .sum::<i64>()
            .into())
    }
}
