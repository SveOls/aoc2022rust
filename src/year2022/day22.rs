use std::collections::HashSet;

use super::*;

struct PuzzleBlock {
    position: [usize; 2],
    data: Vec<Vec<char>>,
}

impl PuzzleBlock {
    fn sanitize(mut self, size: usize) -> Vec<Self> {
        let mut ret = Vec::new();
        // dbg!(self.data.len());
        // dbg!(self.data[0].len());
        for i in (0..self.data.len() / size).rev() {
            // dbg!(i);
            let chunk = self.data.split_off(size * i);
            // dbg!(chunk.len());
            // dbg!(chunk[0].len());
            for j in 0..chunk[0].len() / size {
                ret.push(Self {
                    position: [self.position[0] + i * size, self.position[1] + j * size],
                    data: chunk
                        .iter()
                        .map(|x| x[j * size..j * size + size].to_vec())
                        .collect(),
                });
            }
        }
        // for i in &ret {
        //     dbg!(i.position);
        //     dbg!(i.data.len());
        //     dbg!(i.data[0].len());
        // }
        ret
    }
    fn contains(&self, pos: [usize; 2]) -> Option<bool> {
        // dbg!(pos);
        // dbg!(self.position);
        if pos[0] < self.position[0] || pos[1] < self.position[1] {
            // dbg!("1");
            None
        } else if (pos[0] - self.position[0]) >= self.data.len()
            || (pos[1] - self.position[1]) >= self.data[0].len()
        {
            // dbg!("2");
            None
        } else {
            // dbg!("3");
            Some(self.data[pos[0] - self.position[0]][pos[1] - self.position[1]] == '.')
        }
    }
}

