use std::time::Instant;

use itertools::{repeat_n, Itertools};
use rayon::prelude::*;

fn find_operations((target, nums): (usize, Vec<usize>)) -> bool {
    let ops = vec![true; nums.len()];

    for ele in (0..(1 << nums.len())).map(|n| {
        (0..nums.len())
            .map(move |i| (n & (1 << i)) != 0) // Check if the i-th bit is set
            .collect::<Vec<bool>>() // Collect into a Vec<bool>
    }) {
        let sum = nums
            .iter()
            .copied()
            .zip(ele.iter().copied())
            .reduce(|acc, u| {
                if acc.1 {
                    (acc.0 * u.0, u.1)
                } else {
                    (acc.0 + u.0, u.1)
                }
            })
            .unwrap()
            .0;
        if sum == target {
            return true;
        }
    }
    false
}
pub fn part1() {
    let data = std::fs::read_to_string("../day7.txt").unwrap();
    let puzzle: usize = data
        .lines()
        .map(|l| {
            let (target, nums) = l.split_once(": ").unwrap();
            let target = target.parse::<usize>().unwrap();
            let nums = nums
                .split(' ')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (target, nums)
        })
        .filter(|test| find_operations(test.clone()))
        .map(|t| t.0)
        .sum();
    println!("{puzzle}");
}
pub fn part2() {
    let data = std::fs::read_to_string("../day7.txt").unwrap();
    let puzzle = data
        .lines()
        .map(|l| {
            let (target, nums) = l.split_once(": ").unwrap();
            let target = target.parse::<usize>().unwrap();
            let nums = nums
                .split(' ')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (target, nums)
        })
        .collect::<Vec<_>>();

    let start = Instant::now();
    let ans: usize = puzzle
        .par_iter()
        .filter(|test| find_operations_2(test))
        .map(|t| t.0)
        .sum();
    let duration = start.elapsed();
    println!("{ans}->{:?}", duration);

    let start = Instant::now();
    let ans: usize = puzzle
        .par_iter()
        .filter(|test| is_reachable(test.0, &test.1))
        .map(|t| t.0)
        .sum();
    let duration = start.elapsed();
    println!("{ans}->{:?}", duration);
}

fn find_operations_2((target, nums): &(usize, Vec<usize>)) -> bool {
    for ops in repeat_n(0..3, nums.len() - 1).multi_cartesian_product() {
        let sum = nums
            .iter()
            .copied()
            .zip(ops.iter().chain(std::iter::once(&0)))
            .reduce(|acc, u| {
                if *acc.1 == 0 {
                    (acc.0 * u.0, u.1)
                } else if *acc.1 == 1 {
                    (acc.0 + u.0, u.1)
                } else {
                    let new_app = acc.0 * 10_usize.pow((u.0 as f32).log10() as u32 + 1) + u.0;
                    (new_app, u.1)
                }
            })
            .unwrap()
            .0;
        if sum == *target {
            return true;
        }
    }

    false
}
//This was not my code
fn is_reachable(target: usize, nums: &[usize]) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }
    let (&last, rest) = nums.split_last().unwrap();
    if target % last == 0 && is_reachable(target / last, rest) {
        return true;
    }
    if target > last && is_reachable(target - last, rest) {
        return true;
    }
    let last_len = last.ilog10() + 1;
    let magnitude = 10usize.pow(last_len);
    let target_len = target.ilog10() + 1;
    let ending = target % magnitude;
    if target_len > last_len && last == ending && is_reachable(target / magnitude, rest) {
        return true;
    }
    false
}
