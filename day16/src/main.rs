use std::iter::Peekable;


// Would have been better to keep operator and to let it contain an enum of the types
#[derive(Debug, Clone)]
enum Node {
    Sum{ children: Vec<Node>, version: u64},
    Product{ children: Vec<Node>, version: u64},
    Minimum{ children: Vec<Node>, version: u64},
    Maximum{ children: Vec<Node>, version: u64},
    Gt{ children: Vec<Node>, version: u64},
    Lt{ children: Vec<Node>, version: u64},
    Equal{ children: Vec<Node>, version: u64},
    Literal{ num: u64, version: u64},
}


fn get_bit<T: Iterator<Item=(usize, char)>>(bin_iter: &mut Peekable<T>) -> u64 {
    bin_iter.next()
            .expect("Tried get bit past EOF")
            .1
            .to_digit(2)
            .expect("Expected to get a bit")
            .into()
}


fn parse_binary<T: Iterator<Item=(usize, char)>>(bin_iter: &mut Peekable<T>, len: usize) -> u64 {
    let mut sum = 0;
    for _ in 0..len {
        let bit = get_bit(bin_iter);

        sum = sum*2 + bit;
    }

    sum
}


fn accept_operator<T: Iterator<Item=(usize, char)>>(bin_iter: &mut Peekable<T>, version: u64, type_id: u64) -> Node {
    let (index, length_type) = bin_iter.next().expect("Did not expect EOF");

    let children: Vec<Node> = match length_type {
        '0' => {
            let stop = index + 16 + parse_binary(bin_iter, 15) as usize;

            let mut childs = vec![];
            while bin_iter.peek().unwrap().0 < stop {
                childs.push(parse_node(bin_iter));
            }

            assert!(bin_iter.peek().unwrap().0 == stop);
            childs

        },
        '1' => {
            let subs = parse_binary(bin_iter, 11);

            (0..subs).map(|_| parse_node(bin_iter)).collect()

        },
        _         => panic!("Unknown length type")
    };

    // Would want to just save constructur into a value
    match type_id {
        0 => { Node::Sum {children: children, version: version} },
        1 => { Node::Product {children: children, version: version} },
        2 => { Node::Minimum {children: children, version: version} },
        3 => { Node::Maximum {children: children, version: version} },
        5 => { 
            assert!(children.len() == 2);
            Node::Gt {children: children, version: version} },
        6 => { 
            assert!(children.len() == 2);
            Node::Lt {children: children, version: version} },
        7 => { 
            assert!(children.len() == 2);
            Node::Equal {children: children, version: version} },
        _ => { panic!("Unknown operator type") },
    }    

}


fn accept_literal<T: Iterator<Item=(usize, char)>>(bin_iter: &mut Peekable<T>, version: u64) -> Node {
    let mut num = 0;

    loop {
        let cont = get_bit(bin_iter);

        num = (num << 4) + parse_binary(bin_iter, 4);

        if cont == 0 { break; }
    }

    Node::Literal{num: num, version: version}
}


fn parse_node<T: Iterator<Item=(usize, char)>>(bin_iter: &mut Peekable<T>) -> Node {
    let version = parse_binary(bin_iter, 3);
    let type_id = parse_binary(bin_iter, 3);

    match type_id {
        4 => accept_literal(bin_iter, version),
        _ => accept_operator(bin_iter, version, type_id),
    }
}


fn parse(path: &str) -> Node {
    let binaries: Vec<String> = std::fs::read_to_string(path).unwrap()
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect();

    let mut bin_iter = binaries.iter()
        .map(|s| s.chars())
        .flatten()
        .enumerate()
        .peekable();

    let root = parse_node(&mut bin_iter);

    assert!(bin_iter.count() < 8);

    root
}


fn version_sum(node: &Node) -> u64 {

    // Damn this became ugly with changing out Operator to many different
    match node {
        Node::Literal{num: _, version} => *version,
        Node::Sum{children, version} => *version + children.iter()
            .map(version_sum)
            .sum::<u64>(),
        Node::Product{children, version} => *version + children.iter()
            .map(version_sum)
            .sum::<u64>(),
        Node::Maximum{children, version} => *version + children.iter()
            .map(version_sum)
            .sum::<u64>(),
        Node::Minimum{children, version} => *version + children.iter()
            .map(version_sum)
            .sum::<u64>(),
        Node::Gt{children, version} => *version + children.iter()
            .map(version_sum)
            .sum::<u64>(),
        Node::Lt{children, version} => *version + children.iter()
            .map(version_sum)
            .sum::<u64>(),
        Node::Equal{children, version} => *version + children.iter()
            .map(version_sum)
            .sum::<u64>(),
    
        /*Node::Operator{children, version} => *version + children.iter()
            .map(version_sum)
            .sum::<u64>()*/
    }
}


fn part1(root: &Node) {
    let sum = version_sum(root);

    println!("Sum of all versions: {}", sum);
}


fn evaluate(node: &Node) -> u64 {
    match node {
        Node::Literal{num, version: _} => *num,
        Node::Sum{children, version: _} => children.iter()
            .map(evaluate)
            .sum::<u64>(),
        Node::Product{children, version: _} => children.iter()
            .map(evaluate)
            .product::<u64>(),
        Node::Maximum{children, version: _} => children.iter()
            .map(evaluate)
            .max()
            .unwrap(),
        Node::Minimum{children, version: _} => children.iter()
            .map(evaluate)
            .min()
            .unwrap(),
        Node::Gt{children, version: _} => (evaluate(&children[0]) > evaluate(&children[1])) as u64,
        Node::Lt{children, version: _} => (evaluate(&children[0]) < evaluate(&children[1])) as u64,
        Node::Equal{children, version: _} => (evaluate(&children[0]) == evaluate(&children[1])) as u64,
    }
}


fn part2(root: &Node) {
    let sum = evaluate(root);

    println!("Result of evaluating all packets: {}", sum);
}


// By padding they just mean all binaries have fixed size... So no problem
fn main() {
    let root = parse("input.in");

    part1(&root);
    part2(&root);

}
