use std::cmp::Ordering;
use std::collections::BinaryHeap;
use itertools::Itertools;


#[derive(Copy, Clone, Eq, PartialEq)]
struct NodeState {
    row: usize,
    col: usize,
    dist: u32,
}


impl NodeState {

    // Third time? Really want some generator/iterator for this instead of array
    fn neighbours(&self, rows: usize, cols: usize) -> Vec<(usize, usize)> {
        let mut neighs = vec![];

        for (r, c) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let row = r + self.row as i32;
            let col = c + self.col as i32;
            if row < rows as i32
                && row >= 0 
                && col < cols as i32 
                && col >= 0 {
                    neighs.push((row as usize, col as usize));
                }
        }

        neighs
    }
}


impl Ord for NodeState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
            .then_with(|| self.row.cmp(&other.row))
            .then_with(|| self.col.cmp(&other.col))
    }
}

impl PartialOrd for NodeState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn parse(path: &str) -> Vec<Vec<u32>> {
    std::fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .map(|line| line.chars()
            .map(move |c| c.to_digit(10).unwrap())
            .collect()
        )
        .collect()
}


fn upscale(graph: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let rows = graph.len();
    let cols = graph[0].len();

    let mut upscaled = vec![vec![0; cols*5]; rows*5];

    // Really did not want a double for loop
    for (row, col, val) in graph.iter()
        .enumerate()
        .map(|(i, row)| row.iter()
            .enumerate()
            .map(move |(j, &val)| (i, j, val))
        ).flatten() {
        
        for (row_shift, col_shift) in Itertools::cartesian_product(0..5, 0..5) {
            let new_row = row + rows*row_shift;
            let new_col = col + cols*col_shift;
            let new_val = ((val + row_shift as u32 + col_shift as u32 - 1) % 9) + 1;

            upscaled[new_row][new_col] = new_val;
        }
        
    }

    upscaled
}


fn part2(graph: &Vec<Vec<u32>>) {
    
    let dist = dijkstras(graph);
    println!("Shortest path to sink in upscaled map: {}", dist);

}


fn part1(graph: &Vec<Vec<u32>>) {
    
    let dist = dijkstras(graph);
    println!("Shortest path to sink in small map: {}", dist);

}


fn dijkstras(graph: &Vec<Vec<u32>>) -> u32 {
    let rows = graph.len();
    let cols = graph[0].len();

    let mut dists = vec![vec![u32::MAX; cols]; rows];

    let mut heap:BinaryHeap<NodeState> = BinaryHeap::new();
    heap.push(NodeState{row: 0, col: 0, dist: 0});

    while let Some(node) = heap.pop(){

        if node.dist > dists[node.row][node.col] { continue; }

        if node.row == rows - 1 && node.col == cols - 1 { break; }

        for (row, col) in node.neighbours(rows, cols) {
            let dist = graph[row][col] + node.dist;

            if dists[row][col] > dist {
                heap.push(NodeState {row: row, col: col, dist: dist});
                dists[row][col] = dist;
            }
        }

    }

    dists[rows-1][cols-1]
}


fn main() {
    let graph = parse("input.in");
    part1(&graph);
    
    let upscaled = upscale(graph);
    part2(&upscaled);
}
