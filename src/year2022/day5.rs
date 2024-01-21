use super::*;

impl Run for Aoc<2022, 5> {
    fn parta(&self) -> Result<AocResult, AocError> {
        Ok(self
            .split_once("\n\n")
            .map(|(x, y)| x.lines().rev().flat_map(x_int).chain(y.lines().map(y_int)))
            .unwrap()
            .fold([b' '; 66], folder_a)
            .split(|&x| x == b' ')
            .filter_map(|x| x.iter().last())
            .map(|&x| x as char)
            .collect::<String>()
            .into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        Ok(self
            .split_once("\n\n")
            .map(|(x, y)| x.lines().rev().flat_map(x_int).chain(y.lines().map(y_int)))
            .unwrap()
            .fold([b' '; 66], folder_b)
            .split(|&x| x == 32)
            .filter_map(|x| x.iter().last())
            .map(|&x| x as char)
            .collect::<String>()
            .into())
    }
}

fn folder_b(mut acc: [u8; 66], x: [u8; 4]) -> [u8; 66] {
    let start_ind = acc
        .iter()
        .enumerate()
        .filter(|(_, &i)| i == b' ')
        .nth(x[2] as usize - 1)
        .unwrap()
        .0;
    let end_ind = acc
        .iter()
        .enumerate()
        .filter(|(_, &i)| i == b' ')
        .nth(x[3] as usize - 1)
        .unwrap()
        .0;
    acc[acc
        .iter()
        .enumerate()
        .filter(|(_, &i)| i == b' ')
        .nth(9)
        .unwrap()
        .0] = x[0];
    if let Some(a) = acc
        .get_mut(start_ind - x[1] as usize..end_ind)
        .filter(|a| a.len() > 0)
    {
        a.rotate_left(x[1] as usize);
    }
    if let Some(a) = acc.get_mut(end_ind..start_ind).filter(|a| a.len() > 0) {
        a.rotate_right(x[1] as usize);
    }
    acc
}

fn folder_a(mut acc: [u8; 66], x: [u8; 4]) -> [u8; 66] {
    let start_ind = acc
        .iter()
        .enumerate()
        .filter(|(_, &i)| i == b' ')
        .nth(x[2] as usize - 1)
        .unwrap()
        .0;
    let end_ind = acc
        .iter()
        .enumerate()
        .filter(|(_, &i)| i == b' ')
        .nth(x[3] as usize - 1)
        .unwrap()
        .0;
    acc[acc
        .iter()
        .enumerate()
        .filter(|(_, &i)| i == b' ')
        .nth(9)
        .unwrap()
        .0] = x[0];
    for i in 0..x[1] as usize {
        if let Some(a) = acc
            .get_mut(start_ind - i - 1..end_ind)
            .filter(|a| a.len() > 0)
        {
            a.rotate_left(1);
            // break;
        }
        if let Some(a) = acc.get_mut(end_ind + i..start_ind).filter(|a| a.len() > 0) {
            a.rotate_right(1);
        }
    }
    acc
}
// 0 0 0 0 1 2 3 4 5 81
fn x_int(inp: &str) -> impl Iterator<Item = [u8; 4]> + '_ {
    inp.as_bytes()
        .iter()
        .skip(1)
        .step_by(4)
        .enumerate()
        .filter(|(_, &x)| x != b' ')
        .map(|(i, &x)| [x, 1, 11, i as u8 + 1])
        .filter(|x| !(x[0] as char).is_ascii_digit())
}

fn y_int(inp: &str) -> [u8; 4] {
    std::iter::once(b' ')
        .chain(
            inp.split(|c: char| !c.is_ascii_digit())
                .filter_map(|x| x.parse::<u8>().ok()),
        )
        .array_chunks()
        .next()
        .unwrap()
}
