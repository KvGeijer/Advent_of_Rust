use regex::Regex;


fn trick_shot(ymin: i32, ymax: i32) -> i32 {

    let mut best = 0;
    for yv in 1..10000 {    // Find a better stopping criteria
        let t = yv as f64 + 0.5;
        let i = (t + (t.powf(2.0) - (2*ymax) as f64).sqrt()).ceil() as i32;  //yi <= ymax
        let yi = i*yv - i*(i-1)/2;

        assert!(yi <= ymax);
        assert!((i-1)*yv - (i-1)*(i-2)/2 > ymax);

        if yi >= ymin {
            best = yv;
        }

    }

    best
}


fn part1(ymin: i32, ymax: i32) {

    let opt_yv = trick_shot(ymin, ymax);
    let top = opt_yv*(opt_yv+1)/2;

    println!("Highest point: {}", top);

}


fn parse(path: &str) -> (i32, i32, i32, i32) {
    let input = std::fs::read_to_string(path).unwrap();

    let reg = Regex::new(r"x=(.+)\.\.(.+), y=(.+)\.\.(.+)$").unwrap();
    let caps = reg.captures(&input).unwrap();
    
    (   caps[1].parse().unwrap(), caps[2].parse().unwrap(), 
        caps[3].parse().unwrap(), caps[4].parse().unwrap())
}


fn main() {
    let (xmin, xmax, ymin, ymax) = parse("input.in");

    part1(ymin, ymax);

}
