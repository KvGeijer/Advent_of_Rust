use std::fs;


struct Display {
    input: [String; 10],
    output: [String; 4],
}


impl Display {
    fn parse (line: &str) -> Display{

        let mut patterns: Vec<Vec<String>> = line.split(" | ")
            .map(|half| half.split(' ')
                .map(|pattern| pattern.to_string())
                .collect())
            .collect();

        let mut disp = Display {input: Default::default(), output: Default::default()};
        let mut output = patterns.pop().unwrap();
        let mut input = patterns.pop().unwrap();

        for i in (0..10).rev() {
            disp.input[i] = input.pop().unwrap();
        }

        for i in (0..4).rev() {
            disp.output[i] = output.pop().unwrap();
        }

        disp
    }
}


fn parse(path: &str) -> Vec<Display> {
    let input = fs::read_to_string(path)
        .expect("No file found");

    input.split('\n')
        .map(Display::parse) 
        .collect()
}


fn part1(displays: &Vec<Display>) {
    let count = displays.iter()
        .map(|disp| disp.output
            .iter()
            .map(|pattern| [2, 3, 4, 7].contains(&pattern.len()) as u32)
            .sum::<u32>())
        .sum::<u32>();
    
    println!("The numbers 1, 7, 4 and 8 occured {} times!", count);
}


fn main() {
    let displays = parse("input.in");
    part1(&displays);
}
