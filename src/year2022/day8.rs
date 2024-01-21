use super::*;

impl Run for Aoc<2022, 8> {
    fn parta(&self) -> Result<AocResult, AocError> {
        let a: [[u8; 99]; 99] = self
            .lines()
            .map(|x| {
                x.chars()
                    .map(|x| x as u8)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let mut ret = 0i64;
        for i in 0..99 {
            for j in 0..99 {
                if !a[i].get(0..j).unwrap().iter().rev().any(|&x| x >= a[i][j])
                    || !a[i].get(j + 1..99).unwrap().iter().any(|&x| x >= a[i][j])
                    || !a
                        .get(0..i)
                        .unwrap()
                        .iter()
                        .rev()
                        .map(|x| &x[j])
                        .any(|&x| x >= a[i][j])
                    || !a
                        .get(i + 1..99)
                        .unwrap()
                        .iter()
                        .map(|x| &x[j])
                        .any(|&x| x >= a[i][j])
                {
                    ret += 1;
                }
            }
        }
        Ok(ret.into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        let mut a: [[u8; 99]; 99] = self
            .lines()
            .map(|x| {
                x.chars()
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        for i in 0..99 {
            a[0][i] = 10;
            a[98][i] = 10;
            a[i][0] = 10;
            a[i][98] = 10;
        }
        Ok((1..98)
            .flat_map(|i| std::iter::repeat(i).zip(1..98))
            .map(|(i, j)| (a[i].split_at(j), a.split_at(i).0, &a.split_at(i).1[1..], j))
            .map(|((x, y), x2, x3, j)| (x, y.split_first().unwrap(), x2, x3, j))
            .map(|(x, (t, y), x2, x3, j)| {
                (x.iter().rev().take_while(|&x| x < t).count() + 1)
                    * (y.iter().take_while(|&x| x < t).count() + 1)
                    * (x2.iter().rev().map(|x| x[j]).take_while(|x| x < t).count() + 1)
                    * (x3.iter().map(|x| x[j]).take_while(|x| x < t).count() + 1)
            })
            .max()
            .unwrap()
            .into())
    }
}
