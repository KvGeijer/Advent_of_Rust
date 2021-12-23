use itertools::Itertools;
use std::cmp::{min, max};
use regex::Regex;
use std::time::Instant;


#[derive(Clone, Debug)]
struct Cuboid {
    bounds: [[i64; 2]; 3],
}

#[derive(Debug)]
struct Region {
    original: Cuboid,
    cuboids: Vec<Cuboid>
}


fn parse(path: &str) -> Vec<(bool, Cuboid)> {
    let reg = Regex::new(r"^(on|off) x=(.+)\.\.(.+),y=(.+)\.\.(.+),z=(.+)\.\.(.+)$").unwrap();
    

    std::fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .map(|line| {
            let caps = reg.captures(line).unwrap();
            (&caps[1] == "on", Cuboid { bounds: [
                [caps[2].parse().unwrap(), 
                caps[3].parse().unwrap()], 
                [caps[4].parse().unwrap(), 
                caps[5].parse().unwrap()], 
                [caps[6].parse().unwrap(), 
                caps[7].parse().unwrap()]]})
        })
        .collect()
}


#[allow(dead_code)]
fn part1_naive(cuboids: &Vec<(bool, Cuboid)>) {
    // Here we know its only a grind from -50 to 50 in all dimensions. So represent as matrix?
    let mut grid = vec![vec![vec![false; 101]; 101]; 101];

    for (on, Cuboid {bounds: [[xmin, xmax], [ymin, ymax], [zmin, zmax]]}) in cuboids {
        
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


impl Cuboid {

    fn volume(&self) -> i64 {
        self.bounds.iter()
            .map(|&[lower, upper]| upper - lower + 1)
            .product()
    }

    fn overlap(mut self, other: &Cuboid) -> Vec<Cuboid> {
        // Returns vector of cuboids not overlapping with other but so that their union is the same

        // First check if actually any overlap.
        if self.bounds.iter()
            .zip(other.bounds.iter())
            .any(|([self_min, self_max], [other_min, other_max])| 
                other_max < self_min || self_max < other_min) {
                    // Some coordinate did not have any overlap, so returning!
                    return vec![self];
                }
            
        // Splits a cuboid into up to seven new ones without any overlap with other
        let mut cuboids = vec![];

        // Better solution: For each coordinate, if the overlap has a stopping point, split cuboid
        // into two/three with only one of the resulting ones (this one) having any conflict left

        for i in 0..3 {
            if self.bounds[i][0] < other.bounds[i][0] {  // ERROR
                // Overlap from top

                assert!(other.bounds[i][0] <= self.bounds[i][1]);

                // Create the new one
                let mut new_bounds = self.bounds.clone();
                new_bounds[i] = [self.bounds[i][0], other.bounds[i][0] - 1];
                cuboids.push(Cuboid {bounds: new_bounds});

                // Update the bounds of self
                //self.bounds[i][1] = other.bounds[i][0] - 1;     //error
                self.bounds[i][0] = other.bounds[i][0];
            }
            if  other.bounds[i][1] < self.bounds[i][1] { // ERROR
                // Overlap from bottom
                
                assert!(self.bounds[i][0] <= other.bounds[i][1]);

                // Create the new one
                let mut new_bounds = self.bounds.clone();
                new_bounds[i] = [other.bounds[i][1] + 1, self.bounds[i][1]];
                cuboids.push(Cuboid {bounds: new_bounds});

                // Update the bounds of self
                //self.bounds[i][0] = other.bounds[i][1] + 1;     // Error
                self.bounds[i][1] = other.bounds[i][1];

            } 
            // If overlapping the whole coordinate, nothing to do about it. Go next
        }

        // TODO: Assert that there is no overlap between self and other anymore
        // Maybe also check so that we have not increased in size and such

        cuboids
    }
}


impl Region {
    fn potential_overlap(&self, cuboid: &Cuboid) -> bool {
        // Could be done more functionally

        // Refactor to impl method in Cuboid
        !self.original.bounds.iter()
            .zip(cuboid.bounds.iter())
            .any(|([self_min, self_max], [other_min, other_max])| 
                other_max < self_min || self_max < other_min)
    }


    fn volume(&self) -> i64 {
        self.cuboids.iter().map(|cuboid| cuboid.volume()).sum()
    }


    fn remove_overlap(&mut self, cuboid: &Cuboid) {
        // Removes overlap between self and cuboid by splitting selfs cuboids

        self.cuboids = self.cuboids.drain(0..)
            .flat_map(|self_cuboid| self_cuboid.overlap(cuboid))
            .collect();
        
    }

}


fn calculate_overlaps(instructions: Vec<(bool, Cuboid)>) -> Vec<Region> {

    let mut regions: Vec<Region> = Vec::with_capacity(instructions.len());

    for (on, cuboid) in instructions {

        for region in regions.iter_mut().filter(|reg| reg.potential_overlap(&cuboid)) {
            region.remove_overlap(&cuboid);
            
        }
        if on {
            regions.push(Region {original: cuboid.clone(), cuboids: vec![cuboid]});
        }
    }

    regions

}


fn part1(regions: &Vec<Region>) {
    let inner_cuboid = Cuboid{bounds: [[-50, 50], [-50, 50], [-50, 50]]};
    let mut inner = Region {original: inner_cuboid.clone(), cuboids: vec![inner_cuboid.clone()]};

    for region in regions.iter().filter(|region| region.potential_overlap(&inner_cuboid) ) {
        for cuboid in region.cuboids.iter() {
            inner.remove_overlap(cuboid);
        }
    }

    let inner_on = 101i64.pow(3) - inner.volume();
    println!("Number pixels on looking at inner grid: {}", inner_on);

}


fn part2(regions: &Vec<Region>) {

    let total_on: i64 = regions.iter().map(|region| region.volume()).sum(); 
    println!("Total pixels on looking at whole grid: {}", total_on);

}


fn main() {
    let start = Instant::now();

    let cuboids = parse("input.in");

    let resulting_regions = calculate_overlaps(cuboids);

    part1(&resulting_regions);
    part2(&resulting_regions);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
