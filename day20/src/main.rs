use std::collections::HashSet;
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
    assert!(alternating);

    (points, enhance, alternating)
        
}


fn get_bin(&(i, j): &(i32, i32), points: &HashSet<(i32, i32)>, bg: bool) -> usize {
    let mut bin = 0;
    for (off_i, off_j) in Itertools::cartesian_product(-1..=1, -1..=1) {
        let ind = (i + off_i, j + off_j);
        bin = (bin << 1) + (bg != points.contains(&ind)) as usize;  //ERROR: Make sure this is correct
    }
    bin
}


fn enhance(rules: &Vec<bool>, points: HashSet<(i32, i32)>, bg: bool, alt: bool) -> HashSet<(i32, i32)> {
    // bg is wether the blackground is light or dark
    // alt is wether bg changes each iteration. If true then new_points should contains every active node same as bg

    // First find a set of "active nodes". These are nodes which have any 8-neighbors in points,
    // meaning they can have the opposite color of the next background.

    let mut active: HashSet<(i32, i32)> = points.iter()
        .cartesian_product(Itertools::cartesian_product(-1..=1, -1..=1))
        .map(|(&(i, j), (off_i, off_j))| (i + off_i, j + off_j))
        .collect();

    // Now find all points which will not have the same color as the background in the next iteration
    let next_bg = alt && !bg;
    active.retain(|&ind| {      // This retaining does not seem that fast
            let bin = get_bin(&ind, &points, bg);
            rules[bin] != next_bg
        });
    
    active
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
