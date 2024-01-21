use super::*;

impl Run for Aoc<2022, 20> {
    fn parta(&self) -> Result<AocResult, AocError> {
        let mut data: Vec<(usize, i64)> = self
            .lines()
            .enumerate()
            .map(|(i, x)| x.parse::<i64>().map(|y| (i, y)))
            .try_collect()?;
        let len = data.len() as i64;
        for i in 0..data.len() {
            let pos = data.iter().position(|x| x.0 == i).unwrap();
            let next_more = (data[pos].1 + pos as i64).rem_euclid(len - 1) as usize;
            if pos < next_more {
                data[pos..=next_more].rotate_left(1);
            } else {
                data[next_more..=pos].rotate_right(1);
            }
        }
        let zero_ind = data.iter().position(|x| x.1 == 0).unwrap();
        let mut ret = 0;
        for i in (1000..=3000).step_by(1000) {
            ret += data[(zero_ind + i) % data.len()].1;
        }

        Ok(ret.into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        let key = 811589153;
        let mut data: Vec<(usize, i64)> = self
            .lines()
            .enumerate()
            .map(|(i, x)| x.parse::<i64>().map(|y| (i, y * key)))
            .try_collect()?;
        let len = data.len() as i64;
        for i in (0..10).flat_map(|_| 0..len as usize) {
            let pos = data.iter().position(|x| x.0 == i).unwrap();
            let next_more = (data[pos].1 + pos as i64).rem_euclid(len - 1) as usize;
            if pos < next_more {
                data[pos..=next_more].rotate_left(1);
            } else {
                data[next_more..=pos].rotate_right(1);
            }
        }
        let zero_ind = data.iter().position(|x| x.1 == 0).unwrap();
        let mut ret = 0;
        for i in (1000..=3000).step_by(1000) {
            ret += data[(zero_ind + i) % data.len()].1;
        }

        Ok(ret.into())
    }
}
