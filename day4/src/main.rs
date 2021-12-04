use std::fs;
//use itertools;


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
        assert!(!self.finished);
        //println!("Board: \n{:?}, sums: \n{:?}, finised");

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


fn main() {
    let (nums, boards) = parse("input.in");
    part1(nums, boards);
}
