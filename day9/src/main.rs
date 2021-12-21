use itertools::Itertools;
use std::time::Instant;


fn parse(path: &str) -> Vec<Vec<u32>> {
    let input = std::fs::read_to_string(path)
        .unwrap();

    input.split("\n")
        .map(|line| line.chars()
            .map(|x| x.to_digit(10u32).unwrap())
            .collect())
        .collect()
}


fn find_low_points(board: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let (rows, cols) = (board.len() as i32, board[0].len() as i32);

    let mut low_points: Vec<(usize, usize)> = vec![];

    for (row, col) in Itertools::cartesian_product(0..rows, 0..cols) {
        let value = board[row as usize][col as usize];

        let mut neighbours = [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .iter()
            .filter_map(|(i, j)| board.get((row + i) as usize).and_then(|sublist| sublist.get((col + j) as usize)));

        if neighbours.all(|&x| x > value) {
            low_points.push((row as usize, col as usize))
        }
    }

    low_points
}


fn part1(board: &Vec<Vec<u32>>) {
    let low_points = find_low_points(board);

    let risk_levels: u32 = low_points.iter()
        .map(|(row, col)| board[*row][*col] + 1)
        .sum();

    println!("Total risk level: {}", risk_levels);
}


fn find_basin(low_point: &(usize, usize), region: i32, search_board: &mut Vec<Vec<i32>>) -> i32 {
    search_board[low_point.0][low_point.1] = region;

    let mut search_stack: Vec<(usize, usize)> = vec![*low_point];
    let mut size = 1;

    while !search_stack.is_empty() {
        let (row, col) = search_stack.pop().unwrap();

        let dirs: Vec<(i32, i32)> = [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .iter()
            .filter(|(i, j)| search_board.get((row as i32 + i) as usize)
                .and_then(|sublist| sublist.get((col as i32 + j) as usize))
                .is_some())
            .copied()
            .collect();

        for (i, j) in dirs {
            let ii = (row as i32 + i) as usize;
            let jj = (col as i32 + j) as usize;

            if search_board[ii][jj] == 0 {
                search_board[ii][jj] = region;
                search_stack.push((ii, jj));
                size += 1;
            }
        }
    }

    size
}


fn part2(board: &Vec<Vec<u32>>) {
    let low_points = find_low_points(board);

    let mut search_board: Vec<Vec<i32>> = board.iter()
        .map(|row| row.iter()
            .map(|x| (*x == 9) as i32 * -1)
            .collect())
        .collect();
    
    let mut basins: Vec<i32> = low_points.iter()
        .enumerate()
        .map(|(i, low_point)| find_basin(low_point, i as i32 + 1, &mut search_board))
        .collect();
    
    basins.sort();

    let prod: i32 = basins.iter()
        .rev()
        .take(3)
        .product();

    println!("Multiplication of 3 largest basins: {}", prod);
}


fn main() {
    let start = Instant::now();

    let grid = parse("input.in");
    part1(&grid);
    part2(&grid);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
