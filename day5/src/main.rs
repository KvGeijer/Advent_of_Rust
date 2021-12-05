use std::fs;
use std::cmp;


struct Line {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}


impl Line {
    fn parse (string: &str) -> Line {
        let vec: Vec<Vec<i32>> = string.split(" -> ")
            .map(|x| x.split(',')
                .map(|y| y.parse::<i32>().unwrap())
                .collect())
            .collect();

        Line{x1: vec[0][0],
            y1: vec[0][1],
            x2: vec[1][0],
            y2: vec[1][1]}
    }


    fn sizeof_board(lines: &Vec<Line>) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;

        for line in lines.iter() {
            let xx = cmp::max(line.x1, line.x2);
            let yy = cmp::max(line.y1, line.y2);

            x = cmp::max(x, xx);
            y = cmp::max(y, yy);
        }

        (x, y)
    }


    fn get_points(&self) -> Vec<(usize, usize)> {
        let mut points: Vec<(usize, usize)> = vec![];
        
        let diff_fn = |z1, z2| 
            if z2 == z1 {
                0
            } else if z2 > z1 {
                1
            } else {
                -1
            };

        let x_diff = diff_fn(self.x1, self.x2);
        let y_diff = diff_fn(self.y1, self.y2);

        let mut x = self.x1;
        let mut y = self.y1;

        while !(x == self.x2 && y == self.y2) {
            points.push((x as usize, y as usize));
            x += x_diff;
            y += y_diff; 
        }
        points.push((self.x2 as usize, self.y2 as usize));

        points
    }
}


fn print_board(board: &Vec<Vec<i32>>) {
    for row in board {
        let mut string = String::new();
        for cell in row {
            if *cell == 0 {
                string.push('.');
            } else {
                string.push_str(&cell.to_string());
            }
        }
        println!("{}", string);
    }
}


fn parse_lines(file: &str) -> Vec<Line> {
    let input: String = fs::read_to_string(file).unwrap();
    input.split('\n')
        .map(Line::parse)
        .collect()
}


fn count_dangerous(board: &Vec<Vec<i32>>) -> i32 {
    let sum = board.iter()
        .map(|row| row.iter()
            .filter(|&&c| c >= 2)
            .count() as i32)
        .sum();
    
    sum
}


fn find_nbr_dangerous(lines: Vec<Line>) -> i32{
    let (x_max, y_max) = Line::sizeof_board(&lines);
    let mut board: Vec<Vec<i32>> = vec![vec![0; x_max as usize + 1]; y_max as usize + 1];

    for line in lines.iter() {
        for (x, y) in line.get_points().iter() {
            board[*y][*x] += 1;
        }
    }

    count_dangerous(&board)
}


fn part2() {
    let lines: Vec<Line> = parse_lines("input.in");
    let dangerous = find_nbr_dangerous(lines);

    println!("Number dangerous spots including diagonals: {}", dangerous);
}


fn part1() {
    let mut lines: Vec<Line> = parse_lines("input.in");
    lines.retain(|line| line.x1 == line.x2 || line.y1 == line.y2);

    let dangerous = find_nbr_dangerous(lines);
    println!("Number dangerous spots excluding diagonals: {}", dangerous);
}


fn main() {
    part1();
    part2();
}
