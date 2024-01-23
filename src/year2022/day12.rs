use std::collections::{HashSet, VecDeque};

use super::*;

impl Run for Aoc<2022, 12> {
    fn parta(&self) -> Result<AocResult, AocError> {
        let mut data: Vec<Vec<_>> = self.lines().map(|x| x.chars().collect()).collect();
        let mut queue = VecDeque::new();
        let start = data
            .iter()
            .enumerate()
            .find_map(|(i, x)| x.iter().position(|&c| c == 'S').map(|j| [i, j]))
            .unwrap();
        let goal = data
            .iter()
            .enumerate()
            .find_map(|(i, x)| x.iter().position(|&c| c == 'E').map(|j| [i, j]))
            .unwrap();
        data[start[0]][start[1]] = 'a';
        data[goal[0]][goal[1]] = 'z';
        queue.push_back((start, 0));
        let mut ret = 0i64;
        let mut visited = HashSet::new();
        while let Some((pos, i)) = queue.pop_front() {
            if !visited.insert(pos) {
                continue;
            }
            if pos == goal {
                ret = i;
                break;
            }
            if pos[0] != 0 && (data[pos[0] - 1][pos[1]] as u8).saturating_sub(data[pos[0]][pos[1]] as u8) <= 1 {
                queue.push_back(([pos[0] - 1, pos[1]], i + 1))
            }
            if pos[1] != 0 && (data[pos[0]][pos[1] - 1] as u8).saturating_sub(data[pos[0]][pos[1]] as u8) <= 1 {
                queue.push_back(([pos[0], pos[1] - 1], i + 1))
            }
            if pos[0] < data.len() - 1 && (data[pos[0] + 1][pos[1]] as u8).saturating_sub(data[pos[0]][pos[1]] as u8) <= 1 {
                queue.push_back(([pos[0] + 1, pos[1]], i + 1))
            }
            if pos[1] < data[0].len() - 1 && (data[pos[0]][pos[1] + 1] as u8).saturating_sub(data[pos[0]][pos[1]] as u8) <= 1 {
                queue.push_back(([pos[0], pos[1] + 1], i + 1))
            }
        }
        Ok(ret.into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        let mut data: Vec<Vec<_>> = self.lines().map(|x| x.chars().collect()).collect();
        let mut queue = VecDeque::new();
        let start = data
            .iter()
            .enumerate()
            .find_map(|(i, x)| x.iter().position(|&c| c == 'S').map(|j| [i, j]))
            .unwrap();
        let goal = data
            .iter()
            .enumerate()
            .find_map(|(i, x)| x.iter().position(|&c| c == 'E').map(|j| [i, j]))
            .unwrap();
        data[start[0]][start[1]] = 'a';
        data[goal[0]][goal[1]] = 'z';
        queue.push_back((goal, 0));
        let mut ret = 0i64;
        let mut visited = HashSet::new();
        while let Some((pos, i)) = queue.pop_front() {
            if !visited.insert(pos) {
                continue;
            }
            if data[pos[0]][pos[1]] == 'a' {
                ret = i;
                break;
            }
            if pos[0] != 0 && (data[pos[0]][pos[1]] as u8).saturating_sub(data[pos[0] - 1][pos[1]] as u8) <= 1 {
                queue.push_back(([pos[0] - 1, pos[1]], i + 1))
            }
            if pos[1] != 0 && (data[pos[0]][pos[1]] as u8).saturating_sub(data[pos[0]][pos[1] - 1] as u8) <= 1 {
                queue.push_back(([pos[0], pos[1] - 1], i + 1))
            }
            if pos[0] < data.len() - 1 && (data[pos[0]][pos[1]] as u8).saturating_sub(data[pos[0] + 1][pos[1]] as u8) <= 1 {
                queue.push_back(([pos[0] + 1, pos[1]], i + 1))
            }
            if pos[1] < data[0].len() - 1 && (data[pos[0]][pos[1]] as u8).saturating_sub(data[pos[0]][pos[1] + 1] as u8) <= 1 {
                queue.push_back(([pos[0], pos[1] + 1], i + 1))
            }
        }
        Ok(ret.into())
    }
}
