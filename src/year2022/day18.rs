use std::collections::HashSet;

use super::*;

impl Run for Aoc<2022, 18> {
    fn parta(&self) -> Result<AocResult, AocError> {
        let data: Vec<[usize; 3]> = self
            .lines()
            .flat_map(|x| {
                x.split(',')
                    .array_chunks::<3>()
                    .map(|z| z.try_map(|y| y.parse::<usize>()))
            })
            .try_collect()?;
        Ok(data
            .iter()
            .enumerate()
            .map(|x| {
                6 - 2 * std::iter::repeat(x.1)
                    .zip(data.iter().skip(x.0 + 1))
                    .filter(|(x, y)| {
                        x.iter()
                            .zip(y.iter())
                            .map(|z| z.0.abs_diff(*z.1))
                            .sum::<usize>()
                            == 1
                    })
                    .count()
            })
            .sum::<usize>()
            .into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        let data: HashSet<[usize; 3]> = self
            .lines()
            .flat_map(|x| {
                x.split(',')
                    .array_chunks::<3>()
                    .map(|z| z.try_map(|y| y.parse::<usize>().map(|x| x + 1)))
            })
            .try_collect()?;
        let xrange = data.iter().map(|x| x[0]).fold([usize::MAX, 0], |acc, x| {
            [acc[0].min(x - 1), acc[1].max(x + 1)]
        });
        let yrange = data.iter().map(|x| x[1]).fold([usize::MAX, 0], |acc, x| {
            [acc[0].min(x - 1), acc[1].max(x + 1)]
        });
        let zrange = data.iter().map(|x| x[2]).fold([usize::MAX, 0], |acc, x| {
            [acc[0].min(x - 1), acc[1].max(x + 1)]
        });
        let position = [0; 3];
        let mut checked: HashSet<[usize; 3]> = HashSet::new();
        Ok(recur(position, &mut checked, &data, [xrange, yrange, zrange]).into())
    }
}

fn recur(
    position: [usize; 3],
    visited: &mut HashSet<[usize; 3]>,
    data: &HashSet<[usize; 3]>,
    range: [[usize; 2]; 3],
) -> i64 {
    if data.contains(&position) {
        1
    } else if !visited.insert(position) {
        0
    } else {
        let mut ret = 0;
        for i in 0..3 {
            let mut newpos = position;
            newpos[i] = position[i].saturating_sub(1).max(range[i][0]);
            ret += recur(newpos, visited, data, range);
            newpos[i] = (position[i] + 1).min(range[i][1]);
            ret += recur(newpos, visited, data, range);
        }
        ret
    }
}
