use std::fs;


fn pseudo_median<V: std::cmp::Ord + Copy>(vec: &Vec<V>) -> V {
    // Relies on the vector being sorted beforehand

    let len = vec.len();
    vec[len/2]
}


fn parse_crabs(path: &str) -> Vec<i32> {
    let input = fs::read_to_string(path)
        .unwrap();
    input.split(',')
        .map(|x| x.parse().expect("Expected number"))
        .collect()
}


fn part1(positions: &Vec<i32>) {
    let optimal = pseudo_median(&positions);
    let fuel: i32 = positions.iter()
        .map(|x| (x - optimal).abs())
        .sum();

    println!("The optimal position is {}, using {} fuel", optimal, fuel);
}


fn main() {
    let mut positions = parse_crabs("input.in");
    positions.sort();

    part1(&positions);
}
