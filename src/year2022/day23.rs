use std::collections::{HashMap, HashSet};

use super::*;

struct Map {
    elves: HashSet<[i64; 2]>,
    orients: [u8; 4],
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut range = [[i64::MAX, i64::MIN]; 2];
        for e in &self.elves {
            range[0][0] = range[0][0].min(e[0]);
            range[0][1] = range[0][1].max(e[0]);
            range[1][0] = range[1][0].min(e[1]);
            range[1][1] = range[1][1].max(e[1]);
        }
        for i in range[0][0]..=range[0][1] {
            for j in range[1][0]..=range[1][1] {
                if self.elves.contains(&[i, j]) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Iterator for Map {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut moves: HashMap<[i64; 2], Vec<[i64; 2]>> = HashMap::new();
        for i in &self.elves {
            let n = !self.elves.contains(&[i[0] - 1, i[1]]);
            let nw = !self.elves.contains(&[i[0] - 1, i[1] - 1]);
            let ne = !self.elves.contains(&[i[0] - 1, i[1] + 1]);
            let s = !self.elves.contains(&[i[0] + 1, i[1]]);
            let sw = !self.elves.contains(&[i[0] + 1, i[1] - 1]);
            let se = !self.elves.contains(&[i[0] + 1, i[1] + 1]);
            let w = !self.elves.contains(&[i[0], i[1] - 1]);
            let e = !self.elves.contains(&[i[0], i[1] + 1]);
            if n && nw && ne && w && e && s && sw && se {
                continue;
            }
            for o in self.orients {
                match o {
                    0 => {
                        if n && nw && ne {
                            moves.entry([i[0] - 1, i[1]]).or_default().push(*i);
                            break;
                        }
                    }
                    1 => {
                        if se && ne && e {
                            moves.entry([i[0], i[1] + 1]).or_default().push(*i);
                            break;
                        }
                    }
                    2 => {
                        if s && sw && se {
                            moves.entry([i[0] + 1, i[1]]).or_default().push(*i);
                            break;
                        }
                    }
                    3 => {
                        if sw && nw && w {
                            moves.entry([i[0], i[1] - 1]).or_default().push(*i);
                            break;
                        }
                    }
                    _ => panic!(),
                }
            }
        }
        let mut range = [[i64::MAX, i64::MIN]; 2];
        let mut mov = false;
        for (k, v) in moves.drain() {
            if v.len() == 1 {
                self.elves.remove(&v[0]);
                self.elves.insert(k);
                mov = true;
            }
        }
        for e in &self.elves {
            range[0][0] = range[0][0].min(e[0]);
            range[0][1] = range[0][1].max(e[0]);
            range[1][0] = range[1][0].min(e[1]);
            range[1][1] = range[1][1].max(e[1]);
        }
        self.orients.rotate_left(1);
        mov.then_some(
            (range[0][1] - range[0][0] + 1) * (range[1][1] - range[1][0] + 1)
                - self.elves.len() as i64,
        )
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Self {
            orients: [0, 2, 3, 1],
            elves: value
                .lines()
                .enumerate()
                .flat_map(|(i, x)| {
                    x.chars()
                        .enumerate()
                        .filter(|(_, y)| y == &'#')
                        .map(move |(j, _)| [i as i64, j as i64])
                })
                .collect(),
        }
    }
}

impl Run for Aoc<2022, 23> {
    fn parta(&self) -> Result<AocResult, AocError> {
        //         let test = "....#..
        // ..###.#
        // #...#.#
        // .#...##
        // #.###..
        // ##.#.##
        // .#..#..";
        let mut a = Map::from(self.to_string().as_str());
        // println!("{}", a);
        // for i in 0..10 {
        //     println!("{}", a.next().unwrap());
        //     println!("{}", a);
        // }
        Ok(a.nth(9).unwrap().into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        //         let test = "....#..
        // ..###.#
        // #...#.#
        // .#...##
        // #.###..
        // ##.#.##
        // .#..#..";
        let a = Map::from(self.to_string().as_str());
        // println!("{}", a);
        // for i in 0..10 {
        //     println!("{}", a.next().unwrap());
        //     println!("{}", a);
        // }
        Ok((a.count() + 1).into())
    }
}
