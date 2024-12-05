fn find_xmas(
    matrix: &[Vec<char>],
    start: (usize, usize),
    direction: (i32, i32),
    target: &str,
) -> bool {
    let (i, j) = start;

    let word = (0..target.len() as i32)
        .map(|k| {
            let i = i as i32 + k * direction.0;
            let i: usize = match i.try_into() {
                Ok(v) => v,
                _ => return '#',
            };

            let j = j as i32 + k * direction.1;
            let j: usize = match j.try_into() {
                Ok(v) => v,
                _ => return '#',
            };

            matrix.get(i).and_then(|v| v.get(j)).copied().unwrap_or('#')
        })
        .collect::<String>();

    word == target
}
fn generate_x(matrix: &[Vec<char>], start: (usize, usize)) -> bool {
    let (i, j) = start;

    let left_top = (i - 1, j - 1);
    let right_top = (i - 1, j + 1);

    let bot_left = (i + 1, j - 1);
    let bot_right = (i + 1, j + 1);

    let left_top = matrix
        .get(left_top.0)
        .and_then(|v| v.get(left_top.1))
        .copied()
        .unwrap_or('#');
    let right_top = matrix
        .get(right_top.0)
        .and_then(|v| v.get(right_top.1))
        .copied()
        .unwrap_or('#');

    let bot_left = matrix
        .get(bot_left.0)
        .and_then(|v| v.get(bot_left.1))
        .copied()
        .unwrap_or('#');
    let bot_right = matrix
        .get(bot_right.0)
        .and_then(|v| v.get(bot_right.1))
        .copied()
        .unwrap_or('#');

    let mas_1 = format!("{left_top}{bot_right}");
    let mas_1 = mas_1 == "MS" || mas_1 == "SM";

    let mas_2 = format!("{right_top}{bot_left}");
    let mas_2 = mas_2 == "MS" || mas_2 == "SM";

    mas_1 && mas_2
}

fn calculate_value(matrix: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    // Example: Add the value at (row, col) to the sum of its indices
    matrix[row][col] + row as i32 + col as i32
}
pub fn part1() {
    let text = std::fs::read_to_string("../day4.txt").expect("The file should upload");
    let matrix = text
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let all_direction = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let count_xmas = {
        let matrix_ref = &matrix; // Shorten the borrow scope
        let all_direction = &all_direction;
        (0..matrix_ref.len())
            .flat_map(move |i| {
                (0..matrix_ref[i].len()).flat_map(move |j| {
                    all_direction
                        .iter()
                        .map(move |d| find_xmas(matrix_ref, (i, j), *d, "XMAS"))
                })
            })
            .filter(|f| *f)
            .count()
    };
    println!("{count_xmas}");
}
pub fn part2() {
    let text = std::fs::read_to_string("../test4.txt").expect("The file should upload");
    let matrix = text
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut counter = 0;

    for i in 1..matrix.len() {
        for j in 1..matrix[i].len() {
            if matrix[i][j] == 'A' && generate_x(&matrix, (i, j)) {
                counter += 1;
            }
        }
    }
    println!("{counter}");
}
