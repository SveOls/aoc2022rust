use super::*;

enum Instr {
    Addx(i64),
    Noop,
}

impl Instr {
    fn time(&self) -> usize {
        match self {
            Instr::Addx(_) => 2,
            Instr::Noop => 1,
        }
    }
    fn value(&self) -> i64 {
        match self {
            Instr::Addx(a) => *a,
            Instr::Noop => 0,
        }
    }
}

impl From<&str> for Instr {
    fn from(value: &str) -> Self {
        match value.split_once(' ') {
            None => Self::Noop,
            Some(("addx", a)) => Self::Addx(a.parse().unwrap()),
            _ => panic!(),
        }
    }
}

impl Run for Aoc<2022, 10> {
    fn parta(&self) -> Result<AocResult, AocError> {
        Ok(self
            .lines()
            .map(Instr::from)
            .scan((1, 0), |state, x| {
                state.0 += x.value();
                state.1 += x.time();
                Some(*state)
            })
            .map_windows(|[x, y]| (x.0, x.1, y.1))
            .filter(|(_, low, high)| (low + 20) % 40 > (high + 20) % 40)
            .fold(0i64, |acc, (x, i, _)| acc + x * ((i + 19) / 20) as i64 * 20)
            .into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        Ok(self
            .lines()
            .map(Instr::from)
            .fold((String::new(), 39i64, 0i64), |mut acc, x| {
                for _ in 0..x.time() {
                    acc.1 += 1;
                    if acc.1 == 40 {
                        acc.0 += "\n";
                        acc.1 = 0;
                    }
                    if (acc.1 - 1).abs_diff(acc.2) < 2 {
                        acc.0 += "█"
                    } else {
                        acc.0 += " "
                    }
                }
                acc.2 += x.value();
                acc
            })
            .0
            .into())
    }
}
