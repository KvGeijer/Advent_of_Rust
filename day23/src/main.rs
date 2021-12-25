use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::time::Instant;
use regex::Regex;


const ROOMS_SIZE: usize = 4;
const ROOMS: usize = 4;
const CORRIDOR_SIZE: usize = 7;
const SHIFT: usize = 5;


#[derive(Debug, Clone)]
struct State {
    id: usize,
    corridor: [Option<u8>; 7],
    rooms: [Vec<u8>; 4],
}


impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.board.id == other.board.id
    }
}


impl Eq for Node {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


impl Eq for State {}


#[derive(Clone)]
struct Node {
    board: State,
    value: u32,
}


impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.id as i32);
        state.finish();
    }
}


impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.cmp(&self.value)
            .then_with(|| self.board.id.cmp(&other.board.id))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn kind_to_char(kind: u8) -> char {
    match kind {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        _ => panic!("Error kind"),
    }
}


fn to_digit(letter: &str) -> u8 {
    match letter {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "D" => 3,
        _ => panic!("Wrong"),
    }
}


fn move_cost(kind: u8) -> u32 {
    10u32.pow(kind as u32)  // TODO, replace with match?
}


fn adjacent_hallways(room: u8) -> (u8, u8) {
    // The two hallway positions adjacent to room exit. One is really enough, but this is kinda nice
    (room + 1, room + 2)
}


impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = String::from("#############\n#");

        for i in 0..2 {
            match self.corridor[i] {
                Some(kind) => string.push(kind_to_char(kind)),
                None => string.push('.'),
            }
        }

        string.push(' ');
        for i in 2..=4 {
            match self.corridor[i] {
                Some(kind) => string.push(kind_to_char(kind)),
                None => string.push('.'),
            }
            string.push(' ');
        }

        for i in 5..7 {
            match self.corridor[i] {
                Some(kind) => string.push(kind_to_char(kind)),
                None => string.push('.'),
            }
        }
        string.push('#');

        for layer in (0..ROOMS_SIZE).rev() {
            string.push_str("\n  #");
            for room in 0..4 {
                match self.rooms[room].get(layer) {
                    Some(&kind) => string.push(kind_to_char(kind)),
                    None => string.push('.'),
                }
                string.push('#');
            }
        }

        string.push_str("\n  #########");
        
        write!(f, "{}", string)
    }
}


impl State {
    fn compute_id(rooms: &[Vec<u8>; 4], corridor: &[Option<u8>; 7]) -> usize {
        let mut id = 0;
        for val in corridor.iter() {
            id *= SHIFT;   // TODO: Should be able to change to 5 or two bit shifts for efficiency
            if let Some(kind) = val {
                id += *kind as usize + 1; // +1 needed so 0 is not the same as empty
            }
        }

        
        for room in rooms.iter() {
            for i in (0..ROOMS_SIZE).rev() {
                id *= SHIFT;
                if let Some(kind) = room.get(i) {
                    id += *kind as usize + 1;
                }
            }
        }

        id
    }


    fn mandatory_from_room(&mut self, room: u8) -> Option<u32> {
        if self.rooms[room as usize].is_empty() {
            return None;
        }
        
        let kind = *self.rooms[room as usize].last().unwrap();

        if  !self.rooms[kind as usize].is_empty() {
            // If still shit in the room we want to go to we can't do anything.
            return None;
        }

        // Nothing blocking the corridor?
        let mut iter = if kind >= room {
            let (_, start) = adjacent_hallways(room);
            let (end, _) = adjacent_hallways(kind);
            start..=end
        } else {
            let (_, start) = adjacent_hallways(kind);
            let (end, _) = adjacent_hallways(room);
            start..=end
        };

        let possible = iter.all(|corr| self.corridor[corr as usize].is_none());
        if possible {

            // Don't replace it, just remove (Causes trouble for counting final steps) TODO
            let steps = (ROOMS_SIZE - self.rooms[room as usize].len() + 1) + 2*(room as i32 - kind as i32).abs() as usize;

            let old_pos = (ROOMS - (room as usize + 1))*ROOMS_SIZE + (self.rooms[room as usize].len() - 1);
            self.id -= (kind as usize + 1) * SHIFT.pow(old_pos as u32);
            self.rooms[room as usize].pop();

            assert!(self.id == State::compute_id(&self.rooms, &self.corridor));

            let cost = move_cost(kind)*steps as u32;
            Some(cost)
        }
        else {
            None
        }
    }


