use std::collections::{HashMap, VecDeque};

fn get_rules(rules: &str) -> HashMap<usize, Vec<usize>> {
    rules
        .lines()
        .map(|line| {
            line.split_once('|')
                .map(|(key, value)| {
                    (
                        key.parse::<usize>().unwrap(),
                        value.parse::<usize>().unwrap(),
                    )
                })
                .unwrap()
        })
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_default().push(value);
            acc
        })
}
fn check_page(page: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    page.iter()
        .fold((true, Vec::<usize>::new()), |(mut acc, mut set), num| {
            if let Some(before) = rules.get(num) {
                if set.iter().all(|s| !before.iter().any(|b| s == b)) {
                    set.push(*num);
                } else {
                    acc = false;
                }
            } else {
                set.push(*num);
            }

            (acc, set)
        })
        .0
}
fn fix_page(page: &[usize], rules: &HashMap<usize, Vec<usize>>) -> VecDeque<usize> {
    let mut set = VecDeque::new();
    for num in page {
        if let Some(before) = rules.get(num) {
            let mut later = VecDeque::new();
            for check in set.clone().iter() {
                if before.iter().any(|b| check == b) {
                    let swap = set.pop_back().unwrap();
                    later.push_front(swap);
                }
            }
            set.push_back(*num);
            set.extend(later);
        } else {
            set.push_back(*num);
        }
    }
    set
}
pub fn part1_2() {
    let data = std::fs::read_to_string("../day5.txt").unwrap();
    let (rules, pages) = data.split_once("\n\n").unwrap();

    let rules = get_rules(rules);
    let pages = pages
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let ans = pages
        .iter()
        .filter(|p| check_page(p, &rules))
        .map(|v| v[v.len() / 2])
        .sum::<usize>();

    let ans2 = pages
        .iter()
        .filter(|p| !check_page(p, &rules))
        .map(|inv| fix_page(inv, &rules))
        .map(|v| v[v.len() / 2])
        .sum::<usize>();

    println!("{:?}", ans);

    println!("{:?}", ans2);
}
