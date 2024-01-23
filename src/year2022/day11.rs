use std::collections::VecDeque;

use super::*;

enum Operation {
    Mul(i64),
    Add(i64),
    Square,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match (
            value.split_whitespace().last().unwrap().parse(),
            value.split_whitespace().rev().nth(1).unwrap(),
        ) {
            (Ok(a), "*") => Operation::Mul(a),
            (Ok(a), "+") => Operation::Add(a),
            (Err(_), "*") => Operation::Square,
            _ => panic!(),
        }
    }
}

impl Operation {
    fn edit(&self, inp: i64) -> i64 {
        match self {
            Operation::Mul(a) => inp * a,
            Operation::Add(a) => inp + a,
            Operation::Square => inp.pow(2),
        }
    }
}

struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    test: (i64, [usize; 2]),
}

impl Monkey {
    fn inspect(&mut self) -> Option<(i64, usize)> {
        let mut ret = self.items.pop_front()?;
        ret = self.operation.edit(ret);
        ret /= 3;
        if ret % self.test.0 == 0 {
            Some((ret, self.test.1[0]))
        } else {
            Some((ret, self.test.1[1]))
        }
    }
    fn inspect_b(&mut self) -> Option<(i64, usize)> {
        let mut ret = self.items.pop_front()?;
        ret = self.operation.edit(ret);
        if ret % self.test.0 == 0 {
            Some((ret, self.test.1[0]))
        } else {
            Some((ret, self.test.1[1]))
        }
    }
    fn catch(&mut self, inp: i64) {
        self.items.push_back(inp);
    }
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let mut lines = value.lines().skip(1);
        let items = lines
            .next()
            .unwrap()
            .split(|c: char| !c.is_ascii_digit())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();
        let operation = Operation::from(lines.next().unwrap().trim());
        let test_v = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let test_true = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let test_false = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        Self {
            items,
            operation,
            test: (test_v, [test_true, test_false]),
        }
    }
}

impl Run for Aoc<2022, 11> {
    fn parta(&self) -> Result<AocResult, AocError> {
        let mut monkeys: Vec<Monkey> = self.split("\n\n").map(Monkey::from).collect();
        let mut ret = vec![0; monkeys.len()];
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                while let Some((popp, i_0)) = monkeys[i].inspect() {
                    ret[i] += 1i64;
                    monkeys[i_0].catch(popp);
                }
            }
        }
        ret.sort();
        ret.reverse();
        Ok((ret[0] * ret[1]).into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        let mut monkeys: Vec<Monkey> = self.split("\n\n").map(Monkey::from).collect();
        let lcm = monkeys
            .iter()
            .map(|x| x.test.0)
            .reduce(num::integer::lcm)
            .unwrap();
        let mut ret = vec![0; monkeys.len()];
        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                while let Some((popp, i_0)) = monkeys[i].inspect_b() {
                    ret[i] += 1i64;
                    monkeys[i_0].catch(popp % lcm);
                }
            }
        }
        ret.sort();
        ret.reverse();
        Ok((ret[0] * ret[1]).into())
    }
}
