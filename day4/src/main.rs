use std::fs;

struct Pair {
    r1: Range,
    r2: Range,
}

struct Range {
    start: usize,
    end: usize,
}

impl Pair {
    fn new(from: &str) -> Self {
        let ranges: Vec<&str> = from.split(",").collect();

        Pair {
            r1: Range::new(&ranges[0]),
            r2: Range::new(&ranges[1]),
        }
    }
}

impl Range {
    fn new(from: &str) -> Self {
        let mut endpoints: Vec<usize> = from.split("-").map(|point| point.parse::<usize>().unwrap()).collect();
        endpoints.sort();

        Range {
            start: endpoints[0],
            end: endpoints[1],
        }
    }
}

fn main() {
    let lines = fs::read_to_string("puzzle_input").unwrap();
    let lines = lines.split_terminator("\n").map(|l| l.trim());

    let range_pairs: Vec<Pair> = lines.map(|line| Pair::new(&line)).collect();

    println!("------ Puzzle 1 -----");
    puzzle_1(&range_pairs);

    println!("------ Puzzle 2 -----");
    puzzle_2(&range_pairs);
}

fn is_contained(range_1: &Range, range_2: &Range) -> bool {
    let diff_1 = range_1.end - range_1.start;
    let diff_2 = range_2.end - range_2.start;

    let (larger, smaller) = if diff_1 > diff_2 { (range_1, range_2) } else { (range_2, range_1) };

    smaller.start >= larger.start
        && smaller.start <= larger.end
        && smaller.end >= larger.start
        && smaller.end <= larger.end
}

fn does_overlap(range_1: &Range, range_2: &Range) -> bool {
    let diff_1 = range_1.end - range_1.start;
    let diff_2 = range_2.end - range_2.start;

    let (larger, smaller) = if diff_1 > diff_2 { (range_1, range_2) } else { (range_2, range_1) };

    (smaller.start >= larger.start && smaller.start <= larger.end)
        || (smaller.end >= larger.start && smaller.end <= larger.end)
}

fn puzzle_1(pairs: &Vec<Pair>) {
    let mut sum: usize = 0;

    for pair in pairs {
        if is_contained(&pair.r1, &pair.r2) {
            sum += 1;
        }
    }

    println!("Sum: {}", sum);
}

fn puzzle_2(pairs: &Vec<Pair>) {
    let mut sum: usize = 0;

    for pair in pairs {
        if does_overlap(&pair.r1, &pair.r2) {
            sum += 1;
        }
    }

    println!("Sum: {}", sum);
}
