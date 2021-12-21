use std::fs;
use std::collections::HashSet;
use std::time::Instant;


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


macro_rules! new_num {
    ($id:expr, $($arg:expr),*) => {
            Number {nbr: $id, chars: HashSet::from([$($arg),*]), relations: (0, 0)}
    };
}

/*


#[derive(PartialEq, Eq)]
struct Number {
    hash: i32,
    chars: HashSet<char>
}


impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.hash);
        state.finish();
    }
}


struct ShuffledNumber<'a> {
    chars: Vec<char>,
    originals: HashSet<&'a Number>
}


impl ShuffledNumber<'_> {
    fn update_originals(&mut self, char_possibilities: &HashMap<char, HashSet<char>>) {
        // Try to exclude some originals which should not be possible

        let len = self.chars.len();
        let chars = self.chars.iter().fold(HashSet::new(),
            |set, c| set.union(char_possibilities.get(c).unwrap())
            .copied()
            .collect()
        );

        println!("OLD ORIGINALS: {:?}", self.originals.iter().map(|o| o.hash).collect::<Vec<i32>>());
        self.originals.retain(|org| org.chars.len() == len && org.chars.is_subset(&chars));
        println!("NEW ORIGINALS: {:?}", self.originals.iter().map(|o| o.hash).collect::<Vec<i32>>());

    }

    fn restrict_uses(&self, char_possibilities: &mut HashMap<char, HashSet<char>>) -> bool {
        // Restrict each of the used chars to be in the union of the originals chars.


        let chars: HashSet<char> = HashSet::from_iter(self.originals
            .iter()
            .map(|org| &org.chars)
            .flatten()
            .copied()
        );

        // Restrict possibilities of chars to the ones found
        let mut changes = false;
        for c in self.chars.iter() {
            let set: &HashSet<char> = char_possibilities.get(c).unwrap();
            let new = set & &chars;

            if set != &new {
                println!("Set: {:?}\nChars: {:?} \nNew: {:?} \nchanges: {}\n\n", set, chars, &new, &new != set);
            }
            
            changes |= &new != set;

            char_possibilities.insert(*c, new);
        }

        changes
    }
}





fn create_shuffled<'a> (input: &[String; 10], originals: &'a [Number; 10] ) -> Vec<ShuffledNumber<'a>> {
    // Could not bear to do it with arrays... HOW? Wrap each in Result?
    let shuffleds = input.iter()
        .map(|pattern| {
            let chars: Vec<char> = pattern.chars().collect();
            let orig = HashSet::from_iter(originals.iter().filter(|org| org.chars.len() == chars.len()));
            ShuffledNumber{chars: chars, originals: orig}
        })
        .collect();

    shuffleds
}



// * Decodes the monitor and returns the sum of its outputs.
// * 
// * It does it through fixed point iteration where you over time
// * reduce the possible original characters each shuffled character
// * can be. 
fn decode(disp: &Display, originals: &[Number; 10]) -> u32 {

    let mut shuffleds:Vec<ShuffledNumber> = create_shuffled(&disp.input, originals);
    let mut char_possibilities: HashMap<char, HashSet<char>> = HashMap::from_iter(
        ('a'..='g').map(|c| (c, HashSet::from_iter('a'..='g'))));
    let mut changes = true;

    println!("input: {:?}", disp.input);

    // Fixed point iteration 
    while changes {
        changes = false;

        for shuffled in shuffleds.iter_mut() {
            shuffled.update_originals(&char_possibilities);
            changes |= shuffled.restrict_uses(&mut char_possibilities);
        }

        println!("looping?");
    }

    println!("{:?}", char_possibilities);

    for (_, set) in char_possibilities.iter() {
        assert!(set.len() == 1);
    }

    // Decode output
    let mut total = 0;
    for output in &disp.output {

        let chars: HashSet<char> = HashSet::from_iter(output.chars()
                .map(|c|
                    *char_possibilities.get(&c).unwrap().iter().next().unwrap()
                ));

        // Not maybe the prettiest with for here. Should do some filtering/find
        for original in originals {
            if original.chars == chars {
                total = total * 10 + original.hash;
                break ;
            }
        }
    }

    panic!("ONLY PROCESS ONE ATM");
    total as u32
}

*/

struct Number {
    nbr: u32,
    chars: HashSet<char>,
    relations: (usize, usize),
}


fn get_originals() -> [Number; 10] {
    let mut originals = [
        new_num!(0, 'a', 'b', 'c', 'e', 'f', 'g'),
        new_num!(1, 'c', 'f'),
        new_num!(2, 'a', 'c', 'd', 'e', 'g'),
        new_num!(3, 'a', 'c', 'd', 'f', 'g'),
        new_num!(4, 'b', 'c', 'd', 'f'),
        new_num!(5, 'a', 'b', 'd', 'f', 'g'),
        new_num!(6, 'a', 'b', 'd', 'e', 'f', 'g'),
        new_num!(7, 'a', 'c', 'f'),
        new_num!(8, 'a', 'b', 'c', 'd', 'e', 'f', 'g'),
        new_num!(9, 'a', 'b', 'c', 'd', 'f', 'g'),
    ];

    let mut relations: Vec<(usize, usize)> = originals.iter()
        .map(|original| (
            originals.iter()
                .filter(|org| original.chars.is_subset(&org.chars))
                .count(),
            originals.iter()
                .filter(|org| original.chars.is_superset(&org.chars))
                .count()
        ))
        .collect();

    //calculate relations
    for original in originals.iter_mut().rev() {
        original.relations = relations.pop().expect("Not enough originals");
    }

    originals

}


fn decode(disp: &Display, originals: &[Number; 10]) -> u32 {
    let relations: Vec<(usize, usize)> = disp.output
        .iter()
        .map(|output| {
            let output_set:HashSet<char> = HashSet::from_iter(output.chars());
            (
            disp.input.iter()
                .filter(|input| output_set.is_subset(&HashSet::from_iter(input.chars())))
                .count(),
                disp.input.iter()
                .filter(|input| output_set.is_superset(&HashSet::from_iter(input.chars())))
                .count(),
        )})
        .collect();

    relations.iter()
        .fold(0, |sum, rel| {
            let nbr = originals.iter()
                .filter(|org| &org.relations == rel)
                .next()
                .unwrap()
                .nbr;

            sum*10 + nbr
        })
}


fn part2(displays: &Vec<Display>) {
    let originals = get_originals();
    let result: u32 = displays.iter()
        .map(|disp| decode(disp, &originals))
        .sum::<u32>();

    println!("The sum of all outputs is: {}! SICK", result);
}


fn main() {
    let start = Instant::now();

    let displays = parse("input.in");
    part1(&displays);
    part2(&displays);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
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
