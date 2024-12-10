use rayon::prelude::*;
use std::collections::HashSet;

struct Lab {
    pos: (i32, i32),
    direction: Direction,
}
impl Lab {
    fn turn_90(&mut self) {
        self.direction = match self.direction {
            Direction::Left(_, _) => Direction::Up(-1, 0),
            Direction::Right(_, _) => Direction::Down(1, 0),
            Direction::Up(_, _) => Direction::Right(0, 1),
            Direction::Down(_, _) => Direction::Left(0, -1),
        }
    }
    fn new(pos: (i32, i32)) -> Self {
        Self {
            pos,
            direction: Direction::Up(-1, 0),
        }
    }
    fn get_direction(&self) -> (i32, i32) {
        match self.direction {
            Direction::Left(i, j) => (i, j),
            Direction::Right(i, j) => (i, j),
            Direction::Up(i, j) => (i, j),
            Direction::Down(i, j) => (i, j),
        }
    }
}
enum Direction {
    Left(i32, i32),
    Right(i32, i32),
    Up(i32, i32),
    Down(i32, i32),
}

fn walk(mut lab: Lab, map: &[Vec<char>]) -> usize {
    let mut unique = HashSet::new();
    while (lab.pos.0 > 0 || lab.pos.0 < map.len() as i32)
        || (lab.pos.1 > 0 || lab.pos.1 < map[0].len() as i32)
    {
        unique.insert(lab.pos);
        let (ahead_i, ahead_j) = (
            lab.pos.0 + lab.get_direction().0,
            lab.pos.1 + lab.get_direction().1,
        );

        let ahead_i: usize = match ahead_i.try_into() {
            Ok(ahead) => ahead,
            _ => break,
        };

        let ahead_j: usize = match ahead_j.try_into() {
            Ok(ahead) => ahead,
            _ => break,
        };

        let cell = match map.get(ahead_i).and_then(|row| row.get(ahead_j)) {
            Some(cell) => cell,
            _ => break,
        };

        if *cell == '#' {
            lab.turn_90();
        } else {
            lab.pos = (ahead_i.try_into().unwrap(), ahead_j.try_into().unwrap());
        }
    }
    unique.len()
}
fn loops(mut lab: Lab, map: &[Vec<char>]) -> bool {
    let mut unique = HashSet::new();
    while (lab.pos.0 > 0 || lab.pos.0 < map.len() as i32)
        || (lab.pos.1 > 0 || lab.pos.1 < map[0].len() as i32)
    {
        if !unique.insert((lab.pos, lab.get_direction())) {
            return true;
        }
        let (ahead_i, ahead_j) = (
            lab.pos.0 + lab.get_direction().0,
            lab.pos.1 + lab.get_direction().1,
        );

        let ahead_i: usize = match ahead_i.try_into() {
            Ok(ahead) => ahead,
            _ => break,
        };

        let ahead_j: usize = match ahead_j.try_into() {
            Ok(ahead) => ahead,
            _ => break,
        };

        let cell = match map.get(ahead_i).and_then(|row| row.get(ahead_j)) {
            Some(cell) => cell,
            _ => break,
        };

        if *cell == '#' {
            lab.turn_90();
        } else {
            lab.pos = (ahead_i.try_into().unwrap(), ahead_j.try_into().unwrap());
        }
    }
    false
}
fn loops_par(mut lab: Lab, map: &[Vec<char>], obstacle: (usize, usize)) -> bool {
    let mut unique = HashSet::new();
    while (lab.pos.0 > 0 || lab.pos.0 < map.len() as i32)
        || (lab.pos.1 > 0 || lab.pos.1 < map[0].len() as i32)
    {
        let (ahead_i, ahead_j) = (
            lab.pos.0 + lab.get_direction().0,
            lab.pos.1 + lab.get_direction().1,
        );

        let ahead_i: usize = match ahead_i.try_into() {
            Ok(ahead) => ahead,
            _ => break,
        };

        let ahead_j: usize = match ahead_j.try_into() {
            Ok(ahead) => ahead,
            _ => break,
        };

        let cell = match map.get(ahead_i).and_then(|row| row.get(ahead_j)) {
            Some(cell) => cell,
            _ => break,
        };

        if *cell == '#' || (ahead_i, ahead_j) == obstacle {
            if !unique.insert((lab.pos, lab.get_direction())) {
                return true;
            }

            lab.turn_90();
        } else {
            lab.pos = (ahead_i.try_into().unwrap(), ahead_j.try_into().unwrap());
        }
    }
    false
}
pub fn part1() {
    let data = std::fs::read_to_string("../day6.txt").unwrap();
    let map = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (pos_i, pos_j) = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, grid)| **grid == '^')
                .map(|(j, _)| (i, j))
        })
        .unwrap();
    let ans = walk(Lab::new((pos_i as i32, pos_j as i32)), &map);

    println!(" {ans}");
}
pub fn part2() {
    let data = std::fs::read_to_string("../day6.txt").unwrap();
    let map = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (pos_i, pos_j) = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, grid)| **grid == '^')
                .map(|(j, _)| (i, j))
        })
        .unwrap();

    let ans: usize = (0..map.len())
        .into_par_iter()
        .map(|i| {
            let map = &map;
            (0..map[0].len())
                .map(move |j| {
                    let lab = Lab::new((pos_i as i32, pos_j as i32));
                    loops_par(lab, map, (i, j))
                })
                .filter(|f| *f)
                .count()
        })
        .sum();

    println!("{ans}");
}
