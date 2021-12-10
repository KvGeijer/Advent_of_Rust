const OPEN: [char; 4] = ['(', '{', '[', '<'];
// const MAP: HashMap<char, char> = HashMap::from([ ('(', ')'), ('[', ']'), ('{', '}'), ('<', '>') ]);


fn closes(opening: char, closing: char) -> bool {
    [ ('(', ')'), ('[', ']'), ('{', '}'), ('<', '>') ].contains(&(opening, closing)) 
}


fn analyze(line: &str) -> (char, ()) {
    let mut stack = vec![];

    for c in line.chars() {
        if OPEN.contains(&c) {  //New block, push to stack
            stack.push(c);
        }
        else if let Some(opening) = stack.pop() {  // Closing block, pop from stack and check if it is correct or not
            if !closes(opening, c) {
                return (c, ());
            }
        } else {    // More closing ones than opening ones
            println!("What to do?");
        }
    }

    ('.', ())
}


fn part1(lines: &Vec<&str>) {
    let score: u32 = lines.iter()
        .map(|line| match analyze(line).0 {
            '}' => 1197,
            ')' => 3,
            '>' => 25137,
            ']' => 57,
            _   => 0,
        })
        .sum();
    
    println!("Score for invalid lines {}", score);
}


fn main() {
    let string = std::fs::read_to_string("input.in").unwrap();
    let lines: Vec<&str> = string.split('\n').collect();
    
    part1(&lines);

}
