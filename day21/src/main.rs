use std::time::Instant;


fn parse(path: &str) -> [usize; 2] {
    let positions: Vec<usize> = std::fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .map(|line| line.as_bytes()[line.len()-1] as usize - '0' as usize)
        .collect();
    
    [positions[0], positions[1]]
}


fn get_possibilities(sum: usize) -> usize {
    // Distributions of possible additions:
    // (3: 1), (4, 3), (5: 6), (6: 7), (7: 6), (8: 3), (9: 1)

    match (sum as i32 - 6).abs() {
        0 => 7,
        1 => 6,
        2 => 3,
        3 => 1,
        _ => panic!("What is this?"),
    }
}


fn dynamic_quantum(you_p: usize, them_p: usize, you_pos: usize, them_pos: usize, dyn_array: &mut [[[[[usize; 2]; 10]; 10]; 21]; 21]) -> [usize; 2] {
    if dyn_array[you_p][them_p][you_pos-1][them_pos-1] != [0, 0] {
        // Already computed solution
        return dyn_array[you_p][them_p][you_pos-1][them_pos-1];
    }
    
    for sum in 3..=9 {

        let new_universes = get_possibilities(sum);
        let new_you_pos = ((you_pos + sum - 1) % 10) + 1;
        let new_you_p = you_p + new_you_pos;

        if new_you_p >= 21 {
            dyn_array[you_p][them_p][you_pos-1][them_pos-1][0] += new_universes;
        } else {
            // In how many universes in the next step do either win?
            let next_pos_res = dynamic_quantum(them_p, new_you_p, them_pos, new_you_pos, dyn_array);

            dyn_array[you_p][them_p][you_pos-1][them_pos-1][0] += new_universes * next_pos_res[1];
            dyn_array[you_p][them_p][you_pos-1][them_pos-1][1] += new_universes * next_pos_res[0];
        };
    }

    dyn_array[you_p][them_p][you_pos-1][them_pos-1]
}


fn part2([you_start, them_start]: [usize; 2]) {
    
    let mut dyn_array = [[[[[0, 0]; 10]; 10]; 21]; 21];

    let [winning_uni, losing_uni] = dynamic_quantum(0, 0, you_start, them_start, &mut dyn_array);

    let most_wins = std::cmp::max(winning_uni, losing_uni);
    println!("The most favourable person wins in {} universes of quantom die!", most_wins)

}


fn part1(mut pos: [usize; 2]) {
    let mut scores = [0, 0];
    let mut add = 6;

    for round in 0.. {
        for i in 0..2 {
            pos[i] = ((pos[i] + add - 1) % 10) + 1;
            scores[i] += pos[i];
            add = (add + 9) % 10;   // ERROR: sub & add 1?

            if scores[i] >= 1000 {
                let res = (round*2 + 1 + i)*3 * scores[1-i] ;
                println!("Player {} won at round {}, with scores: {:?} and result is {}", i, round, scores, res);
                return;
            }

        }
    }
}


fn main() {
    let start = Instant::now();

    let starts = parse("input.in");

    part1(starts);
    part2(starts);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
