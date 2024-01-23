use super::*;

struct Snafu(i64);


impl From<&str> for Snafu {
    fn from(value: &str) -> Self {
        let mut ret = 0;
        for (i, c) in value.chars().rev().enumerate() {
            match c {
                '=' => ret -= 2 * 5i64.pow(i as u32),
                '-' => ret -= 1 * 5i64.pow(i as u32),
                '0' => {},
                '1' => ret += 1 * 5i64.pow(i as u32),
                '2' => ret += 2 * 5i64.pow(i as u32),
                _ => panic!()
            }
        }
        Self(ret)
    }
}

impl Into<String> for Snafu {
    fn into(self) -> String {
        let mut ret = String::new();
        let mut temp = self.0;
        while temp != 0 {
            let rem = temp % 5;
            let tepest = match rem {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => {
                    temp += 5;
                    "="
                }
                4 => {
                    temp += 5;
                    "-"
                }
                _ => unreachable!()
            };
            ret = tepest.to_string() + ret.clone().as_str();
            temp -= rem;
            temp /= 5;
        }
        ret
    }
}

impl Run for Aoc<2022, 25> {
    fn parta(&self) -> Result<AocResult, AocError> {
        Ok(<Snafu as Into<String>>::into(Snafu(self.lines().map(Snafu::from).map(|x| x.0).sum())).into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        Ok("Finished!!".to_owned().into())
    }
}
