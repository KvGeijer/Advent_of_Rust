use std::io::Read;

fn get_input() -> String {    
    let mut file = std::fs::File::open("input1.in").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    return contents;
}


fn part2() {
    /* Just like part 1, but first construct sliding sum list */
    let contents = get_input();

    // Do everything in several steps to be more clear
    let lines = contents.split("\n");
    let depths = lines.map(|x| x.parse::<i32>().unwrap());

    let sliding_zipped = depths.clone().skip(2).zip(depths.clone().skip(1)).zip(depths);
    let sliding_sum = sliding_zipped.map(|((x, y), z)| x + y + z);
    
    let increases = count_increases(sliding_sum);

    println!("Number of depth increases for sliding sum: {}", increases);
}


// The type of this one is confusing. I need to learn traits better
fn count_increases<I: Clone>(list: I) -> usize 
where 
    I: Iterator<Item = i32>
{
    let zipped = list.clone().skip(1).zip(list);
    let diffs = zipped.map(|(x, y)| x - y);
    let increases = diffs.filter(|x| *x > 0).count();

    return increases;
}


fn part1_2() {
    /* Now try to do without for loop */
    let contents = get_input();

    // Do everything in several steps to be more clear
    let lines = contents.split("\n");
    let depths = lines.map(|x| x.parse::<i32>().unwrap());

    let increases = count_increases(depths);

    println!("Number of depth increases: {}", increases);

}


fn part1_1() {
    // Naive first solution, maybe look at part1_2 instead.
    let contents = get_input();

    let lines = contents.split("\n");
    let mut depths = lines.map(|x| x.parse::<i32>().unwrap());

    let mut last = depths.next().unwrap();
    let mut diffs = 0;

    for depth in depths {
        if depth > last {
            diffs += 1;
        }
        last = depth;
    }

    println!("Number of depth increases: {}", diffs);

}


fn main() {
    part1_2();
    part2();
}
