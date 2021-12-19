use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;


const THRESHOLD: usize = 11*(11 + 1)/2;


struct Scanner {
    points: Vec<[i32; 3]>,
    distances: HashMap<i32, (usize, usize)>,
    origin: [i32; 3],
}


fn parse(path: &str) -> VecDeque<Scanner> {
    std::fs::read_to_string(path).expect("No file found")
        .split("\n\n")
        .map(|chunk| {
            let points: Vec<[i32; 3]> = chunk.split('\n')
                .skip(1)
                .map(|line| {
                    let mut it = line.split(',');
                    let mut arr = [0; 3];
                    for pos in 0..=2 {
                        arr[pos] = it.next()
                            .expect("Ran out of line")
                            .parse()
                            .expect("Expected an integer");
                    }
                    arr
            })
            .collect();
            
            let map = points.iter()
                .enumerate()
                .map(|(i, point)| points.iter()
                    .enumerate()
                    .skip(i+1)
                    .map(move |(j, other)| {
                        let [x, y, z] = diff(point, other);
                        let dist = x*x + y*y + z*z;
                        (dist, (i, j))
                    })
                ).flatten()
                .collect();
            
            Scanner {points: points, distances: map, origin: [0; 3]}
        })
        .collect()
}


// So we can "save" a rotation for later
fn rotation([x, y, z]: &[i32; 3], (x_pos, x_sign, y_pos, y_sign): (usize, i32, usize, i32)) -> [i32; 3] {
    let z_pos = 3 - x_pos - y_pos;

    let xy_same = x_sign == y_sign;
    let y_follows_x = y_pos == (x_pos + 1) % 3;
    let z_sign = if y_follows_x == xy_same { 1 } else { -1 };

    let mut pos = [0;3];
    pos[x_pos] = *x * x_sign;
    pos[y_pos] = *y * y_sign;
    pos[z_pos] = *z * z_sign;

    pos
}

// Tried returning a vector of functions. Did not work...
fn rotations(coords: &[i32; 3]) -> Vec<([i32; 3], (usize, i32, usize, i32))> {
    let mut positions = vec![];

    for (x_pos, x_sign) in Itertools::cartesian_product(0..=2, [-1, 1]) {
        for (y_pos, y_sign) in Itertools::cartesian_product(0..=2, [-1, 1]).filter(|(pos, _)| pos != &x_pos) {
            
            let pos = rotation(coords, (x_pos, x_sign, y_pos, y_sign));
            positions.push((pos, (x_pos, x_sign, y_pos, y_sign)));
        }
    }

    positions
}


fn diff(&[x1, y1, z1]: &[i32; 3], &[x2, y2, z2]: &[i32; 3]) -> [i32;3] {
    [x1-x2, y1-y2, z1-z2]
}


fn sum(&[x1, y1, z1]: &[i32; 3], &[x2, y2, z2]: &[i32; 3]) -> [i32;3] {
    [x1+x2, y1+y2, z1+z2]
}


fn try_anchor(scanner: &mut Scanner, anchored: &Scanner) -> bool {
    let overlapping_dists: Vec<(&i32, &(usize, usize))> = scanner.distances.iter()
        .filter(|(dist, _)| anchored.distances.contains_key(dist))
        .collect();

    if overlapping_dists.len() < THRESHOLD {
        // Can't anchor as we certainly don't have enough common points
        return false;
    }

    let mut rotation_votes: HashMap<(usize, i32, usize, i32), (usize, [i32;3])> = HashMap::new(); 
        
    for (dist, (i, j)) in overlapping_dists {
        // Find The rotation that works to match the two pairs of points to the anchored ones:
        // rot(scan1) - rot(scan2) == +-(anch1 - anch2) <=> rot(scan1 - scan2) == +-(anch1 - anch2)

        let (anch_i, anch_j) = anchored.distances.get(dist).unwrap();

        let anch_diff = diff(&anchored.points[*anch_i], &anchored.points[*anch_j]);
        let scan_diff = diff(&scanner.points[*i], &scanner.points[*j]);

        for ([x, y, z], rot) in rotations(&scan_diff) {
            if [x, y, z] == anch_diff || [-x, -y, -z] == anch_diff {

                // Not that nice...
                let found_offset = if [x, y, z] == anch_diff {
                    diff(&anchored.points[*anch_i], &rotation(&scanner.points[*i], rot))
                } else {
                    diff(&anchored.points[*anch_j], &rotation(&scanner.points[*i], rot))
                };

                // Should maybe do with entry
                if !rotation_votes.contains_key(&rot) {
                    rotation_votes.insert(rot, (1, found_offset));
                }
                else {
                    rotation_votes.get_mut(&rot).unwrap().0 += 1;
                }
            }
        }
    }

    
    // Now just check which rotation is the most preferred! Must also be above THRESHOLD
    for (rot, &(votes, offset)) in rotation_votes.iter() {
        if votes >= THRESHOLD {
            for point in scanner.points.iter_mut() {
                *point = sum(&rotation(point, *rot), &offset);  
            }
    
            scanner.origin = sum(&rotation(&scanner.origin, *rot), &offset);
            return true;
        }
    }

    false
}


fn find_mappings(mut scanners: VecDeque<Scanner>) -> (HashSet<[i32; 3]>, Vec<[i32; 3]>) {
    // One by one select a scanner from all scanners and try to anchor it
    // By finding an anchored one with enough overlap. Then move over!
    let mut anchored: Vec<Scanner> = vec![scanners.pop_front().unwrap()];

    while let Some(mut scanner) = scanners.pop_front() {
        let mut found = false;
        for anchor in anchored.iter() {
            if try_anchor(&mut scanner, anchor) {
                found = true;
                break;
            }
        }

        if found {
            anchored.push(scanner);
        } else {
            scanners.push_back(scanner);
        }

    }   

    let beacons: HashSet<[i32; 3]> = HashSet::from_iter(anchored.iter().map(|scan| scan.points.iter().copied()).flatten());
    let scanners: Vec<[i32; 3]> = anchored.iter().map(|scan| scan.origin).collect();

    (beacons, scanners)
}


fn part1(beacons: HashSet<[i32; 3]>) {
    println!("Amount of beacons found: {}", beacons.len());
}


fn part2(scanners: Vec<[i32; 3]>) {
    // Double for loop is your friend (at least when so few iterations)

    let dist: i32 = scanners.iter()
        .map(|scanner| scanners.iter()
            .map(|other| diff(scanner, other).iter().map(|x| x.abs()).sum() )
            .max()
            .unwrap() 
        ).max()
        .unwrap();

    println!("Longest manhattan distance between scanners: {}", dist);
}


fn main() {
    // Idea: Have map of all relative distances between beacons for each
    // scanner. Then check for overlaps in the keys of those maps.

    let scanners = parse("input.in");

    let (beacons, scanners) = find_mappings(scanners);
    part1(beacons);
    part2(scanners);

}