    fn mandatory_to_room(&mut self, kind: u8) -> Option<u32> {
        // This function is to show how good I can be at writing ugly code when I feel like it :)

        // Corridor to room seems to give too high cost!

        if !self.rooms[kind as usize].is_empty() {
            return None;
        }

        let mut cost = 0;
        
        let (mut lower_corr, mut upper_corr) = adjacent_hallways(kind);

        let mut moves = 0;
        // Go down the corridor!
        loop {
            moves += 1;

            if Some(kind) == self.corridor[lower_corr as usize] {
                // Move it to the room and continue!

                cost += move_cost(kind) * (moves*2 - 1);
                if lower_corr == 0 {cost -= move_cost(kind)};

                // Update the id
                let pos = ROOMS*ROOMS_SIZE + CORRIDOR_SIZE - (lower_corr + 1) as usize;
                self.id -= (kind + 1) as usize * SHIFT.pow(pos as u32);

                // Actually remove the element as well
                self.corridor[lower_corr as usize] = None;

            } else if self.corridor[lower_corr as usize].is_some() {
                break; // Blocked with the wrong kind
            }

            // Nothing blocking the corridor. Check the room and continue
            if lower_corr > 1 {
                // There is a room to check
                let room = lower_corr - 2;
                loop {
                    if self.rooms[room as usize].last() == Some(&kind) {
                        // Move it!
                        // Move roo,m to room TODO. Check if next one too, and then possibly move that one
                        
                        let steps = 1 + moves*2 + (ROOMS_SIZE - self.rooms[room as usize].len()) as u32; 
                        cost += move_cost(kind) * steps;

                        // Update the id
                        let old_pos = (ROOMS - (room + 1) as usize)*ROOMS_SIZE + (self.rooms[room as usize].len() - 1);
                        self.id -= (kind + 1) as usize * SHIFT.pow(old_pos as u32);

                        // Remove the number
                        self.rooms[room as usize].pop();
                    }
                    else {
                        break; //Wrong kind in the room
                    }
                }
            }

            if lower_corr == 0 {
                break;
            }
            else {
                lower_corr -= 1;
            }
        }

        // Now repeat for going up the corridor (I know, horrendous :))
        moves = 0;
        while upper_corr < CORRIDOR_SIZE as u8 {
            moves += 1;

            if Some(kind) == self.corridor[upper_corr as usize] {
                // Move it to the room and continue!

                cost += move_cost(kind) * (moves*2 - 1);
                if upper_corr == CORRIDOR_SIZE as u8 - 1 {cost -= move_cost(kind)};

                // Update the id
                let pos = ROOMS*ROOMS_SIZE + CORRIDOR_SIZE - (upper_corr + 1) as usize;
                self.id -= (kind + 1) as usize * SHIFT.pow(pos as u32);

                // Actually remove the element as well
                self.corridor[upper_corr as usize] = None;

            } else if self.corridor[upper_corr as usize].is_some() {
                break; // Blocked with the wrong kind
            }

            // Nothing blocking the corridor. Check the room and continue
            if (upper_corr as usize) < CORRIDOR_SIZE - 2 {
                // There is a room to check
                let room = upper_corr - 1;
                loop {
                    if self.rooms[room as usize].last() == Some(&kind) {
                        // Move it!
                        // Move roo,m to room TODO. Check if next one too, and then possibly move that one
                        
                        let steps = 1 + moves*2 + (ROOMS_SIZE - self.rooms[room as usize].len()) as u32; 
                        cost += move_cost(kind) * steps;

                        // Update the id
                        let old_pos = (ROOMS - (room + 1) as usize)*ROOMS_SIZE + (self.rooms[room as usize].len() - 1);
                        self.id -= (kind as usize + 1) * SHIFT.pow(old_pos as u32);

                        // Remove the number
                        self.rooms[room as usize].pop();
                    }
                    else {
                        break; //Wrong kind in the room
                    }
                }
            }

        
            upper_corr += 1;
        }

        if cost > 0 {
            Some(cost)
        } else {
            None
        }
    }


    fn mandatory_moves(mut self, room: u8, mut cost: u32) -> (State, u32) {
        // Have moved thing from room. Can we move things from the room to their correct places?

        // This should really just borrow the self as mutable...

        // Three types of mandatory moves:
        // - From specific room
        // - From corridor to specific room
        // - From anywhere to any ready room
        
        // This is just brute force checking all possible mandatory moves
        let mut changes = false;
        if let Some(extra_cost) = self.mandatory_from_room(room) {
            cost += extra_cost;
            changes = true;
        }

        if let Some(extra_cost) = self.mandatory_to_room(room) {
            cost += extra_cost;
            changes = true;
        }

        while changes {
            changes = false;
            for to_room in 0..ROOMS as u8 {
                // Check if we can move any thing to it
                if let Some(extra_cost) = self.mandatory_to_room(to_room) {
                    cost += extra_cost;
                    changes = true;
                }
            }
        }

        (self, cost)

    }


