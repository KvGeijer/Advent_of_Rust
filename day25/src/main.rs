use std::time::Instant;


#[derive(PartialEq, Debug)]
enum Pixel {
    OFF,
    RIGHT,
    DOWN,
}


#[allow(dead_code)]
fn print_board(board: &Vec<Vec<Pixel>>) {
    let mut string = String::new();
    for row in board.iter() {
        for pixel in row.iter() {
            match pixel {
                Pixel::OFF => string.push('.'),
                Pixel::RIGHT => string.push('>'),
                Pixel::DOWN => string.push('v'),
            }
        }
        string.push('\n');
    }

    println!("{}", string);

}


fn parse(path: &str) -> Vec<Vec<Pixel>> {
    std::fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .map(|line| line.chars()
            .map(|c| match c {
                '.' => Pixel::OFF,
                '>' => Pixel::RIGHT,
                'v' => Pixel::DOWN,
                _ => panic!("tired"),
            })
            .collect()
        )
        .collect()
}


fn part1(mut board: Vec<Vec<Pixel>>) {
    // Overall quite inefficient. Probably best to have a bool array
    // and then update everything else according to it

    let mut change = true;
    let mut just_moved = false;
    let mut moved_first = false;    //Ugly :)

    let rows = board.len();
    let cols = board[0].len();

    let mut iter = 0;
    while change {
        change = false;

        // Try to shift everything right!
        for i in 0..rows {
            for j in 0..cols-1 {    // Dont do edge case here
                
                if just_moved {
                    just_moved = false;
                }
                else if board[i][j] == Pixel::RIGHT && board[i][j + 1] == Pixel::OFF {
                    change = true;
                    just_moved = true;
                    board[i][j] = Pixel::OFF;
                    board[i][j + 1] = Pixel::RIGHT;

                    if j == 0 {
                        moved_first = true;
                    }

                }
                else {
                    // Nothing to move, just do nothing :(
                }
            }

            if !just_moved && !moved_first && board[i][cols-1] == Pixel::RIGHT && board[i][0] == Pixel::OFF {
                change = true;
                board[i][cols - 1] = Pixel::OFF;
                board[i][0] = Pixel::RIGHT;
            }
            
            moved_first = false;
            just_moved = false;
        }

        // Then try to shift everything down!
        // To use the exact same idea as above we have to loop over rows... If slow it can be fixed
        for j in 0..cols {
            for i in 0..rows-1 {
                
                if just_moved {
                    just_moved = false;
                }
                else if board[i][j] == Pixel::DOWN && board[i + 1][j] == Pixel::OFF {
                    change = true;
                    just_moved = true;
                    board[i][j] = Pixel::OFF;
                    board[i + 1][j] = Pixel::DOWN;

                    if i == 0 {
                        moved_first = true;
                    }

                }
                else {
                    // Nothing to move, just do nothing :(
                }
            }
            if !just_moved && !moved_first && board[rows - 1][j] == Pixel::DOWN && board[0][j] == Pixel::OFF {
                change = true;
                board[rows - 1][j] = Pixel::OFF;
                board[0][j] = Pixel::DOWN;
            }
            
            moved_first = false;
            just_moved = false;
        }
        
        iter += 1;
    }

    println!("iterations needed for everything to stop moving: {}", iter);
}


fn main() {
    let start = Instant::now();

    let board = parse("input.in");

    part1(board);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
