use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

use itertools::Itertools;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Valve {
    name: [char; 2],
    flow: u64,
    into: HashMap<[char; 2], usize>,
}

impl Valve {
    fn keyed(self) -> ([char; 2], Self) {
        (self.name, self)
    }
}

impl FromStr for Valve {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Valve ").unwrap();
        let (name, s) = s.split_once(' ').unwrap();
        let s = s.strip_prefix("has flow rate=").unwrap();
        let (flow, s) = s.split_once(';').unwrap();
        let s = s
            .strip_prefix(" tunnels lead to valves ")
            .or(s.strip_prefix(" tunnel leads to valve "))
            .unwrap();
        let into = s
            .split(", ")
            .map(|x| x.chars().array_chunks().next().map(|x| (x, 1)).unwrap())
            .collect();
        Ok(Self {
            name: name.chars().array_chunks().next().unwrap(),
            flow: flow.parse()?,
            into,
        })
    }
}

#[derive(PartialEq, Eq)]
struct Path(Vec<[char; 2]>, u64);

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

fn down_deep(
    position: [char; 2],
    mut data: HashMap<[char; 2], Valve>,
    depth: u64,
) -> BinaryHeap<Path> {
    let mut ret = BinaryHeap::new();
    ret.push(Path(Vec::new(), 0));
    if depth == 0 {
        return ret;
    }
    // dbg!(&data);
    // dbg!(&position);
    let flow = data.get(&position).unwrap().flow;
    if flow > 0 {
        let mut down = data.clone();
        down.get_mut(&position).unwrap().flow = 0;
        ret.extend(
            down_deep(position, down, depth - 1)
                .into_iter()
                .map(|x| (x.0, x.1 + (depth - 1) * flow))
                .map(|x| Path(x.0, x.1)),
        );
        // ret.0.push(position);
        // println!("opening {:?}", position);
        // if data.values().map(|x| x.flow).max().unwrap() == flow {
        return ret;
        // }
    }
    // dbg!(&data);
    // dbg!(position);
    let a = if flow == 0 {
        let a = data.remove(&position).unwrap();

        for i in data.values_mut() {
            i.into.remove(&position);
        }
        for (other, familiar) in a
            .into
            .iter()
            .flat_map(|x| std::iter::repeat(x).zip(a.into.iter()))
            .filter(|(x, y)| x != y)
        {
            // dbg!(other);
            // dbg!(familiar);
            data.entry(*other.0).and_modify(|x| {
                let b = x.into.entry(*familiar.0).or_insert(usize::MAX);
                *b = (other.1 + familiar.1).min(*b);
            });
        }
        a
    } else {
        data.get(&position).unwrap().clone()
    };
    // dbg!(data.len());
    for i in a.into.into_iter().filter(|x| x.1 <= depth as usize) {
        if !data.contains_key(&i.0) {
            continue;
        }
        // println!("moving from {:?} to {:?}", position, i.0);
        // let tester = down_deep(i.0, data.clone(), depth - i.1 as u64);
        ret.extend(
            down_deep(i.0, data.clone(), depth - i.1 as u64)
                .into_iter()
                .map(|x| (x.0.into_iter().chain(std::iter::once(i.0)).collect(), x.1))
                .map(|x| Path(x.0, x.1)),
        );
    }
    ret
}

impl Run for Aoc<2022, 16> {
    fn parta(&self) -> Result<AocResult, AocError> {
        let mut positions: HashMap<[char; 2], Valve> = self
            .lines()
            .map(Valve::from_str)
            .map(|x| x.map(Valve::keyed))
            .try_collect()?;
        while positions
            .values()
            .map(|x| x.into.len())
            .any(|x| x != positions.len() - 1)
        {
            // dbg!(&positions);
            for position in positions.clone().keys() {
                let a = positions.get(position).unwrap().clone();
                for (other, familiar) in a
                    .into
                    .iter()
                    .flat_map(|x| std::iter::repeat(x).zip(a.into.iter()))
                    .filter(|(x, y)| x != y)
                {
                    // dbg!(other);
                    // dbg!(familiar);
                    positions
                        .entry(*other.0)
                        .and_modify(|x| {
                            let b = x.into.entry(*familiar.0).or_insert(usize::MAX);
                            *b = (other.1 + familiar.1).min(*b);
                        })
                        .or_insert_with(|| panic!());
                }
            }
        }
        positions.retain(|k, v| v.flow > 0 || k == &['A'; 2]);
        // Ok(0i64.into())
        Ok((down_deep(['A'; 2], positions, 30).pop().unwrap().1 as usize).into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        let mut positions: HashMap<[char; 2], Valve> = self
            .lines()
            .map(Valve::from_str)
            .map(|x| x.map(Valve::keyed))
            .try_collect()?;
        while positions
            .values()
            .map(|x| x.into.len())
            .any(|x| x != positions.len() - 1)
        {
            // dbg!(&positions);
            for position in positions.clone().keys() {
                let a = positions.get(position).unwrap().clone();
                for (other, familiar) in a
                    .into
                    .iter()
                    .flat_map(|x| std::iter::repeat(x).zip(a.into.iter()))
                    .filter(|(x, y)| x != y)
                {
                    // dbg!(other);
                    // dbg!(familiar);
                    positions
                        .entry(*other.0)
                        .and_modify(|x| {
                            let b = x.into.entry(*familiar.0).or_insert(usize::MAX);
                            *b = (other.1 + familiar.1).min(*b);
                        })
                        .or_insert_with(|| panic!());
                }
            }
        }
        positions.retain(|k, v| v.flow > 0 || k == &['A'; 2]);
        let mut heap = down_deep(['A'; 2], positions, 26);
        let mut ret = 0;
        'outer: while let Some(Path(path, val)) = heap.pop() {
            for Path(path2, val2) in heap.iter().rev() {
                if !path.iter().any(|x| path2.contains(x)) {
                    ret = ret.max(val + val2);
                }
                if ret > 2 * val {
                    break 'outer;
                }
            }
            heap.retain(|x| x.1 + val >= ret);
        }
        Ok((ret as usize).into())
        // Ok((down_deep(['A'; 2], positions, 26, HashSet::new()).1 as usize).into())
    }
}

// 0 1 2 3 4
//
