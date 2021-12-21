use std::collections::HashMap;
use std::time::Instant;


fn parse(path: &str) -> (HashMap<[u8; 2], usize>, HashMap<[u8; 2], u8>, HashMap<u8, usize>){
    let input = std::fs::read_to_string(path).expect("Expected file");
    let (string, instr) = input.split_once("\n\n").unwrap();

    let instr_map = HashMap::from_iter(instr.split('\n')
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();
            let mut from_iter = from.as_bytes()
                .iter()
                .copied();
            let from_arr = [from_iter.next().unwrap(), from_iter.next().unwrap()];

            (from_arr, to.as_bytes().iter().copied().next().unwrap())
        }));

    let mut char_counts: HashMap<u8, usize> = HashMap::new();

    let mut count_map = HashMap::from_iter(instr_map.keys().map(|&key| (key, 0)));

    let mut chars = string.as_bytes().iter();
    let mut first = chars.next().unwrap();

    let cc = char_counts.entry(*first).or_insert(0);
    *cc += 1;

    while let Some(second) = chars.next() {
        let count = count_map.get_mut(&[*first, *second]).unwrap();
        *count += 1;

        let cc = char_counts.entry(*second).or_insert(0);
        *cc += 1;

        first = second;
    }

    (count_map, instr_map, char_counts)
}


// Counts should really have borrowed arrays as keys
fn simulate(mut counts: HashMap<[u8; 2], usize>, instr: &HashMap<[u8; 2], u8>, mut char_count: HashMap<u8, usize>, iterations: usize) {
    
    for _it in 0..iterations {
        let mut new: HashMap<[u8; 2], usize> = HashMap::from_iter(counts.keys().map(|&key| (key, 0)));

        for (&[first, second], &val) in counts.iter().filter(|(_, &v)| v > 0) {
            let mid = *instr.get(&[first, second]).expect("All pairs should have instructions");

            let cc = char_count.entry(mid).or_insert(0);
            *cc += val;

            for arr in [[first, mid], [mid, second]] {
                let count = new.get_mut(&arr).unwrap();
                *count += val;
            }
        }

        counts = new;
    }

    let mut values: Vec<usize> = char_count.into_values().collect();
    values.sort();
    let answer = values[values.len()-1] - values[0];

    println!("Diff after {} iterations: {}", iterations, answer);
}


fn main() {
    let start = Instant::now();

    let (arr_counts, instr, char_counts) = parse("input.in");
    simulate(arr_counts, &instr, char_counts, 40);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
