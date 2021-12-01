use std::io::Read;


fn get_input() -> String {    
    let mut file = std::fs::File::open("input1.in").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    return contents;
}


fn part1_2() {
    /* Now try to do without for loop */
    let contents = get_input();

    // Do everything in several steps to be more clear
    let lines = contents.split("\n");
    let depths = lines.map(|x| x.parse::<i32>().unwrap());

    let zipped = depths.clone().skip(1).zip(depths);
    let diffs = zipped.map(|(x, y)| x - y);
    let increases = diffs.filter(|x| *x > 0).count();

    println!("Number of depth increases: {}", increases);

}


fn part1_1() {
    /* The main crux here is how to read and parse input well */
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
}
