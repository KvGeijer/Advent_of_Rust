use regex::Regex;
use std::collections::HashSet;


fn parse(path: &str) -> (HashSet<(usize, usize)>, Vec<(usize, usize)>) {
    let input = std::fs::read_to_string(path)
        .expect("Expected input file");
    
    let (points, instructions) = input.split_once("\n\n")
        .expect("Could not split once");

    let board: HashSet<(usize, usize)> = points.split('\n')
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
    }).collect();

    let re = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();
    let folds: Vec<(usize, usize)> = instructions.split('\n')
        .map(|line| {
            let caps = re.captures(&line).unwrap();
            let nbr: usize = caps.get(2).unwrap().as_str().parse().unwrap();

            match caps.get(1).unwrap().as_str() {
                "x" => (0, nbr),
                "y" => (1, nbr),
                _   => panic!("parsing"),
            }
        }).rev()
        .collect();

    (board, folds)
}


fn do_fold(fold: (usize, usize), board: HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {

    // Not nice...
    board.iter()
        .map(|&(x, y): &(usize, usize)| {
            let (coord, refl) = fold;
            match coord {
                0 => (refl - (refl as i32 - x as i32).abs() as usize, y),
                _ => (x, refl - (refl as i32 - y as i32).abs() as usize),
            }
        })
        .collect()
}


fn part1(mut board: HashSet<(usize, usize)>, instr: &mut Vec<(usize, usize)>) -> HashSet<(usize, usize)> {
    for _i in 0..1 {
        let fold = instr.pop().unwrap();

        board = do_fold(fold, board);
    }

    let nbr = board.len();
    println!("Number of unique points left after one fold: {}", nbr);

    board
}


fn part2(mut board: HashSet<(usize, usize)>, instr: &mut Vec<(usize, usize)>) {
    let mut xmax = 1000;
    let mut ymax = 1000;
    
    while let Some(fold) = instr.pop() {
        if fold.0 == 0 {
            xmax = fold.1;
        } else {
            ymax = fold.1;
        }

        board = do_fold(fold, board);
    }

    let mut bool_board: Vec<Vec<bool>> = vec![vec![false; xmax]; ymax];
    for &(x, y) in board.iter() {
        bool_board[y][x] = true;
    }

    // Ugly
    let mut string = String::new();
    for row in bool_board.iter() {
        for val in row.iter() {
            if *val {
                string.push('#');
            } else {
                string.push('.');
            }
        }
        string.push('\n');
    }

    println!("{}", string);
}


fn main() {
    let (mut board, mut instr) = parse("input.in");

    board = part1(board, &mut instr);
    part2(board, &mut instr);
}
