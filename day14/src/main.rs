use std::collections::HashMap;


fn parse(path: &str) -> (Vec<u8>, HashMap<[u8; 2], u8>){
    let input = std::fs::read_to_string(path).expect("Expected file");
    let (string, instr) = input.split_once("\n\n").unwrap();

    let map = HashMap::from_iter(instr.split('\n')
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();
            let mut from_iter = from.as_bytes()
                .iter()
                .copied();
            let from_arr = [from_iter.next().unwrap(), from_iter.next().unwrap()];

            (from_arr, to.as_bytes().iter().copied().next().unwrap())
        }));

    // Could also have HashMap<[u8; 2], usize> keeping count of each pair. Probably more efficient
    let work = string.as_bytes()
        .iter()
        .copied()
        .collect();

    (work, map)
}

// Not efficient at all :/
fn part1(mut string: Vec<u8>, map: &HashMap<[u8; 2], u8>) {

    
    for _ in 0..40 {
        let mut new = vec![];

        //println!("String: {:?}", string.iter().map(|&c| c as char).collect::<Vec<char>>());

        let mut first = string.pop().unwrap();
        new.push(first.clone());

        while let Some(second) = string.pop() {
           
            if let Some(mid) = map.get(&[second, first]) {  // Reverse order
                new.push(*mid);
            }
            new.push(second.clone());

            first = second;
        }

        println!("New length: {}", new.len());

        string = new;
        string.reverse();

    }

    let mut quantities = HashMap::new();
    for c in string.iter() {
        let counter = quantities.entry(c).or_insert(0);
        *counter += 1;
    }

    let mut values: Vec<i32> = quantities.into_values().collect();
    values.sort();
    println!("Values: {:?}", values);

    let answer = values[values.len()-1] - values[0];

    //println!("String: {:?}", string);
    println!("Diff after 10 iterations: {}", answer);
}


fn main() {
    let (string, map) = parse("input.in");
    
    part1(string.clone(), &map);
}
