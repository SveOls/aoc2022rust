#![feature(min_specialization)]
use std::error::Error;

mod year2022;

fn main() -> Result<(), Box<dyn Error>> {
    Aoc::<2022, 0>::run()?;
    Ok(())
}

trait Run {
    fn run() -> Result<(), Box<dyn Error>> {
        unreachable!()
    }
    fn parta(&self) -> Result<String, Box<dyn Error>> {
        unimplemented!()
    }
    fn partb(&self) -> Result<String, Box<dyn Error>> {
        unimplemented!()
    }
}

struct Aoc<const Y: u32, const D: u8>(String);

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
    default fn parta(&self) -> Result<String, Box<dyn Error>> {
        unimplemented!()
    }
    default fn partb(&self) -> Result<String, Box<dyn Error>> {
        unimplemented!()
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
