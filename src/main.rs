#![feature(min_specialization)]
use std::{error::Error, fmt::Display, ops::Deref};

mod year2022;

fn main() -> Result<(), Box<dyn Error>> {
    Aoc::<2022>::run()?;
    Ok(())
}

enum AocResult {
    Num(i64),
    Str(String),
}

impl From<i64> for AocResult {
    fn from(value: i64) -> Self {
        Self::Num(value)
    }
}

impl From<String> for AocResult {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

impl Display for AocResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocResult::Num(a) => write!(f, "{a}"),
            AocResult::Str(a) => write!(f, "{a}"),
        }
    }
}

trait Run {
    fn run() -> Result<(), Box<dyn Error>> {
        unreachable!()
    }
    fn parta(&self) -> Result<AocResult, Box<dyn Error>> {
        unreachable!()
    }
    fn partb(&self) -> Result<AocResult, Box<dyn Error>> {
        unreachable!()
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
    default fn run() -> Result<(), Box<dyn Error>> {
        let a = Self(
            std::fs::read_to_string(format!("input/{Y}/day{D}.txt"))
                .map_err(<Box<dyn Error>>::from)?,
        );
        println!("{Y}-{D}a: {}", a.parta()?);
        println!("{Y}-{D}b: {}", a.partb()?);
        Ok(())
    }
    default fn parta(&self) -> Result<AocResult, Box<dyn Error>> {
        unimplemented!("tried to run {Y}-{D}-a")
    }
    default fn partb(&self) -> Result<AocResult, Box<dyn Error>> {
        unimplemented!("tried to run {Y}-{D}-a")
    }
}

impl<const Y: u32> Run for Aoc<Y, 0> {
    fn run() -> Result<(), Box<dyn Error>> {
        Aoc::<Y, 1>::run()?;
        Aoc::<Y, 2>::run()?;
        Aoc::<Y, 3>::run()?;
        Aoc::<Y, 4>::run()?;
        Aoc::<Y, 5>::run()?;
        Aoc::<Y, 6>::run()?;
        Aoc::<Y, 7>::run()?;
        Aoc::<Y, 8>::run()?;
        Aoc::<Y, 9>::run()?;
        Aoc::<Y, 10>::run()?;
        Aoc::<Y, 11>::run()?;
        Aoc::<Y, 12>::run()?;
        Aoc::<Y, 13>::run()?;
        Aoc::<Y, 14>::run()?;
        Aoc::<Y, 15>::run()?;
        Aoc::<Y, 16>::run()?;
        Aoc::<Y, 17>::run()?;
        Aoc::<Y, 18>::run()?;
        Aoc::<Y, 19>::run()?;
        Aoc::<Y, 20>::run()?;
        Aoc::<Y, 21>::run()?;
        Aoc::<Y, 22>::run()?;
        Aoc::<Y, 23>::run()?;
        Aoc::<Y, 24>::run()?;
        Aoc::<Y, 25>::run()?;
        Ok(())
    }
}

impl<const Y: u32, const D: u8> Deref for Aoc<Y, D> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
