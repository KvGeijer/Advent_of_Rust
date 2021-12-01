use std::io::Read;

fn day1 () {
    /* The main crux here is how to read and parse input well */

    let mut file = std::fs::File::open("input1.in").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines = contents.split("\n");

    let mut last = lines.next().unwrap();
    let mut diffs = 0;
    for line in lines {
        let depth:i32 = line.parse().unwrap();

        if depth > last {
            diffs += 1;
        }
        last = depth;
    }

    println!("Number of depth increases: {}", diffs);

}

fn main() {
    day1();
}
