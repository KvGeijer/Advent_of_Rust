use std::fs;


const AGES:usize = 9;


fn parse_fishes(path: &str) -> [usize; AGES] {
    let mut fishes = [0; AGES];

    let input: String = fs::read_to_string(path).unwrap();
    let read_fishes = input.split(',')
        .map(|x| x.parse::<usize>().unwrap());
    
    for fish in read_fishes {
        fishes[fish] += 1;
    }

    fishes
}


fn simulate_fishes(mut fishes: [usize; AGES], days: usize) -> usize {
    for _day in 0..days {
        let birthing = fishes[0];
        for age in 0..(AGES-1) {
            fishes[age] = fishes[age + 1];
        }

        fishes[AGES - 2 - 1] += birthing;   // Hard coded
        fishes[AGES - 1] = birthing;
    }

    fishes.iter().sum()
}


fn part1(path: &str) {
    let fishes = parse_fishes(path);
    let days = 80;
    
    let nbr_fishes = simulate_fishes(fishes, days);
    println!("Final number fishes in 1: {}", nbr_fishes);
}


fn part2(path: &str) {
    let fishes = parse_fishes(path);
    let days = 256;
    
    let nbr_fishes = simulate_fishes(fishes, days);
    println!("Final number fishes in 1: {}", nbr_fishes);
}


fn main() {
    let path = "input.in";
    part1(path);
    part2(path);
}
