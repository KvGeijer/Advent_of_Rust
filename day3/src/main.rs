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


fn part1(byte_vec_vec: &Vec<Vec<u32>>) {
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

    let treshold = byte_vec_vec.len() / 2;
    let gamma_rate: usize = sum.iter()
        .map(|&count| (count as usize > treshold) as usize)
        .reduce(|acc, count| (acc << 1) + count)
        .unwrap();

    let epsilon_rate = (1 << sum.len()) - gamma_rate - 1;

    println!("Gamma rate: {}, epsilon rate: {}, multiplication: {}", 
        gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
    
}


fn main() {
    let byte_vec_vec = parse();
    part1(&byte_vec_vec);
}
