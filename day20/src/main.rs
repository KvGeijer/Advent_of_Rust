use std::collections::HashSet;
use itertools::Itertools;
use std::time::Instant;

#[allow(dead_code)]
fn print_points(points: &HashSet<(usize, usize)>) {
    let xmin = points.iter().map(|&(_, x)| x).min().unwrap();
    let xmax = points.iter().map(|&(_, x)| x).max().unwrap();
    let ymin = points.iter().map(|&(y, _)| y).min().unwrap();
    let ymax = points.iter().map(|&(y, _)| y).max().unwrap();

    let mut string = String::new();
    for row in ymin..=ymax {
        for col in xmin..=xmax {
            string.push(if points.contains(&(row, col)) {'#'} else {'.'});
        }
        string.push('\n');
    }

    println!("{}", string);

}


fn parse(path: &str) -> (HashSet<(usize, usize)>, Vec<bool>, bool) {
    let input = std::fs::read_to_string(path).expect("expected valid file");

    let (enh, image) = input.split_once("\n\n").unwrap();

    let enhance = enh.chars()
        .map(|c| c == '#')
        .collect();

    let points = image.split('\n')
        .enumerate()
        .map(|(i, row)| row.chars()
            .enumerate()
            .filter_map(move |(j, c)| match c {
                '#' => Some((i, j)),
                '.' => None,
                _   => panic!("filter_map is hard :)")
            }
        ))
        .flatten()
        .collect();

    let alternating = enh.as_bytes()[0] == '#' as u8 && enh.as_bytes()[enh.len()-1] == '.' as u8;

    (points, enhance, alternating)
        
}


fn enhance(rules: &Vec<bool>, mut points: HashSet<(usize, usize)>, bg: bool, alt: bool, rows: usize, cols: usize) -> HashSet<(usize, usize)> {
    // bg is wether the blackground is light or dark
    // alt is wether bg changes each iteration. If true then new_points should contains every active node same as bg

    let next_bg = alt && !bg;

    // Counts have all indexes off by one. so old (0,0) -> (1, 1)
    let mut counts = vec![vec![0; 1 + 2 + rows]; 1 + 2 + cols];
    for (i, j) in points.drain() {
        for (en, (off_i, off_j)) in Itertools::cartesian_product(0..=2, 0..=2).enumerate() {
            let (row, col) = (i + off_i, j + off_j);

            counts[row][col] += if en != 0 {2 << (en - 1)} else { 1 };
        }
    }

    counts.into_iter()
        .enumerate()
        .map(|(i, row)| row.into_iter()
            .enumerate()
            .filter_map(move |(j, count)| {
                let bin = if bg { (2 << 8) - 1 - count } else { count };
            
                if rules[bin] != next_bg { 
                    Some ((i, j))       // This way we simply translate all points! 
                } else { 
                    None 
                }
            }))
        .flatten()
        .collect()

}


fn solve(mut points: HashSet<(usize, usize)>, enhancement: &Vec<bool>, alt: bool, it: usize) -> usize {
    // By calculating these here we don't have to do it every iteration.
    let rows = points.iter().map(|&(y, _)| y).max().unwrap();
    let cols = points.iter().map(|&(_, x)| x).max().unwrap();

    for i in 0..it {
        points = enhance(enhancement, points, alt && i % 2 == 1, alt, rows + 2*i, cols + 2*i);
    }

    points.len()
}



fn part1(points: HashSet<(usize, usize)>, enhancement: &Vec<bool>, alt: bool) {

    let size = solve(points, enhancement, alt, 2);
    println!("The size after 2 enhancements: {}", size);
}


fn part2(points: HashSet<(usize, usize)>, enhancement: &Vec<bool>, alt: bool) {

    let size = solve(points, enhancement, alt, 50);
    println!("The size after 50 enhancements: {}", size);
}


fn main() {

    let start = Instant::now();

    let (points, enhancement, alternating) = parse("input.in");

    part1(points.clone(), &enhancement, alternating);
    part2(points, &enhancement, alternating);
    
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
