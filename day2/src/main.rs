use std::io::{BufReader, BufRead};
use std::fs::File;
use regex::Regex;
use std::time::Instant;


struct Position {
    depth: i32,
    dist: i32,
    aim: i32,
}

// To be able to print it as a normal value. Not really needed.
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(dist: {}, depth: {}, aim {})", self.dist, self.depth, self.aim)
    }
}


fn get_lines(path: &str) -> std::io::Lines<BufReader<File>> {    
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    buffered.lines()
}

fn part2() {
    let some_lines = get_lines("input1.in").map(|line| line.unwrap());
    let mut pos = Position{dist: 0, depth: 0, aim: 0};

    let re = Regex::new(r"^(\w+) (\d+)$").unwrap();
    for line in some_lines {
        let caps = re.captures(&line).unwrap();

        let command = caps.get(1).unwrap().as_str();
        let nbr: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        match command {
            "down" => pos.aim += nbr,
            "up" => pos.aim -= nbr,
            "forward" => {
                pos.dist += nbr;
                pos.depth += nbr * pos.aim;
            }
            _ => println!("Strange match: {:?}", command),
        }

    }

    println!("Final position using aim: {}, mult = {}", pos, pos.depth * pos.dist);
}


fn part1() {
    let some_lines = get_lines("input1.in").map(|line| line.unwrap());
    let mut pos = Position{dist: 0, depth: 0, aim: 0};

    let re = Regex::new(r"^(\w+) (\d+)$").unwrap();
    for line in some_lines {
        let caps = re.captures(&line).unwrap();

        let command = caps.get(1).unwrap().as_str();
        let nbr: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        match command {
            "forward" => pos.dist += nbr,
            "down" => pos.depth += nbr,
            "up" => pos.depth -= nbr,
            _ => println!("Strange match: {:?}", command),
        }

    }

    println!("Final position: {}, mult = {}", pos, pos.depth * pos.dist);
}


fn main() {
    let start = Instant::now();

    part1();
    part2();

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
