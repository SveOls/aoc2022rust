use super::*;
use std::error::Error;

#[derive(Default)]
struct File {
    name: [u8; 8],
    above: Option<Box<File>>,
    below: Box<Vec<File>>,
    value: i64,
}

impl File {
    fn finalize(mut self) -> Self {
        self.value += self.below.iter().map(|x| x.value).sum::<i64>();
        self
    }
    fn minvalabove(&self, above: i64) -> Option<i64> {
        std::iter::once(self.value)
            .filter(|&x| x >= above)
            .chain(self.below.iter().filter_map(|x| x.minvalabove(above)))
            .min()
    }
    fn sumbelow(&self, above: i64) -> i64 {
        std::iter::once(self.value)
            .filter(|&x| x <= above)
            .chain(self.below.iter().map(|x| x.sumbelow(above)))
            .sum()
    }
}

impl Run for Aoc<2022, 7> {
    fn parta(&self) -> Result<AocResult, Box<dyn Error>> {
        Ok(self
            .lines()
            .skip(1)
            .chain(std::iter::once("$ cd .."))
            .fold(File::default(), |mut acc, x| {
                match x.as_bytes() {
                    [b'$', b' ', b'c', b'd', b' ', b'.', b'.'] => {
                        acc = acc.finalize();
                        let mut old_acc = acc;
                        let mut parent = *old_acc.above.take().unwrap();
                        parent.below.push(old_acc);
                        acc = parent;
                    }
                    [b'$', b' ', b'c', b'd', b' ', a @ ..] => {
                        let name: [u8; 8] = std::array::from_fn(|i| *a.get(i).unwrap_or(&0));
                        let mut old_acc = acc;
                        let mut child = old_acc.below.swap_remove(
                            old_acc.below.iter().position(|x| x.name == name).unwrap(),
                        );

                        child.above = Some(Box::new(old_acc));
                        acc = child;
                    }
                    [b'$', b' ', b'l', b's'] => {}
                    [b'd', b'i', b'r', b' ', b @ ..] => acc.below.push(File {
                        name: std::array::from_fn(|i| *b.get(i).unwrap_or(&0)),
                        ..Default::default()
                    }),
                    [c @ ..] => {
                        acc.value += String::from_utf8_lossy(c)
                            .split_once(|c| c == ' ')
                            .unwrap()
                            .0
                            .parse::<i64>()
                            .unwrap()
                    }
                }
                acc
            })
            .sumbelow(100000)
            .into())
    }
    fn partb(&self) -> Result<AocResult, Box<dyn Error>> {
        let a = self
            .lines()
            .skip(1)
            .chain(std::iter::once("$ cd .."))
            .fold(File::default(), |mut acc, x| {
                match x.as_bytes() {
                    [b'$', b' ', b'c', b'd', b' ', b'.', b'.'] => {
                        acc.value += acc.below.iter().map(|x| x.value).sum::<i64>();
                        let mut old_acc = acc;
                        let mut parent = *old_acc.above.take().unwrap();
                        parent.below.push(old_acc);
                        acc = parent;
                    }
                    [b'$', b' ', b'c', b'd', b' ', a @ ..] => {
                        let name: [u8; 8] = std::array::from_fn(|i| *a.get(i).unwrap_or(&0));
                        let mut old_acc = acc;
                        let mut child = old_acc.below.swap_remove(
                            old_acc.below.iter().position(|x| x.name == name).unwrap(),
                        );

                        child.above = Some(Box::new(old_acc));
                        acc = child;
                    }
                    [b'$', b' ', b'l', b's'] => {}
                    [b'd', b'i', b'r', b' ', b @ ..] => acc.below.push(File {
                        name: std::array::from_fn(|i| *b.get(i).unwrap_or(&0)),
                        ..Default::default()
                    }),
                    [c @ ..] => {
                        acc.value += String::from_utf8_lossy(c)
                            .split_once(|c| c == ' ')
                            .unwrap()
                            .0
                            .parse::<i64>()
                            .unwrap()
                    }
                }
                acc
            })
            .finalize();
        Ok(a.minvalabove(30000000 - (70000000 - a.value))
            .unwrap()
            .into())
    }
}
