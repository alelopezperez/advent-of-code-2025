use itertools::Itertools;
fn parse_level_list(input: String) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
#[derive(Debug, PartialEq, Eq)]
enum Safety {
    Increase,
    Decrease,
    NotValid,
}
fn check_safety<'a>(report: impl Iterator<Item = &'a i64> + Clone) -> bool {
    let mut it = report
        .scan(-1, |prev, level| {
            if *prev == -1 {
                *prev = *level;
                Some(Safety::Increase)
            } else if *prev > *level && (*prev - *level) <= 3 {
                *prev = *level;
                Some(Safety::Decrease)
            } else if *level > *prev && (*level - *prev) <= 3 {
                *prev = *level;
                Some(Safety::Increase)
            } else {
                *prev = *level;
                Some(Safety::NotValid)
            }
        })
        .skip(1);
    !it.clone().contains(&Safety::NotValid) && it.all_equal()
}
pub fn part1() {
    let input = std::fs::read_to_string("../day2.txt").unwrap();
    let levels = parse_level_list(input);

    let valid = levels
        .iter()
        .map(|report| {
            let n_report = report
                .iter()
                .scan(-1, |prev, level| {
                    if *prev == -1 {
                        *prev = *level;
                        Some(Safety::Increase)
                    } else if *prev > *level && (*prev - *level) <= 3 {
                        *prev = *level;
                        Some(Safety::Decrease)
                    } else if *level > *prev && (*level - *prev) <= 3 {
                        *prev = *level;
                        Some(Safety::Increase)
                    } else {
                        *prev = *level;
                        Some(Safety::NotValid)
                    }
                })
                .collect::<Vec<_>>();
            n_report.iter().skip(1).all_equal()
        })
        .filter(|r| *r)
        .count();
    println!("{valid}");
}
pub fn part2() {
    let input = std::fs::read_to_string("../day2.txt").unwrap();
    let levels = parse_level_list(input);

    let ans = levels
        .iter()
        .map(|r| {
            for i in 0..r.len() {
                if check_safety(r.iter().skip_nth(i)) {
                    return true;
                }
            }
            false
        })
        .filter(|r| *r)
        .count();

    println!("{ans}");
}

trait SkipNth: Iterator {
    fn skip_nth(self, nth: usize) -> SkipNthAdapter<Self>
    where
        Self: Sized,
    {
        SkipNthAdapter {
            iter: self,
            nth,
            counter: 0,
        }
    }
}
impl<I: Iterator> SkipNth for I {}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone)]
struct SkipNthAdapter<I> {
    iter: I,
    nth: usize,
    counter: usize,
}

impl<I> Iterator for SkipNthAdapter<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == self.nth {
            self.counter += 1;
            self.iter.next();
            self.iter.next()
        } else {
            self.counter += 1;

            self.iter.next()
        }
    }
}
