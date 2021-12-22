use itertools::Itertools;
use std::cmp::{min, max};
use regex::Regex;
use std::time::Instant;


struct Cuboid {
    on: bool,
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    zmin: i32,
    zmax: i32,
}


fn parse(path: &str) -> Vec<Cuboid> {
    let reg = Regex::new(r"^(on|off) x=(.+)\.\.(.+),y=(.+)\.\.(.+),z=(.+)\.\.(.+)$").unwrap();
    

    std::fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .map(|line| {
            let caps = reg.captures(line).unwrap();
            Cuboid {
                on: &caps[1] == "on", 
                xmin: caps[2].parse().unwrap(), 
                xmax: caps[3].parse().unwrap(), 
                ymin: caps[4].parse().unwrap(), 
                ymax: caps[5].parse().unwrap(), 
                zmin: caps[6].parse().unwrap(), 
                zmax: caps[7].parse().unwrap()}
        })
        .collect()
}


fn part1(cuboids: &Vec<Cuboid>) {
    // Here we know its only a grind from -50 to 50 in all dimensions. So represent as matrix?
    let mut grid = vec![vec![vec![false; 101]; 101]; 101];

    for Cuboid {on, xmin, xmax, ymin, ymax, zmin, zmax} in cuboids {
        
        for ((x, y), z) in (max(*xmin, -50)..=min(50, *xmax))
            .cartesian_product(max(*ymin, -50)..=min(50, *ymax))
            .cartesian_product(max(*zmin, -50)..=min(50, *zmax )) {

                grid[(50 + x) as usize][(50 + y) as usize][(50 + z) as usize] = *on;
        }
    }

    let total:usize = grid.iter()
        .map(|xs| xs.iter()
            .map(|ys| ys.iter()
                .filter(|on| **on)
                .count()
            ).sum::<usize>()
        ).sum::<usize>();
    
    println!("Total number of points with -50, 50 bounds: {}", total);

}


fn main() {
    let start = Instant::now();

    let cuboids = parse("input.in");

    part1(&cuboids);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
