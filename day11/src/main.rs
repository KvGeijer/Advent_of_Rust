use ndarray::{Array2};
use itertools::Itertools;

// Could not find a good way to initialize 2d array
fn parse(path: &str) -> Array2<u32> {
    let input = std::fs::read_to_string(path).expect("No file found");

    // Cute
    let iter = input.split('\n')
        .enumerate()
        .map(|(row, line)| line.chars()
            .enumerate()
            .map(move |(col, c)| (row, col, c.to_digit(10).unwrap()))
        ).flatten();

    let mut array: Array2<u32> = Array2::zeros((10, 10));

    for (row, col, val) in iter {
        array[[row, col]] = val;
    }

    array
}


fn part1_and_2(path: &str) {
    let mut board = parse(path);
    let mut flashes = 0;
    let mut found = false;
    let mut step = 0;

    while step < 100 || !found {
        board.map_inplace(|x| *x += 1);

        let mut stack: Vec<(usize, usize)> = board.indexed_iter()
            .filter(|((_, _), &val)| val >= 10)
            .map(|(idx, _)| idx)
            .collect();

        while !stack.is_empty() {
            let (row, col) = stack.pop().unwrap();
            flashes += 1;

            // Want a better way
            let iter = Itertools::cartesian_product(-1..=1, -1..=1)
                .map(|(i, j)| (row as i32 + i, col as i32 + j))
                .filter(|(i, j)| *i >= 0 && *i < 10 && *j >= 0 && *j < 10)
                .map(|(i, j)| (i as usize, j as usize));    // Can do better?
            
            for (i, j) in iter {
                board[[i, j]] += 1;
                if board[[i, j]] == 10 {
                    stack.push((i, j))
                }
            }
        }
        board.map_inplace(|x| if *x >= 10 {*x = 0; });

        if !found && board.iter().all(|x| *x == 0) {
            found = true;
            println!("The first total flash wass after step {}", step + 1);
        }

        step += 1;
        if step == 100 {
            println!("Number of flashes: {}", flashes);
        }
    }
}


fn main() {
    let path = "input.in";
    part1_and_2(path);
}
