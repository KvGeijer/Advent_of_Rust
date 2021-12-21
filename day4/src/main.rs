use std::fs;
//use itertools;
use std::time::Instant;


const BOARD_SIZE: usize = 5;


struct Board {
    board: [[i32; BOARD_SIZE]; BOARD_SIZE],
    sums: [i32; (BOARD_SIZE + 1) * 2],
    sum_unmarked: i32,  // Did not realise this was needed for finals score...
    finished: bool,
}


impl Board {
    fn parse(string: &str) -> Board {
        let mut board = Board{  board: [[0; 5] ; 5], 
                                sums: [0; (BOARD_SIZE + 1) * 2],
                                sum_unmarked: 0,
                                finished: false};

        let numbers = string.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());
    
        for (i, val) in numbers.enumerate() {
            let x = i % BOARD_SIZE;
            let y = i / BOARD_SIZE;

            board.board[y][x] = val;
            board.sum_unmarked += val;
            board.update_diagonals(x, y, val);
        }

        board
    }


    fn get_diags(x: usize, y: usize) -> Vec<usize> {
        let mut diags = vec![x, BOARD_SIZE + y];
    
        if x == y {
            diags.push(BOARD_SIZE*2);
        }
    
        if x + y == BOARD_SIZE - 1 {
            diags.push(BOARD_SIZE*2 + 1)
        }
    
        diags
    }


    fn update_diagonals(&mut self, x: usize, y:usize, val: i32) {
        for diag in Board::get_diags(x, y) {
            self.sums[diag] += val;

            if self.sums[diag] == -(BOARD_SIZE as i32) {
                self.finished = true;
            }
        }
    }


    fn draw_num(&mut self, num: i32) {

        // Should learn itertool to do product
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if self.board[y][x] == num {
                    self.sum_unmarked -= num;
                    self.update_diagonals(x, y, -(num + 1));
                    return ;
                }
            }
        }
    }
}


fn parse(file: &str) -> (Vec<i32>, Vec<Board>) {

    // Have to bind input to a separate variable? Should be nicer way.
    let input: String = fs::read_to_string(file).unwrap();
    let mut lines = input.split("\n\n");

    // Must be nicer ways to take out first element of iterator right?
    let nums: Vec<i32> = lines.next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>()
            .unwrap())
        .collect();

    let boards = lines.map(Board::parse)
        .collect();

    (nums, boards)
}


fn part2_2(nums: Vec<i32>, mut boards: Vec<Board>) {
    // I did not realise you should not count diagonals, so here I purposfully shut that down
    for board in boards.iter_mut() {
        board.sums[2*BOARD_SIZE] += 1;
        board.sums[2*BOARD_SIZE + 1] += 1;
    }

    let mut loser_unmarked = None;
    for &num in nums.iter() {

        {
            for board in boards.iter_mut() {
                board.draw_num(num);
            }
        }

        boards.retain(|board| {
            !board.finished
        });

        // Maybe not the most pretty of ways
        if boards.len() == 1 {
            loser_unmarked = Some(boards[0].sum_unmarked);
        }
        else if boards.len() == 0 {
            let score = (loser_unmarked.expect("Expected final loser") - num) * num;
            println!("Final winner has score: {} with number {}", 
                score, num);
            break ;
        }
    }
}


fn part2(nums: Vec<i32>, mut boards: Vec<Board>) {
    // I did not realise you should not count diagonals, so here I purposfully shut that down
    for board in boards.iter_mut() {
        board.sums[2*BOARD_SIZE] += 1;
        board.sums[2*BOARD_SIZE + 1] += 1;
    }

    let mut loser_unmarked = None;
    for &num in nums.iter() {

        // So here we just borrow boards as mutable until the iterator goes out of scope?
        let losers = boards.iter_mut()
            .map(|x| {
                x.draw_num(num);
                x
            })
            .fold(vec![], |mut vec, board| {
                if !board.finished {
                    vec.push(board)
                }
                vec
            });

        // Maybe not the most pretty of ways
        if losers.len() == 1 {
            loser_unmarked = Some(losers[0].sum_unmarked);
        }
        else if losers.len() == 0 {
            let score = (loser_unmarked.expect("Expected final loser") - num) * num;
            println!("Final winner has score: {} with number {}", 
                score, num);
            break ;
        }
    }
}


fn part1(nums: Vec<i32>, mut boards: Vec<Board>) {
    // I did not realise you should not count diagonals, so here I purposfully shut that down
    for board in boards.iter_mut() {
        board.sums[2*BOARD_SIZE] += 1;
        board.sums[2*BOARD_SIZE + 1] += 1;
    }

    for &num in nums.iter() {
        // So here we just borrow boards as mutable until the iterator goes out of scope?
        let winner = boards.iter_mut()
            .map(|x| {
                x.draw_num(num);
                x
            })
            .fold(None, |acc, board| if board.finished { Some(board) } else { acc });

        if let Some(board) = winner {
            let score = board.sum_unmarked * num;
            println!("Winning board: {:?}\nwith diags: {:?}\n has score: {} with number {}", 
                board.board, board.sums, score, num);
            break ;
        }
    }
}

// Both part 2s work, but maybe 2_2 is nicer. At least faster
fn main() {
    let start = Instant::now();

    let (nums, boards) = parse("input.in");
    //part1(nums, boards);
    //part2(nums, boards);
    part2_2(nums, boards);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
