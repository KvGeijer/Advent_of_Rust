use std::collections::HashMap;
use std::time::Instant;


struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    s: usize,
    t: usize,
}


struct Node {
    edges: Vec<usize>,  // Not really necessary, should only have Vec<usize> for enighbouring nodes
    ind: usize,
    large: bool,
}


struct Edge {
    u: usize,
    v: usize, 
}


struct Visited {
    map: HashMap<usize, u32>,
    treshold: u32,
    double: bool,
}


fn parse(path: &str) -> Graph {
    let input = std::fs::read_to_string(path).expect("File not found");

    let mut cave_mapping: HashMap<&str, usize> = HashMap::new();
    let mut node_ind = 0;
    let mut edge_ind = 0;

    // Ugly but out of time
    let mut s = 11111;
    let mut t = 11111;
    let mut nodes:Vec<Node> = vec![];
    let mut edges:Vec<Edge> = vec![];

    for (from, to) in input.split('\n').map(|line| line.split_once('-').expect("Could not split")) {

        for node in [from, to] {
            if !cave_mapping.contains_key(node) {
                match node {
                    "start" => s = node_ind,
                    "end"  => t = node_ind,
                    _ => {},
                }

                cave_mapping.insert(node, node_ind);

                let u = Node {edges: vec![], ind: node_ind, large: node.chars().next().unwrap().is_uppercase()};
                nodes.push(u);

                node_ind += 1;
            }
        }

        let ui = *cave_mapping.get(from).unwrap();
        let vi = *cave_mapping.get(to).unwrap();

        let edge = Edge {u: ui, v: vi};
        edges.push(edge);

        for i in [ui, vi] {
            nodes[i].edges.push(edge_ind);
        }

        edge_ind += 1;
    }

    assert!(s != 11111 && t != 11111);

    Graph {nodes: nodes, edges: edges, s: s, t: t}
}


impl Visited {
    fn available(&self, node_ind: &usize) -> bool {
        !self.double ||
            !self.map.contains_key(node_ind) ||
            self.map.get(node_ind).unwrap() < &self.treshold
    }


    fn enter(&mut self, node_ind: &usize) {
    
        if !self.map.contains_key(node_ind) {
            self.map.insert(*node_ind, 1);
        }
        else {
            let prev = self.map.get_mut(node_ind).unwrap();

            if *prev == self.treshold {
                self.double = true;
            }

            *prev += 1;
        }
    }


    fn exit(&mut self, node_ind: &usize) {
        assert!(self.map.contains_key(node_ind));
        assert!(self.map.get(node_ind).unwrap() > &0);
        
        let prev = self.map.get_mut(node_ind).unwrap();

        if *prev > self.treshold {
            self.double = false;
        }

        //assert!(*prev <= self.treshold);
        *prev -= 1;
    }
}


impl Node {
    fn other(&self, edge: &Edge) -> usize {
        if edge.u == self.ind {
            edge.v
        }
        else {
            edge.u
        }
    }


    fn find_paths(&self, visited: &mut Visited, graph: &Graph) -> u32 {
        if self.ind == graph.t {
            return 1;
        }

        let mut paths = 0;
        if !self.large {
            visited.enter(&self.ind);
        }

        for edge_ind in self.edges.iter() {
            let other: &Node = &graph.nodes[self.other(&graph.edges[*edge_ind])];

            if other.ind != graph.s && (other.large || visited.available(&other.ind)) {
                paths += other.find_paths(visited, graph);
            }
        }

        if !self.large {
            visited.exit(&self.ind);
        }
        
        paths
    }
}


fn part1(graph: &Graph) {
    let start = &graph.nodes[graph.s];
    
    let mut visited = Visited {map: HashMap::new(), treshold: 1, double: true};

    let nbr_paths = start.find_paths(&mut visited, graph);

    println!("Number of paths found: {}", nbr_paths);
}


fn part2(graph: &Graph) {
    let start = &graph.nodes[graph.s];
    
    let mut visited = Visited {map: HashMap::new(), treshold: 1, double: false};

    let nbr_paths = start.find_paths(&mut visited, graph);

    println!("Number of paths found (allowing double visits): {}", nbr_paths);
}


fn main() {
    let start = Instant::now();

    let graph = parse("input.in");
    part1(&graph);
    part2(&graph);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
