use std::fs;

fn pseudo_median<V: std::cmp::Ord + Copy>(vec: &Vec<V>) -> V {
    // Relies on the vector being sorted beforehand

    let len = vec.len();
    vec[len / 2]
}

fn parse_crabs(path: &str) -> Vec<i32> {
    let input = fs::read_to_string(path).unwrap();
    input
        .split(',')
        .map(|x| x.parse().expect("Expected number"))
        .collect()
}

fn part1(positions: &Vec<i32>) {
    let optimal = pseudo_median(&positions);
    let fuel: i32 = positions.iter().map(|x| (x - optimal).abs()).sum();

    println!(
        "The optimal position with linearity is {}, using {} fuel",
        optimal, fuel
    );
}

fn bounded_positions(positions: &Vec<i32>) -> [i32; 3] {
    let mean: f32 = positions.iter().sum::<i32>() as f32 / positions.len() as f32;
    [
        (mean - 0.5).floor() as i32,
        mean.round() as i32,
        (mean + 0.5).ceil() as i32,
    ]
}

fn quadratic_fuel(positions: &Vec<i32>, d: i32) -> i32 {
    positions
        .iter()
        .map(|x| {
            let y = (x - d).abs();
            (y + 1) * y / 2
        })
        .sum()
}

fn part2(positions: &Vec<i32>) {
    let ds = bounded_positions(positions);

    let (d, fuel): (i32, &i32) = ds
        .iter()
        .map(|&d| quadratic_fuel(positions, d))
        .zip(ds.iter())
        .min_by_key(|(fuel, _)| *fuel)
        .unwrap();

    println!(
        "The optimal position with quadratics is {}, using {} fuel",
        d, fuel
    );
}

fn main() {
    let mut positions = parse_crabs("input.in");
    positions.sort();

    part1(&positions);
    part2(&positions);
}
