pub fn part1() {
    let data_test = std::fs::read_to_string("../day9.txt").unwrap();
    let data_test = data_test.trim();
    let test = data_test.chars().collect::<Vec<_>>();

    let mut new_test = Vec::new();

    let mut id = 0;
    for (i, elem) in test.iter().enumerate() {
        if i % 2 == 0 {
            let j = elem.to_digit(10).unwrap();
            for _ in 0..j {
                new_test.push(id as i64);
            }
            id += 1;
        } else {
            let j = elem.to_digit(10).unwrap();
            for _ in 0..j {
                new_test.push(-1);
            }
        }
    }

    let mut start = 0;
    let mut end = new_test.len() - 1;
    let mut total = 0;
    while start <= end {
        if new_test[start] == -1 {
            if new_test[end] == -1 {
                end -= 1;
            } else {
                total += start * new_test[end] as usize;
                end -= 1;
                start += 1;
            }
        } else {
            total += start * new_test[start] as usize;
            start += 1;
        }
    }
    println!("{total}");
}
pub fn part2() {
    let data_test = std::fs::read_to_string("../day9.txt").unwrap();
    let data_test = data_test.trim();
    let test = data_test.chars().collect::<Vec<_>>();

    let mut new_test = Vec::new();

    #[derive(Debug, Clone, Copy)]
    enum Space {
        File(usize, usize),
        Free(usize),
    }

    let mut id = 0;
    for (i, elem) in test.iter().enumerate() {
        if i % 2 == 0 {
            let j = elem.to_digit(10).unwrap();
            new_test.push(Space::File(j as usize, id));
            id += 1;
        } else {
            let j = elem.to_digit(10).unwrap();
            new_test.push(Space::Free(j as usize));
        }
    }

    let mut end = new_test.len() - 1;

    while end > 0 {
        if let Space::File(file_space, el_id) = new_test[end] {
            for elem in new_test.iter().take(end).copied().enumerate() {
                if let Space::Free(free_space) = elem.1 {
                    if free_space == file_space {
                        new_test[elem.0] = new_test[end];
                        new_test[end] = Space::Free(file_space);
                        break;
                    } else if free_space > file_space {
                        new_test[elem.0] = new_test[end];
                        new_test.insert(elem.0 + 1, Space::Free(free_space - file_space));
                        new_test[end + 1] = Space::Free(file_space);
                        break;
                    }
                }
            }
            end -= 1;
        } else {
            end -= 1
        }
    }

    let total = new_test.iter().fold((0, 0), |mut acc, curr| {
        if let Space::File(space, id) = curr {
            (0..*space).for_each(|_| {
                acc.0 += acc.1 * *id;
                acc.1 += 1;
            });
        }
        if let Space::Free(space) = curr {
            (0..*space).for_each(|_| {
                acc.1 += 1;
            });
        }
        acc
    });

    println!("{:?}", total.0);
}
