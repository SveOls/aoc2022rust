#![feature(error_in_core)]
#![feature(min_specialization)]
#![feature(iter_array_chunks)]
#![feature(array_chunks)]
#![feature(hash_extract_if)]
#![feature(isqrt)]
#![feature(array_try_map)]
#![feature(slice_flatten)]
#![feature(iterator_try_collect)]
#![feature(iter_intersperse)]
#![feature(option_take_if)]
#![feature(iter_map_windows)]
#![feature(array_windows)]
use core::{
    error::Error,
    fmt::{Display, Formatter},
    ops::Deref,
};
use std::time::Duration;

mod year2022;

fn main() -> Result<(), Box<dyn Error>> {
    Aoc::<2022>::run(false)?;
    Ok(())
}

#[derive(Debug)]
enum AocResult {
    Num(i64),
    Str(String),
    Wrong(String)
}

impl From<i64> for AocResult {
    fn from(value: i64) -> Self {
        Self::Num(value)
    }
}

impl From<usize> for AocResult {
    fn from(value: usize) -> Self {
        Self::Num(value as i64)
    }
}

impl From<String> for AocResult {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

impl Display for AocResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            AocResult::Num(a) => write!(f, "{a}"),
            AocResult::Str(a) => write!(f, "{a}"),
            AocResult::Wrong(a) => write!(f, "Failure: {a}"),
        }
    }
}

trait Run {
    fn run(_: bool) -> Result<Vec<(AocResult, std::time::Duration)>, AocError> {
        unreachable!()
    }
    fn parta(&self) -> Result<AocResult, AocError> {
        unreachable!()
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        unreachable!()
    }
}

#[derive(Debug)]
enum AocError {
    StdIo(std::io::Error),
    CoreNumParseInt(core::num::ParseIntError),
}

impl Display for AocError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AocError::StdIo(err) => write!(f, "{}", err),
            AocError::CoreNumParseInt(err) => write!(f, "{}", err),
        }
    }
}

impl Error for AocError {}

impl From<std::io::Error> for AocError {
    fn from(value: std::io::Error) -> Self {
        Self::StdIo(value)
    }
}

impl From<core::num::ParseIntError> for AocError {
    fn from(value: core::num::ParseIntError) -> Self {
        Self::CoreNumParseInt(value)
    }
}

/// Aoc::\<Y\>::run() iterates over all 25 days
///
/// as does Aoc::\<Y, 0\>::run()
///
/// unimplemented days (including any number above 25) yields unimplemented.
///
/// Aoc::\<Y, D\>::run() runs day D
///
/// implements deref -> &str, so can be used in place of String.
struct Aoc<const Y: u32, const D: u8 = 0>(String);

impl<const Y: u32, const D: u8> Run for Aoc<Y, D> {
    default fn run(print: bool) -> Result<Vec<(AocResult, std::time::Duration)>, AocError> {
        // let a = Self(
        //     std::fs::read_to_string(format!("input/{Y}/day{D}.txt"))
        //         .map_err(<Box<dyn Error>>::from)?,
        // );
        // println!("{Y}-{D}a: {}", a.parta()?);
        // println!("{Y}-{D}b: {}", a.partb()?);
        let a = Self(std::fs::read_to_string(format!("input/{Y}/day{D}.txt"))?);

        let now = std::time::Instant::now();
        let a_res = a.parta()?;
        let a_el = now.elapsed();
        println!("{Y}-{D}a: {}", a.parta()?);
        if print {
            println!("{:?}", a_el);
        }
        let b_res = a.partb()?;
        let b_el = now.elapsed() - a_el;
        println!("{Y}-{D}b: {}", a.partb()?);
        if print {
            println!("{:?}", b_el);
        }
        Ok(vec![
            (a_res, a_el),
            (b_res, b_el)
        ])
    }
    default fn parta(&self) -> Result<AocResult, AocError> {
        Ok(AocResult::Wrong(format!("tried to run {Y}-{D}-a")))
    }
    default fn partb(&self) -> Result<AocResult, AocError> {
        Ok(AocResult::Wrong(format!("tried to run {Y}-{D}-b")))
    }
}

impl<const Y: u32> Run for Aoc<Y, 0> {
    fn run(print: bool) -> Result<Vec<(AocResult, std::time::Duration)>, AocError> {
        let mut ret = Vec::new();
        ret.extend(Aoc::<Y, 1>::run(print)?);
        ret.extend(Aoc::<Y, 2>::run(print)?);
        ret.extend(Aoc::<Y, 3>::run(print)?);
        ret.extend(Aoc::<Y, 4>::run(print)?);
        ret.extend(Aoc::<Y, 5>::run(print)?);
        ret.extend(Aoc::<Y, 6>::run(print)?);
        ret.extend(Aoc::<Y, 7>::run(print)?);
        ret.extend(Aoc::<Y, 8>::run(print)?);
        ret.extend(Aoc::<Y, 9>::run(print)?);
        ret.extend(Aoc::<Y, 10>::run(print)?);
        ret.extend(Aoc::<Y, 11>::run(print)?);
        ret.extend(Aoc::<Y, 12>::run(print)?);
        ret.extend(Aoc::<Y, 13>::run(print)?);
        ret.extend(Aoc::<Y, 14>::run(print)?);
        ret.extend(Aoc::<Y, 15>::run(print)?);
        ret.extend(Aoc::<Y, 16>::run(print)?);
        ret.extend(Aoc::<Y, 17>::run(print)?);
        ret.extend(Aoc::<Y, 18>::run(print)?);
        ret.extend(Aoc::<Y, 19>::run(print)?);
        ret.extend(Aoc::<Y, 20>::run(print)?);
        ret.extend(Aoc::<Y, 21>::run(print)?);
        ret.extend(Aoc::<Y, 22>::run(print)?);
        ret.extend(Aoc::<Y, 23>::run(print)?);
        ret.extend(Aoc::<Y, 24>::run(print)?);
        ret.extend(Aoc::<Y, 25>::run(print)?);
        let sum: f64 = ret.iter().map(|x| x.1).sum::<Duration>().as_nanos() as f64;
        for (d, [i, j]) in ret.array_chunks::<2>().enumerate() {
            println!("{:>2}: {:>4.1}% ({:.5?}, {:.5?})", d + 1, 100. * ((i.1 + j.1).as_nanos() as f64 / sum), i.1, j.1)
        }
        println!("Total: {:?}", ret.iter().map(|x| x.1).sum::<Duration>());
        Ok(ret)
    }
}

impl<const Y: u32, const D: u8> Deref for Aoc<Y, D> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