    fn do_move(mut self, room: u8, corr: u8) -> (State, u32) {
        // TODO: Multiply cost with number of steps!

        // Can earlier chack if the move has been done before. OPT

        let moving = *self.rooms[room as usize].last().unwrap();


        let (lower, upper) = adjacent_hallways(room);
        let mut steps = (ROOMS_SIZE - self.rooms[room as usize].len()) + if corr >= upper {
            (1 + corr - upper) as usize * 2
        } else {
            (1 + lower - corr) as usize * 2
        };
        if corr == 0 || (corr as usize) == CORRIDOR_SIZE - 1 {
            steps -= 1;
        }
            
        let cost = move_cost(moving) * steps as u32;

        let old_id = self.id;

        // Can move together these four lines into one or two
        let old_pos = (ROOMS - (room as usize + 1))*ROOMS_SIZE + (self.rooms[room as usize].len() - 1);
        self.id -= (moving + 1) as usize * SHIFT.pow(old_pos as u32);

        let new_pos = ROOMS*ROOMS_SIZE + (CORRIDOR_SIZE - (corr as usize + 1));
        self.id += (moving + 1) as usize * SHIFT.pow(new_pos as u32);

        self.corridor[corr as usize] = self.rooms[room as usize].pop();
        assert!(self.corridor[corr as usize].is_some());

        if self.id != State::compute_id(&self.rooms, &self.corridor) {
            println!("WRONG UPDATE: Old: {}, new: {}, supposed new: {}, kind: {}, room: {}",
                old_id, self.id, State::compute_id(&self.rooms, &self.corridor), moving, room);
            panic!();
        }
        else {
            //println!("CORRECT UPDATE: Old: {}, new: {}, supposed new: {}, kind: {}, room: {}",
            //    old_id, self.id, State::compute_id(&self.rooms, &self.corridor), moving, room);
        }

        self.mandatory_moves(room, cost)
    }


    fn possible_moves(&self) -> Vec<(State, u32)> {
        let mut moves = vec![];

        for room in (0..4).filter(|&i| !self.rooms[i].is_empty()) {
            let (mut lower_corr, mut upper_corr) = adjacent_hallways(room as u8);
            while self.corridor[lower_corr as usize].is_none() {
                // Possible move to furter down in the corridor
                let new_move = self.clone().do_move(room as u8, lower_corr);
                moves.push(new_move);

                if lower_corr == 0 {
                    break;
                }
                else {
                    lower_corr -= 1;
                }
            }

            while upper_corr < CORRIDOR_SIZE as u8 && self.corridor[upper_corr as usize].is_none() {
                // Possible move to furter up the corridor
                let new_move = self.clone().do_move(room as u8, upper_corr);
                moves.push(new_move);

                upper_corr += 1;
            }
        }

        moves
    }
}


fn parse(path: &str) -> State {
    let input = std::fs::read_to_string(path).unwrap();

    let reg = Regex::new(r"#(\w)#(\w)#(\w)#(\w)#").unwrap();

    let mut rooms = [vec![], vec![], vec![], vec![]];
    for cap in reg.captures_iter(&input) {

        for i in 0..4 {
            rooms[i].push(to_digit(&cap[i+1]));
        }
    }

    for i in 0..4 {
        rooms[i] = rooms[i].drain(0..).rev().collect();
    }

    let id = State::compute_id(&rooms, &[None; 7]);
    State {rooms, corridor: [None; 7], id}

}


fn dijkstras(starting_board: State) -> u32 {

    let mut heap = BinaryHeap::new();
    heap.push(Node {board: starting_board, value: 0});

    let mut memo = HashMap::new();

    while let Some(Node {board, value}) = heap.pop() {

        // ERROR: Change to iter
        if board.id == 0 {
            // Have found the quickest way to the final solution

            return value + (0..4)
                .map(|kind| move_cost(kind) * ((ROOMS_SIZE+1)*ROOMS_SIZE/2) as u32)
                .sum::<u32>();
        }

        for (state, cost) in board.possible_moves() {
            // Can do better. TODO

            if !memo.contains_key(&state) || *memo.get(&state).unwrap() > cost + value {
                memo.insert(state.clone(), cost + value);
                heap.push(Node {board: state, value: cost + value});
            }
        }

    }

    panic!("Ran out of moves to evaluate, but did not reach the end?");

}


fn part1(starting_board: State) {
    let cost = dijkstras(starting_board);

    println!("The chapest solution costs: {}", cost);

}


fn main() {
    let start = Instant::now();

    let statring_board = parse("input.in");

    part1(statring_board.clone());
    
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
