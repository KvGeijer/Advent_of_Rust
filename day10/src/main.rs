const OPEN: [char; 4] = ['(', '{', '[', '<'];
// const MAP: HashMap<char, char> = HashMap::from([ ('(', ')'), ('[', ']'), ('{', '}'), ('<', '>') ]);


fn closing(opening: char) -> char {
    match opening {
        '(' => ')',
        '[' => ']', 
        '{' => '}',
        '<' => '>',
        _   => panic!("invalid opening"),
    }    
}


// Should return tuple of results instead to make real nice
fn analyze(line: &str) -> (char, Vec<char>) { 
    let mut stack = vec![];

    for c in line.chars() {
        if OPEN.contains(&c) {  //New block, push to stack
            stack.push(c);
        }
        else if let Some(opening) = stack.pop() {  // Closing block, pop from stack and check if it is correct or not
            if closing(opening) != c {
                return (c, stack);
            }
        } else {    // More closing ones than opening ones
            panic!("What to do?");
        }
    }

    ('.', stack)
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


fn part2(lines: &Vec<&str>) {
    let mut scores: Vec<u64> = lines.iter()
        .map(|line| analyze(line))
        .filter(|(c, _)| c == &'.')
        .map(|(_, stack)| stack.iter()
            .rev()
            .fold(0u64, |sum, opening| 5*sum + 
                match opening {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _   => panic!("INVALID OPENING")
            }))
        .collect();
    
    scores.sort();
    let middle_score = scores[scores.len()/2];
    
    println!("Middle autocomplete score {}", middle_score);
}


fn main() {
    let string = std::fs::read_to_string("input.in").unwrap();
    let lines: Vec<&str> = string.split('\n').collect();
    
    part1(&lines);
    part2(&lines);
}
