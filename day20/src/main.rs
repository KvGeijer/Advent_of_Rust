use std::collections::{HashSet, HashMap};
use itertools::Itertools;
use std::time::Instant;

#[allow(dead_code)]
fn print_points(points: &HashSet<(i32, i32)>) {
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


fn parse(path: &str) -> (HashSet<(i32, i32)>, Vec<bool>, bool) {
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
                '#' => Some((i as i32, j as i32)),
                '.' => None,
                _   => panic!("filter_map is hard :)")
            }
        ))
        .flatten()
        .collect();

    let alternating = enh.as_bytes()[0] == '#' as u8 && enh.as_bytes()[enh.len()-1] == '.' as u8;

    (points, enhance, alternating)
        
}


fn enhance(rules: &Vec<bool>, mut points: HashSet<(i32, i32)>, bg: bool, alt: bool) -> HashSet<(i32, i32)> {
    // bg is wether the blackground is light or dark
    // alt is wether bg changes each iteration. If true then new_points should contains every active node same as bg

    let next_bg = alt && !bg;

    // The addition from neighboring nodes. Even if the background is light we add
    // for dark ones. Then reverse in the end.
    let mut counts: HashMap<(i32, i32), usize> = HashMap::new();
    for (i, j) in points.drain() {
        for (en, (off_i, off_j)) in Itertools::cartesian_product(-1..=1, -1..=1).enumerate() {
            let ind = (i + off_i, j + off_j);

            let count = counts.entry(ind).or_insert(0);
            *count += if en != 0 {2 << (en - 1)} else { 1 };
        }
    }

    counts.drain()
        .filter_map(|(ind, count)| {
            let bin = if bg { (2 << 8) - 1 - count } else { count };
            
            if rules[bin] != next_bg { 
                Some (ind) 
            } else { 
                None 
            }
        })
        .collect()
}


fn part1(mut points: HashSet<(i32, i32)>, enhancement: &Vec<bool>, alt: bool) {
    // Points is a set of all point indexes not matching the background.

    for i in 0..2 {
        points = enhance(enhancement, points, alt && i % 2 == 1, alt);
    }

    println!("The size after 2 enhancements: {}", points.len());
}


fn part2(mut points: HashSet<(i32, i32)>, enhancement: &Vec<bool>, alt: bool) {


    for i in 0..50 {
        points = enhance(enhancement, points, alt && i % 2 == 1, alt);
    }

    println!("The size after 50 enhancements: {}", points.len());
}


fn main() {
    // Sadly today was slowest by far. The solution is really nice, but slow

    let start = Instant::now();

    let (points, enhancement, alternating) = parse("input.in");

    part1(points.clone(), &enhancement, alternating);
    part2(points, &enhancement, alternating);
    
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
