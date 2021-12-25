use std::collections::HashMap;
use std::time::Instant;


const PERIOD: usize = 18;


#[derive(Eq, PartialEq, Debug)]
enum Var {
    Reg(char),
    Num(i32),
}

#[derive(Eq, PartialEq, Debug)]
enum Instr {
    Inp(char),
    Add(char, Var),
    Mul(char, Var),
    Div(char, Var),
    Mod(char, Var),
    Eql(char, Var),
}


struct Block {
    div: i32,
    signed: i32,
    offset: i32,
}


fn parse(path: &str) -> Vec<Instr> {
    std::fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .map(|line| {
            let (instr, vars) = line.split_once(' ').unwrap();
            if instr == "inp" {
                let var = vars.chars().next().unwrap();
                Instr::Inp(var)
            } else {
                let (to, from) = vars.split_once(' ').unwrap();
                let dest = to.chars().next().unwrap();
                let var = if let Some(num) = from.parse().ok() {
                    Var::Num(num)
                } else {
                    Var::Reg(from.chars().next().unwrap())
                };

                match instr {
                    "add" => Instr::Add(dest, var),
                    "mul" => Instr::Mul(dest, var),
                    "div" => Instr::Div(dest, var),
                    "mod" => Instr::Mod(dest, var),
                    "eql" => Instr::Eql(dest, var),
                    _ => panic!("unknown instruction: {}", instr),
                }
            }
        })
        .collect()
}


#[allow(dead_code)]
fn spot_differences(instructions: &Vec<Instr>) {
    let mut differences: HashMap<usize, Vec<&Instr>> = HashMap::new();
    let mut last_instr: [Option<&Instr>; PERIOD] = [None; PERIOD];

    for period in 0..instructions.len()/PERIOD {
        for pos in 0..PERIOD {
            let instr = &instructions[period*PERIOD + pos];
            println!("instr: {:?}, last: {:?}", instr, last_instr[pos]);
            if let Some(last) = last_instr[pos] {
                
                println!("instr: {:?}, last: {:?}, eq: {}", instr, last, instr==last);
                // So bad want to be able to chain these two ifs...
                if last != instr {
                    differences.entry(pos).or_insert(vec![]).push(instr);
                }
            }
            last_instr[pos] = Some(instr);
        } 
    }
    
    println!("Lines with changes: \n");

    for (pos, instructions) in differences {
        println!("\nPosition: {}", pos);
        for instr in instructions {
            println!("\t {:?}", instr);
        }
    }

}


fn parse_specialized(instructions: Vec<Instr>) -> Vec<Block>{
    // This parses the three unique numbers for each loop

    (0..instructions.len()/PERIOD).map(|period| {
        let div = if let Instr::Div(_, Var::Num(num)) = instructions[period*PERIOD + 4] {
            num
        } else { panic!("Incorrect pattern!")};

        let signed = if let Instr::Add(_, Var::Num(num)) = instructions[period*PERIOD + 5] {
            num
        } else { panic!("Incorrect pattern!")};

        let offset = if let Instr::Add(_, Var::Num(num)) = instructions[period*PERIOD + 15] {
            num
        } else { panic!("Incorrect pattern!")};

        Block{div, signed, offset}
    })
    .collect()
}


#[allow(dead_code)]
fn print_equations(equations: &Vec<(usize, usize, i32)>) {
    for &(old_ind, new_ind, diff) in equations.iter() {
        println!("w[{}] - w[{}] = {}", old_ind, new_ind, diff);
    }
}


fn get_equations(blocks: &Vec<Block>) -> Vec<(usize, usize, i32)> {
    // Returns inds for w_old (pushed) and w_new (popped) and w_old - w_new

    let mut stack = vec![];
    let mut equations = vec![];

    for (iteration, Block{div, signed, offset}) in blocks.iter().enumerate() {
        if *div == 1 {
            // push!
            stack.push((iteration, offset));
        }
        else {
            let (old_it, old_offset) = stack.pop()
                .expect("Should not pop more than push, would mean nonzero result");
            
            equations.push((old_it, iteration, -(old_offset + signed)));
        }
    }



    equations
}


fn part1(equations: &Vec<(usize, usize, i32)>) {
    // Use the equations to maximize the w_i given 0 < w < 10
    let mut ws = vec![0; equations.len()*2];

    for &(old_ind, new_ind, diff) in equations.iter() {
        // w_old = w_new + diff
        let (w_old, w_new) = (1..=9).rev()
            .map(|w_old| (w_old, w_old - diff))
            .filter(|&(_, w_new)| w_new > 0 && w_new <= 9)   // One is enough
            .next()
            .unwrap();

        ws[old_ind] = w_old;
        ws[new_ind] = w_new;
    }

    let serial = ws.iter().fold(0u64, |sum, &digit| sum*10 + digit as u64);
    println!("The highest serial is: {}", serial);

}


fn part2(equations: &Vec<(usize, usize, i32)>) {
    // Use the equations to maximize the w_i given 0 < w < 10
    let mut ws = vec![0; equations.len()*2];

    for &(old_ind, new_ind, diff) in equations.iter() {
        // w_old = w_new + diff
        let (w_old, w_new) = (1..=9)
            .map(|w_old| (w_old, w_old - diff))
            .filter(|&(_, w_new)| w_new > 0 && w_new <= 9)   // One is enough
            .next()
            .unwrap();

        ws[old_ind] = w_old;
        ws[new_ind] = w_new;
    }

    let serial = ws.iter().fold(0u64, |sum, &digit| sum*10 + digit as u64);
    println!("The lowest serial is: {}", serial);

}

fn main() {
    let start = Instant::now();

    let instructions = parse("input.in");
    //spot_differences(&instructions);

    let blocks = parse_specialized(instructions);

    let equations = get_equations(&blocks);
    //print_equations(&equations);

    part1(&equations);
    part2(&equations);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
