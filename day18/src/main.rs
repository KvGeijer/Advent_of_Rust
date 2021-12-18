use std::iter::Peekable;
use std::ops::Add;



#[derive(Clone)]
enum Num {
    Literal(u32),
    Pair(Box<[Num; 2]>),
}


fn get_char<T: Iterator<Item=char>>(iter: &mut T) -> char {
    iter.next()
        .expect("Ran out of input!")
}

fn peek_char<T: Iterator<Item=char>>(iter: &mut Peekable<T>) -> &char {
    iter.peek()
        .expect("Ran out of input!")
}


fn accept<T: Iterator<Item=char>>(iter: &mut T, c: char) {
    let next = get_char(iter);
    if next != c {
        panic!("Expected: {}, got: {}", c, next);
    }
}


impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Num::Literal(nbr) => write!(f, "{}", nbr),
            Num::Pair(boxx) => {
                let [left, right] = &**boxx;        // * to get rid of box, then & to get reference to insides
                write!(f, "[{},{}]", left, right)
            }
        }
    }
}


impl Num {
    fn accept_pair<T: Iterator<Item=char>>(iter: &mut Peekable<T>) -> Num {
        accept(iter, '[');

        let first = match peek_char(iter) {
            '[' => Num::accept_pair(iter),
            _   => Num::accept_literal(iter),
        };
        
        accept(iter, ',');

        let second = match peek_char(iter) {
            '[' => Num::accept_pair(iter),
            _   => Num::accept_literal(iter),
        };

        accept(iter, ']');
        
        Num::Pair(Box::new([first, second]))
    }

    fn accept_literal<T: Iterator<Item=char>>(iter: &mut Peekable<T>) -> Num {
        let c = get_char(iter);

        let nbr = c.to_digit(10)
            .expect(&format!("Expected a number, found: {}", c));

        Num::Literal(nbr)
    }

    fn add_last(&mut self, addition: u32) {
        match self {
            Num::Literal(nbr) => *nbr += addition,
            Num::Pair(boxx) => {
                let second = &mut boxx[1];
                second.add_last(addition);
            },
        }
    }

    fn explode(&mut self, addtion: u32) -> (u32, u32) {
        if let Num::Pair(boxx) = self {
            // TODO: Ugly...
            let left: u32 = if let Num::Literal(nbr) = boxx[0] {
                nbr
            } else { 
                panic!("Matching left!!"); 
            };

            let right: u32 = if let Num::Literal(nbr) = boxx[1] {
                nbr
            } else { 
                panic!("Matching right!!"); 
            };

            *self = Num::Literal(0);

            (left + addtion, right)
        }
        else {
            panic!("Pattern matching against a box is hard...");
        }
    }

    fn explosions_rec(&mut self, depth: usize, addtion: u32, changes: &mut bool) -> (u32, u32) {
        // Originally wanted to pass a pointer to last, but then we would have
        // two mutable pointers to the same object => errors

        match self {
            Num::Literal(nbr) => {
                if addtion != 0 {
                    *nbr += addtion;
                }

                (0, 0)
            },

            Num::Pair(boxx) => {
                if depth >= 4 {
                        // TODO: Check all updates applied thouroughly
                    *changes = true;
                    self.explode(addtion)
                }
                else {
                    let [first, second] = &mut **boxx;

                    // ERROR: Can two explosions in some way interfere?
                    let (left1, right1) = first.explosions_rec(depth + 1, addtion, changes);
                    let (left2, right2) = second.explosions_rec(depth + 1, right1, changes);
                    
                    if left2 != 0 {
                        first.add_last(left2);
                    }

                    (left1, right2)
                }
            }
        }
    }

    fn explosions(&mut self, changes: &mut bool) {
        self.explosions_rec(0, 0, changes);
    }


    fn splits(&mut self, changes: &mut bool, depth: usize) -> bool {
        // If any one is spitted and can be exploded, then must be exploded immediately.
        // Now simple solution is to just stop after one split and run explosions, but that is slow. But thats life

        match self {
            Num::Literal(nbr) => {
                if *nbr >= 10 {
                    let left = *nbr/2;
                    let right = (*nbr + 1) / 2;

                    *self = Num::Pair(Box::new([Num::Literal(left), Num::Literal(right)]));

                    *changes = true;
                    depth >= 3
                } else {
                    false
                }
            },
            Num::Pair(boxx) => {
                let [left, right] = &mut **boxx;

                let stop = left.splits(changes, depth+1);
                if stop { assert!(*changes); return true }

                right.splits(changes, depth+1)
            }
        }

    }

    fn magnitude (&self) -> u32 {
        match self {
            Num::Literal(nbr) => *nbr,
            Num::Pair(boxx) => {
                let [left, right] = &**boxx;        // * to get rid of box, then & to get reference to insides
                3*left.magnitude() + 2*right.magnitude()
            }
        }
    }

}


impl Add for Num {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut num = Self::Pair ( Box::new([
            self,
            other
        ]));

        let mut changes = true;
        while changes {
            while changes {
                changes = false;
                num.explosions(&mut changes);
            }
            num.splits(&mut changes, 1);    // This 1 instead of 0 cost me an hour :)

        }

        num
    }
}


fn parse(path: &str) -> Vec<Num> {
    std::fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .rev()
        .map(|line| Num::accept_pair(&mut line.chars().peekable()))
        .collect()
}


fn part1(mut numbers: Vec<Num>) {
    let mut result = numbers.pop().unwrap();

    // Replace with iter.sum :O
    while let Some(next) = numbers.pop() {
        result = result + next;
    }

    println!("Final result: {}", result);
    println!("Final magnitude: {}", result.magnitude());

}


fn part2(numbers: Vec<Num>) {
    let mut best = 0;

    for num in numbers.iter() {
        for other in numbers.iter() {
            let mag = (num.clone() + other.clone()).magnitude();
            if mag > best {
                best = mag;
            }
        }
    }

    println!("The highest posible magnitude from adding two numbers: {}", best);

}


fn main() {
    let numbers = parse("input.in");

    part1(numbers.clone());
    part2(numbers);

}
