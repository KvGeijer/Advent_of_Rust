use std::fs;


fn parse() -> Vec<Vec<u32>> {
    fs::read_to_string("input1.in")
        .expect("No file found")
        .split('\n')
        .map(|x| x
            .as_bytes()
            .iter()
            .map(|&x| (x - b'0') as u32)
            .collect()
        )
        .collect()
}


fn bin_conv(bin: &Vec<u32>) -> u32 {
    bin.iter()
        .fold(0u32, |acc, count| (acc << 1) + count)
}


fn part2(mut byte_vec_vec: Vec<Vec<u32>>) {
    // Tired, lots of multiplication
    
    let mut byte_vec_vec_copy: Vec<Vec<u32>> = byte_vec_vec.clone();

    let mut bit = 0;
    while byte_vec_vec.len() > 1 {
        let (commons, ties) = most_common(&byte_vec_vec);
        let common = commons[bit];
        let tie = ties[bit];

        byte_vec_vec.retain(|vec| 
            !tie && vec[bit] == common
            || tie && vec[bit] != common);

        bit = (bit + 1) % commons.len();
    }

    bit = 0;
    while byte_vec_vec_copy.len() > 1 {
        let (commons, ties) = most_common(&byte_vec_vec_copy);
        let common = commons[bit];
        let tie = ties[bit];

        byte_vec_vec_copy.retain(|vec| 
            !tie && vec[bit] != common
            || tie && vec[bit] == common);

        bit = (bit + 1) % commons.len();
    }

    let oxygen = bin_conv(&byte_vec_vec[0]);
    let co2 = bin_conv(&byte_vec_vec_copy[0]);

    println!("Oxygen: {}, CO2: {}, multiplication: {}", 
        oxygen, co2, oxygen * co2);

}


fn most_common(byte_vec_vec: &Vec<Vec<u32>>) -> (Vec<u32>, Vec<bool>) {
    let mut vec_iter = byte_vec_vec.iter();

    // Don't like this to_vec...
    let mut sum: Vec<u32> = vec_iter.next()
        .unwrap()
        .to_vec();

    // Can we jus replace these two with reduce?
    for byte_vec in vec_iter {
        for i in 0..(sum.len()) {
            sum[i] += byte_vec[i];
        }
    }

    let treshold = byte_vec_vec.len() as u32 / 2;
    let commons = sum.iter()
        .map(|&count| (count > treshold) as u32)
        .collect();
    
    let ties: Vec<bool> = sum.iter()
        .map(|&count| count == treshold && byte_vec_vec.len() % 2 == 0)
        .collect();

    // If both as common, there will be a 0
    (commons, ties)
}


fn part1(byte_vec_vec: &Vec<Vec<u32>>) {

    let (commons, _) = most_common(byte_vec_vec);

    let gamma_rate: u32 = bin_conv(&commons);
    let epsilon_rate = (1 << commons.len()) - gamma_rate - 1;

    println!("Gamma rate: {}, epsilon rate: {}, multiplication: {}", 
        gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
    
}


fn main() {
    let byte_vec_vec = parse();
    part1(&byte_vec_vec);
    part2(byte_vec_vec);
}
