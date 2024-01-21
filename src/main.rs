#![feature(error_in_core)]
#![feature(min_specialization)]
#![feature(iter_array_chunks)]
#![feature(hash_extract_if)]
#![feature(array_try_map)]
#![feature(slice_flatten)]
#![feature(iterator_try_collect)]
#![feature(iter_intersperse)]
#![feature(option_take_if)]
#![feature(iter_map_windows)]
#![feature(slice_first_last_chunk)]
#![feature(array_windows)]
use core::{
    error::Error,
    fmt::{Display, Formatter},
    ops::Deref,
};

mod year2022;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{:?}", Aoc::<2022>::run()?);
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
        }
    }
}

trait Run {
    fn run() -> Result<std::time::Duration, AocError> {
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
    default fn run() -> Result<std::time::Duration, AocError> {
        // let a = Self(
        //     std::fs::read_to_string(format!("input/{Y}/day{D}.txt"))
        //         .map_err(<Box<dyn Error>>::from)?,
        // );
        // println!("{Y}-{D}a: {}", a.parta()?);
        // println!("{Y}-{D}b: {}", a.partb()?);
        let a = Self(std::fs::read_to_string(format!("input/{Y}/day{D}.txt"))?);
        let now = std::time::Instant::now();
        println!("{Y}-{D}a: {}", a.parta()?);
        println!("{Y}-{D}b: {}", a.partb()?);
        println!("{:?}", now.elapsed());
        Ok(now.elapsed())
    }
    default fn parta(&self) -> Result<AocResult, AocError> {
        unimplemented!("tried to run {Y}-{D}-a")
    }
    default fn partb(&self) -> Result<AocResult, AocError> {
        unimplemented!("tried to run {Y}-{D}-a")
    }
}

impl<const Y: u32> Run for Aoc<Y, 0> {
    fn run() -> Result<std::time::Duration, AocError> {
        let mut ret = std::time::Duration::new(0, 0);
        ret += Aoc::<Y, 1>::run()?;
        ret += Aoc::<Y, 2>::run()?;
        ret += Aoc::<Y, 3>::run()?;
        ret += Aoc::<Y, 4>::run()?;
        ret += Aoc::<Y, 5>::run()?;
        ret += Aoc::<Y, 6>::run()?;
        ret += Aoc::<Y, 7>::run()?;
        ret += Aoc::<Y, 8>::run()?;
        ret += Aoc::<Y, 9>::run()?;
        ret += Aoc::<Y, 10>::run()?;
        ret += Aoc::<Y, 11>::run()?;
        ret += Aoc::<Y, 12>::run()?;
        ret += Aoc::<Y, 13>::run()?;
        ret += Aoc::<Y, 14>::run()?;
        // ret += Aoc::<Y, 15>::run()?;
        // ret += Aoc::<Y, 16>::run()?;
        ret += Aoc::<Y, 17>::run()?;
        ret += Aoc::<Y, 18>::run()?;
        // ret += Aoc::<Y, 19>::run()?;
        ret += Aoc::<Y, 20>::run()?;
        ret += Aoc::<Y, 21>::run()?;
        ret += Aoc::<Y, 22>::run()?;
        ret += Aoc::<Y, 23>::run()?;
        ret += Aoc::<Y, 24>::run()?;
        ret += Aoc::<Y, 25>::run()?;
        Ok(ret)
    }
}

impl<const Y: u32, const D: u8> Deref for Aoc<Y, D> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
