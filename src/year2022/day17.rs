use std::collections::HashMap;

use super::*;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Block {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Block {
    fn posis(&self) -> Vec<[usize; 2]> {
        match self {
            Block::Horizontal => vec![[0, 0], [0, 1], [0, 2], [0, 3]],
            Block::Cross => vec![[1, 0], [0, 1], [1, 1], [2, 1], [1, 2]],
            Block::Corner => vec![[0, 0], [0, 1], [0, 2], [1, 2], [2, 2]],
            Block::Vertical => vec![[0, 0], [1, 0], [2, 0], [3, 0]],
            Block::Square => vec![[0, 0], [0, 1], [1, 0], [1, 1]],
        }
    }
    fn dims(&self) -> [usize; 2] {
        match self {
            Block::Horizontal => [1, 4],
            Block::Cross => [3, 3],
            Block::Corner => [3, 3],
            Block::Vertical => [4, 1],
            Block::Square => [2, 2],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct BlockPos<const N: usize> {
    kind: Block,
    position: [usize; 2],
    prev_pos: [usize; 2],
}

impl<const N: usize> BlockPos<N> {
    fn contains(&self, pos: [usize; 2]) -> bool {
        self.kind
            .posis()
            .into_iter()
            .map(|x| [x[0] + self.position[0], x[1] + self.position[1]])
            .any(|x| x == pos)
    }
    fn collision(&self, other: &Self) -> bool {
        let a: Vec<_> = self
            .kind
            .posis()
            .into_iter()
            .map(|x| [x[0] + self.position[0], x[1] + self.position[1]])
            .collect();
        other
            .kind
            .posis()
            .into_iter()
            .map(|x| [x[0] + other.position[0], x[1] + other.position[1]])
            .any(|x| a.contains(&x))
    }
    fn undo(&mut self) {
        self.position = self.prev_pos;
    }
    fn translate(&mut self, direction: Direction) {
        self.prev_pos = self.position;
        match direction {
            Direction::East => {
                self.position[1] = (self.position[1] + 1).min(N - self.kind.dims()[1])
            }
            Direction::South => self.position[0] -= 1,
            Direction::West => self.position[1] = self.position[1].saturating_sub(1),
            Direction::North => panic!(),
        }
    }
    fn occupied_max(&self) -> usize {
        self.position[0] + self.kind.dims()[0]
    }
}

fn run<const N: usize>(inp: &Aoc<2022, 17>, goal: usize) -> Result<AocResult, AocError> {
    // let test = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    // true = right
    let data = inp
        .lines()
        .flat_map(|x| x.chars())
        .map(|x| x == '>')
        .collect::<Vec<bool>>();
    let mut next_wind = {
        let mut i = 0;
        move |inp: &mut BlockPos<N>| -> usize {
            if data[i] {
                inp.translate(Direction::East)
            } else {
                inp.translate(Direction::West)
            }
            i += 1;
            i %= data.len();
            i
        }
    };
    let blocks = [
        Block::Horizontal,
        Block::Cross,
        Block::Corner,
        Block::Vertical,
        Block::Square,
    ];
    let mut next_block = {
        let mut i = 0;
        move |max: usize| -> BlockPos<N> {
            i %= blocks.len();
            i += 1;
            BlockPos {
                kind: blocks[(i - 1) % blocks.len()],
                position: [3 + max, 2],
                prev_pos: [0, 0],
            }
        }
    };
    let mut previous_blocks: Vec<BlockPos<N>> = Vec::new();
    let mut max = 0;
    let mut potentials: HashMap<(Block, Vec<[usize; 2]>, usize), (usize, usize)> = HashMap::new();
    let mut i = 0;
    while i < goal {
        i += 1;
        let mut next_block = next_block(max);
        let block = next_block.kind;
        let windind = loop {
            let wind = next_wind(&mut next_block);
            if previous_blocks
                .iter()
                .rev()
                .any(|x| x.collision(&next_block))
            {
                next_block.undo();
            }
            if next_block.position[0] == 0 {
                max = max.max(next_block.occupied_max());
                previous_blocks.push(next_block);
                break wind;
            }
            next_block.translate(Direction::South);
            if previous_blocks
                .iter()
                .rev()
                .any(|x| x.collision(&next_block))
            {
                next_block.undo();
                max = max.max(next_block.occupied_max());
                previous_blocks.push(next_block);
                break wind;
            }
        };
        if let Some(start) = previous_blocks
            .iter()
            .filter(|x| x.position[1] == 0)
            .map(|x| {
                x.position[0]
                    + x.kind
                        .posis()
                        .iter()
                        .filter(|x| x[1] == 0)
                        .map(|x| x[0])
                        .max()
                        .unwrap()
            })
            .max()
        {
            let mut pos = [start + 1, 0];
            let mut last_move = Direction::East;
            let mut path = Vec::new();
            // dbg!(&previous_blocks);
            while pos[0] > 0 {
                let up = !previous_blocks
                    .iter()
                    .any(|x| x.contains([pos[0] + 1, pos[1]]));
                let down = !previous_blocks
                    .iter()
                    .any(|x| x.contains([pos[0] - 1, pos[1]]));
                let right = !previous_blocks
                    .iter()
                    .any(|x| x.contains([pos[0], pos[1] + 1]));
                let left = !previous_blocks
                    .iter()
                    .any(|x| x.contains([pos[0], pos[1] - 1]));
                // dbg!(pos);
                match last_move {
                    Direction::North => {
                        if right {
                            pos[1] += 1;
                            last_move = Direction::East;
                        } else if up {
                            pos[0] += 1;
                            last_move = Direction::North;
                        } else if left {
                            pos[1] -= 1;
                            last_move = Direction::West;
                        } else {
                            pos[0] -= 1;
                            last_move = Direction::South;
                        }
                    }
                    Direction::East => {
                        if down {
                            pos[0] -= 1;
                            last_move = Direction::South;
                        } else if right {
                            pos[1] += 1;
                            last_move = Direction::East;
                        } else if up {
                            pos[0] += 1;
                            last_move = Direction::North;
                        } else {
                            pos[1] -= 1;
                            last_move = Direction::West;
                        }
                    }
                    Direction::South => {
                        if left {
                            pos[1] -= 1;
                            last_move = Direction::West;
                        } else if down {
                            pos[0] -= 1;
                            last_move = Direction::South;
                        } else if right {
                            pos[1] += 1;
                            last_move = Direction::East;
                        } else {
                            pos[0] += 1;
                            last_move = Direction::North;
                        }
                    }
                    Direction::West => {
                        if up {
                            pos[0] += 1;
                            last_move = Direction::North;
                        } else if left {
                            pos[1] -= 1;
                            last_move = Direction::West;
                        } else if down {
                            pos[0] -= 1;
                            last_move = Direction::South;
                        } else {
                            pos[1] += 1;
                            last_move = Direction::East;
                        }
                    }
                }
                if pos[1] == N {
                    let min_y = path.iter().map(|x: &[usize; 2]| x[0]).min().unwrap();
                    previous_blocks.retain(|x| x.occupied_max() + 5 >= min_y);
                    path.iter_mut().for_each(|x| x[0] -= min_y);
                    if let Some((old_i, old_max)) =
                        potentials.insert((block, path, windind), (i, max))
                    {
                        let year_jump = i - old_i;
                        let height_diff = max - old_max;
                        let skips = (goal - i) / year_jump;
                        i += skips * year_jump;
                        max += skips * height_diff;
                        previous_blocks
                            .iter_mut()
                            .for_each(|x| x.position[0] += skips * height_diff);
                    }
                    break;
                }
                path.push(pos);
            }
        }
    }
    Ok(max.into())
}

impl Run for Aoc<2022, 17> {
    fn parta(&self) -> Result<AocResult, AocError> {
        run::<7>(self, 2022)
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        run::<7>(self, 1000000000000)
    }
}
