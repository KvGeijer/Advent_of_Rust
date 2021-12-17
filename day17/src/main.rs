use regex::Regex;
use std::collections::{HashSet, BTreeSet};


fn parse(path: &str) -> (i32, i32, i32, i32) {
    let input = std::fs::read_to_string(path).unwrap();

    let reg = Regex::new(r"x=(.+)\.\.(.+), y=(.+)\.\.(.+)$").unwrap();
    let caps = reg.captures(&input).unwrap();
    
    (   caps[1].parse().unwrap(), caps[2].parse().unwrap(), 
        caps[3].parse().unwrap(), caps[4].parse().unwrap())
}


fn trick_shot(ymin: i32, ymax: i32) -> i32 {

    let mut best = 0;
    for yv in 1..1000 {    // Find a better stopping criteria
        let t = yv as f64 + 0.5;
        let i = (t + (t.powf(2.0) - (2*ymax) as f64).sqrt()).ceil() as i32;  //yi <= ymax
        let yi = i*yv - i*(i-1)/2;

        assert!(yi <= ymax);
        assert!((i-1)*yv - (i-1)*(i-2)/2 > ymax);

        if yi >= ymin {
            best = yv;
        }
    }

    best
}


fn part1(ymin: i32, ymax: i32) {

    let opt_yv = trick_shot(ymin, ymax);
    let top = opt_yv*(opt_yv+1)/2;

    println!("Highest point: {}", top);

}


fn part2(xmin: i32, xmax: i32, ymin: i32, ymax: i32) {
    // First find all possible vx starts, Then for each find all possible vy.
    // If vx != 0 at the end we know all vy must be compact
    // Else they must not be compact, but we can combine all those states into one search.__rust_force_expr!

    let mut stat_vxs = BTreeSet::new();

    // All possible starting velocities.
    let mut velocities = HashSet::new();

    for vx in 0..=xmax {
        // If we are too slow to reach the target
        let limit = (vx + 1)*vx/2;
        if limit < xmin { continue; }

        // We reach the target with zero velocity, needs some extra care as not compact regions
        if limit <= xmax { stat_vxs.insert(vx); }

        let t = vx as f64 + 0.5;
        let mut k = (t - (-(2*xmin) as f64 + t.powf(2.0)).sqrt()).ceil() as i32;    // xk >= xmin, assuming k <= vx
        
        assert!(k > 0);
        assert!(k <= vx);
        assert!(k*vx - k*(k - 1)/2 >= xmin); // xk >= xmin
        assert!((k-1)*vx - (k-1)*(k-2)/2 < xmin); // x{k-1} < xmin

        // Now for every k resulting in a hit (without stationary x) 
        // we search for possible hits in combinatioon with y
        while k*vx - k*(k - 1)/2 <= xmax && k != vx {
            // Is there any (is there ever more than one?) possble vy for this vx and k?
            let mut vy = (ymax as f64 / k as f64 + (k - 1) as f64 / 2.0).floor() as i32;    // yk <= ymax
            
            assert!(k*(vy+1) - k*(k-1)/2 > ymax);   // y > ymax if using vy + 1 
            assert!(k*vy - k*(k-1)/2 <= ymax);

            // yk between ymin & ymax
            while k*vy - k*(k-1)/2 >= ymin {
                velocities.insert((vx, vy));
                vy -= 1;
            }

            k += 1;
        }
    }

    // Now we have found all solutions where we never reach stationary velocities in x
    // So here we loop over the velocities giving stationarity and check all possible y velocities possible
    for &vx in stat_vxs.iter().rev() {  // Could do several at the same time for efficiency
        
        for vy in 1..1000 {    // Find a better stopping criteria
            let t = vy as f64 + 0.5;
            let k = (t + (t.powf(2.0) - (2*ymax) as f64).sqrt()).ceil() as i32;  //yk <= ymax
            let yk = k*vy - k*(k-1)/2;
    
            assert!(yk <= ymax);
            assert!((k-1)*vy - (k-1)*(k-2)/2 > ymax);
    
            if yk >= ymin {
                velocities.insert((vx, vy));
            }
        }
    }

    println!("Total amount of possible trajectories: {}", velocities.len());

}


fn main() {
    let (xmin, xmax, ymin, ymax) = parse("input.in");

    part1(ymin, ymax);
    part2(xmin, xmax, ymin, ymax);

}
