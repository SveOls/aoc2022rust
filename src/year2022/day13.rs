use super::*;

#[derive(Clone, PartialEq, Eq)]
enum Val {
    Single(i64),
    List(Vec<Val>),
}

impl std::fmt::Debug for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(arg0) => write!(f, "{arg0}"),
            Self::List(arg0) => {
                write!(f, "[")?;
                for i in arg0
                    .iter()
                    .map(|x| format!("{x:?}"))
                    .intersperse(", ".to_string())
                {
                    write!(f, "{}", i)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl Into<String> for Val {
    fn into(self) -> String {
        let mut ret = String::new();
        match self {
            Val::Single(a) => ret += a.to_string().as_str(),
            Val::List(b) => {
                ret += "[";
                for i in 0..b.len() {
                    ret += <Val as Into<String>>::into(b[i].clone()).as_str();
                    if i < b.len() - 1 {
                        ret += ","
                    }
                }
                ret += "]";
            }
        }
        ret
    }
}

impl TryFrom<&str> for Val {
    type Error = core::num::ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let ret = if value.is_empty() {
            panic!()
        } else if value.chars().all(|x| x.is_ascii_digit()) {
            value.parse::<i64>().map(Val::Single)
        } else if let Some(val) = value.strip_prefix('[').and_then(|x| x.strip_suffix(']')) {
            let a = val
                .char_indices()
                .scan(0, |state, x| match x.1 {
                    '[' => {
                        *state += 1;
                        Some(None)
                    }
                    ']' => {
                        *state -= 1;
                        Some(None)
                    }
                    ',' if *state == 0 => Some(Some(x.0)),
                    _ => Some(None),
                })
                .filter_map(|x| x)
                .chain(core::iter::once(val.len()));
            let mut prev = 0;
            let mut ret = Vec::new();
            for i in a {
                if i == prev {
                    continue;
                }
                ret.push(val[prev..i].try_into()?);
                prev = i + 1;
            }
            Ok(Val::List(ret))
        } else {
            panic!()
        };
        ret
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        use Val::*;
        let ret = match (self, other) {
            (Single(a), Single(b)) => a.cmp(b),
            (Single(a), List(b)) => vec![Single(*a)].cmp(&b),
            (List(a), Single(b)) => a.cmp(&vec![Single(*b)]),
            (List(a), List(b)) => a.cmp(b),
        };
        ret
    }
}

impl Run for Aoc<2022, 13> {
    fn parta(&self) -> Result<AocResult, AocError> {
        let mut ret = 0usize;
        for (i, [x, y]) in self
            .lines()
            .filter(|x| !x.is_empty())
            .map(Val::try_from)
            .array_chunks::<2>()
            .enumerate()
        {
            match x?.partial_cmp(&y?).unwrap() {
                core::cmp::Ordering::Equal => panic!(),
                core::cmp::Ordering::Less => ret += i + 1,
                core::cmp::Ordering::Greater => {}
            }
        }
        Ok(ret.into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        let markers = ["[[2]]", "[[6]]"];
        let mut ol = self
            .lines()
            .chain(markers)
            .filter(|x| !x.is_empty())
            .map(Val::try_from)
            .try_collect::<Vec<_>>()?;
        ol.sort();
        Ok(markers
            .into_iter()
            .map(Val::try_from)
            .map(|x| x.unwrap())
            .map(|x| ol.iter().position(|y| *y == x).unwrap() + 1)
            .product::<usize>()
            .into())
    }
}
