fn sorted_pair_list(file: String) -> (Vec<i64>, Vec<i64>) {
    let (mut left, mut right): (Vec<_>, Vec<_>) = file
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|(l, r)| (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap()))
                .unwrap()
        })
        .unzip();

    left.sort();
    right.sort();

    (left, right)
}
pub fn day_1_part_1() {
    let file = std::fs::read_to_string("../day1_1.txt").unwrap();

    let (left, right) = sorted_pair_list(file);
    let ans: i64 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();
    println!("{ans}");
}
pub fn day_1_part_2() {
    let file = std::fs::read_to_string("../day1_1.txt").unwrap();

    let (left, right) = sorted_pair_list(file);

    let ans: usize = left
        .into_iter()
        .map(|l| {
            let num = right.iter().take_while(|&&i| l >= i).fold(0, |acc, curr| {
                if *curr == l {
                    acc + 1
                } else {
                    acc
                }
            });

            l as usize * num
        })
        .sum();

    println!("{ans}");
}