impl From<(usize, &str)> for PuzzleBlock {
    fn from((i, value): (usize, &str)) -> Self {
        let data: Vec<_>;
        let j: Vec<_>;
        (j, data) = value.chars().partition(|&x| x == ' ');
        Self {
            position: [i + 1, j.len() + 1],
            data: vec![data],
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Nort,
    East,
    Sout,
    West,
}

impl Direction {
    fn rotate(&mut self, inp: char) {
        use Direction::*;
        match (&self, inp) {
            (Nort, 'L') | (Sout, 'R') => *self = West,
            (Nort, 'R') | (Sout, 'L') => *self = East,
            (West, 'L') | (East, 'R') => *self = Sout,
            (West, 'R') | (East, 'L') => *self = Nort,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Person {
    position: [usize; 2],
    visited: HashSet<[usize; 2]>,
    orientation: Direction,
    block_index: usize,
}

impl Person {
    fn translate_b(
        &mut self,
        inp: Option<usize>,
        data: &[PuzzleBlock],
        connections: &[[(usize, bool); 4]; 6],
    ) {
        let Some(inp) = inp else {
            panic!();
        };
        self.visited.clear();
        self.visited.insert(self.position);
        // dbg!(self.position);
        for _ in 0..inp {
            let old_pos = self.position;
            match self.orientation {
                Direction::Nort => self.position[0] -= 1,
                Direction::East => self.position[1] += 1,
                Direction::Sout => self.position[0] += 1,
                Direction::West => self.position[1] -= 1,
            }
            if let Some((i, a)) = data
                .iter()
                .enumerate()
                .find_map(|(i, x)| x.contains(self.position).map(|y| (i, y)))
            {
                if !a {
                    self.position = old_pos;
                    // dbg!("found, false");
                    break;
                } else {
                    // dbg!("found, true");
                    self.block_index = i;
                    self.visited.insert(self.position);
                }
            } else {
                let old_dir = self.orientation;
                self.position = old_pos;
                let (into, flip) = connections[self.block_index][old_dir as usize];
                let index = match old_dir {
                    Direction::Nort | Direction::Sout => {
                        self.position[1] - data[self.block_index].position[1]
                    }
                    // Direction::East => data[self.block_index].data.len() - 1 - (self.position[0] - data[self.block_index].position[0]),
                    // Direction::Sout => data[self.block_index].data.len() - 1 - (self.position[1] - data[self.block_index].position[1]),
                    Direction::West | Direction::East => {
                        self.position[0] - data[self.block_index].position[0]
                    }
                };
                let index = if flip {
                    data[0].data.len() - 1 - index
                } else {
                    index
                };
                let from = connections[into]
                    .iter()
                    .position(|&x| x.0 == self.block_index)
                    .unwrap();
                match from {
                    0 => {
                        self.orientation = Direction::Sout;
                        self.position = [data[into].position[0], data[into].position[1] + index]
                    }
                    1 => {
                        self.orientation = Direction::West;
                        self.position = [
                            data[into].position[0] + index,
                            data[into].position[1] + data[into].data.len() - 1,
                        ]
                    }
                    2 => {
                        self.orientation = Direction::Nort;
                        self.position = [
                            data[into].position[0] + data[into].data.len() - 1,
                            data[into].position[1] + index,
                        ]
                    }
                    3 => {
                        self.orientation = Direction::East;
                        self.position = [data[into].position[0] + index, data[into].position[1]]
                    }
                    _ => panic!(),
                }
                if let Some(false) = data.iter().find_map(|x| x.contains(self.position)) {
                    self.position = old_pos;
                    self.orientation = old_dir;
                    // dbg!("missing, false");
                    break;
                } else {
                    self.block_index = into;
                    // dbg!("missing, true");
                    self.visited.insert(self.position);
                }
            }
            // dbg!(self.position);
        }
    }
    fn translate(&mut self, inp: Option<usize>, data: &[PuzzleBlock]) {
        let Some(inp) = inp else {
            let old_pos = self.position;
            let mut last = true;
            while let Some(a) = data.iter().find_map(|x| x.contains(self.position)) {
                last = a;
                match self.orientation {
                    Direction::Nort => self.position[0] += 1,
                    Direction::East => self.position[1] -= 1,
                    Direction::Sout => self.position[0] -= 1,
                    Direction::West => self.position[1] += 1,
                }
            }
            match self.orientation {
                Direction::Nort => self.position[0] -= 1,
                Direction::East => self.position[1] += 1,
                Direction::Sout => self.position[0] += 1,
                Direction::West => self.position[1] -= 1,
            }
            if !last {
                self.position = old_pos
            }
            return;
        };
        self.visited.clear();
        self.visited.insert(self.position);
        // dbg!(self.position);
        for _ in 0..inp {
            let old_pos = self.position;
            match self.orientation {
                Direction::Nort => self.position[0] -= 1,
                Direction::East => self.position[1] += 1,
                Direction::Sout => self.position[0] += 1,
                Direction::West => self.position[1] -= 1,
            }
            if let Some(a) = data.iter().find_map(|x| x.contains(self.position)) {
                if !a {
                    self.position = old_pos;
                    // dbg!("found, false");
                    break;
                } else {
                    // dbg!("found, true");

                    self.visited.insert(self.position);
                }
            } else {
                self.position = old_pos;
                self.translate(None, data);
                if self.position == old_pos {
                    // dbg!("missing, false");
                    break;
                } else {
                    // dbg!("missing, true");
                    self.visited.insert(self.position);
                }
            }
            // dbg!(self.position);
        }
    }
    fn rotate(&mut self, inp: char) {
        self.orientation.rotate(inp)
    }
    fn new(position: [usize; 2]) -> Self {
        Self {
            position,
            visited: HashSet::new(),
            orientation: Direction::East,
            block_index: 0,
        }
    }
}

impl Run for Aoc<2022, 22> {
    fn parta(&self) -> Result<AocResult, AocError> {
        //         let test = "        ...#
        //         .#..
        //         #...
        //         ....
        // ...#.......#
        // ........#...
        // ..#....#....
        // ..........#.
        //         ...#....
        //         .....#..
        //         .#......
        //         ......#.

        // 10R5L5R10L4R5L5";
        let mut taker = self.lines();
        let mut data: Vec<_> = taker
            .by_ref()
            .take_while(|x| !x.is_empty())
            .enumerate()
            .map(PuzzleBlock::from)
            .collect();
        let mut i = 1;
        while i < data.len() {
            if data[i].position[0] == data[i - 1].position[0] + data[i - 1].data.len()
                && data[i].position[1] == data[i - 1].position[1]
                && data[i].data[0].len() == data[i - 1].data[0].len()
            {
                let stuff = data.remove(i).data.remove(0);
                data[i - 1].data.push(stuff);
            } else {
                i += 1;
            }
        }
        let mut person = Person::new([
            data[0].position[0],
            data[0].position[1] + data[0].data[0].iter().position(|x| x == &'.').unwrap(),
        ]);
        let mut ret = 0;
        for instruction in taker
            .by_ref()
            .flat_map(|x| x.split_inclusive(|c: char| c.is_ascii_alphabetic()))
        {
            if instruction.chars().all(|x| x.is_ascii_digit()) {
                person.translate(Some(instruction.parse()?), &data);
            } else {
                person.translate(
                    Some(
                        instruction
                            .strip_suffix(|c: char| c.is_alphabetic())
                            .unwrap()
                            .parse()?,
                    ),
                    &data,
                );
                person.rotate(instruction.chars().last().unwrap());
            }

            ret = 1000 * person.position[0]
                + 4 * person.position[1]
                + match person.orientation {
                    Direction::Nort => 3,
                    Direction::East => 0,
                    Direction::Sout => 1,
                    Direction::West => 2,
                };
        }
        Ok(ret.into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        //         let test = "        ...#
        //         .#..
        //         #...
        //         ....
        // ...#.......#
        // ........#...
        // ..#....#....
        // ..........#.
        //         ...#....
        //         .....#..
        //         .#......
        //         ......#.

        // 10R5L5R10L4R5L5";
        // let connections = [
        //         [(1, true), (5, true), (3, false), (2, false)],
        //         [(0, true), (2, false), (4, true), (5, true)],
        //         [(0, false), (1, false), (4, true), (3, false)],
        //         [(0, false), (5, true), (4, false), (2, false)],
        //         [(3, false), (5, false), (1, true), (2, true)],
        //         [(3, true), (0, true), (1, true), (4, false)],
        // ];
        let mut taker = self.lines();
        let mut data: Vec<_> = taker
            .by_ref()
            .take_while(|x| !x.is_empty())
            .enumerate()
            .map(PuzzleBlock::from)
            .collect();
        let mut i = 1;
        while i < data.len() {
            if data[i].position[0] == data[i - 1].position[0] + data[i - 1].data.len()
                && data[i].position[1] == data[i - 1].position[1]
                && data[i].data[0].len() == data[i - 1].data[0].len()
            {
                let stuff = data.remove(i).data.remove(0);
                data[i - 1].data.push(stuff);
            } else {
                i += 1;
            }
        }

        let size: usize = (data
            .iter()
            .map(|x| x.data.len() * x.data[0].len())
            .sum::<usize>()
            / 6)
        .isqrt();
        // dbg!(size);
        let data: Vec<_> = data
            .into_iter()
            .flat_map(|x| x.sanitize(size).into_iter())
            .collect();

        // hardcoded connections
        let connections = [
            [(5, false), (1, false), (2, false), (3, true)],
            [(5, false), (4, true), (2, false), (0, false)],
            [(0, false), (1, false), (4, false), (3, false)],
            [(2, false), (4, false), (5, false), (0, true)],
            [(2, false), (1, true), (5, false), (3, false)],
            [(3, false), (4, false), (1, false), (0, false)],
        ];

        let mut person = Person::new([
            data[0].position[0],
            data[0].position[1] + data[0].data[0].iter().position(|x| x == &'.').unwrap(),
        ]);
        let mut ret = 0;
        for instruction in taker
            .by_ref()
            .flat_map(|x| x.split_inclusive(|c: char| c.is_ascii_alphabetic()))
        {
            if instruction.chars().all(|x| x.is_ascii_digit()) {
                person.translate_b(Some(instruction.parse()?), &data, &connections);
            } else {
                person.translate_b(
                    Some(
                        instruction
                            .strip_suffix(|c: char| c.is_alphabetic())
                            .unwrap()
                            .parse()?,
                    ),
                    &data,
                    &connections,
                );
                person.rotate(instruction.chars().last().unwrap());
            }
            ret = 1000 * person.position[0]
                + 4 * person.position[1]
                + match person.orientation {
                    Direction::Nort => 3,
                    Direction::East => 0,
                    Direction::Sout => 1,
                    Direction::West => 2,
                };
        }
        Ok(ret.into())
    }
}
