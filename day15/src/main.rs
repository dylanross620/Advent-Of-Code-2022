use std::{fs, collections::HashSet, sync::{Arc, mpsc::channel}};
use regex::Regex;
use threadpool::ThreadPool;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn dist(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn main() {
    let lines = fs::read_to_string("puzzle_input").unwrap();
    let lines = lines.split("\n").map(|l| l.trim()).filter(|l| l.len() > 0);

    let re = Regex::new(r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)").unwrap();
    let mut input: Vec<[Point; 2]> = Vec::new();
    for line in lines {
        for cap in re.captures_iter(line) {
            input.push([Point { x: cap[1].parse().unwrap(), y: cap[2].parse().unwrap() }, Point { x: cap[3].parse().unwrap(), y: cap[4].parse().unwrap() }]);
        }
    }

    println!("----- Puzzle 1 -----");
    //puzzle_1(&input);

    println!("----- Puzzle 2 -----");
    puzzle_2(&input);
}

fn update_max_min(p: Point, dist: isize, x_min: &mut isize, x_max: &mut isize, y_min: &mut isize, y_max: &mut isize) {
    if p.x - dist < *x_min {
        *x_min = p.x - dist;
    }
    if p.x + dist > *x_max {
        *x_max = p.x + dist;
    }
    if p.y - dist < *y_min {
        *y_min = p.y - dist;
    }
    if p.y + dist > *y_max {
        *y_max = p.y + dist;
    }
}

fn puzzle_1(input: &Vec<[Point; 2]>) {
    let mut x_min = isize::MAX;
    let mut x_max = isize::MIN;
    let mut y_min = isize::MAX;
    let mut y_max = isize::MIN;

    let mut dists: Vec<(Point, isize)> = Vec::new();
    let mut beacons: HashSet<Point> = HashSet::new();

    for inp in input {
        let dist = inp[0].dist(&inp[1]);

        update_max_min(inp[0], dist, &mut x_min, &mut x_max, &mut y_min, &mut y_max);
        update_max_min(inp[1], 0, &mut x_min, &mut x_max, &mut y_min, &mut y_max);

        dists.push((inp[0], dist));
        beacons.insert(inp[1]);
    }

    let mut sum = 0;
    for x in x_min..=x_max {
        let pos = Point { x, y: 2000000 };
        
        if beacons.contains(&pos) {
            continue;
        }

        for (sensor, dist) in &dists {
            if pos.dist(sensor) <= *dist {
                sum += 1;
                break;
            }
        }
    }

    println!("Sum: {}", sum);
}

fn puzzle_2(input: &Vec<[Point; 2]>) {
    let mut dists: Vec<(Point, isize)> = Vec::new();

    for inp in input {
        let dist = inp[0].dist(&inp[1]);

        dists.push((inp[0], dist));
    }

    let dists: Arc<Vec<(Point, isize)>> = Arc::new(dists);

    let pool = ThreadPool::new(16);
    let (tx, rx) = channel();

    let square_len = 4000000;
    for x in 0..=square_len {
        for y in 0..=square_len {
            let tx = tx.clone();
            let dists = dists.clone();
            pool.execute(move || {
                let pos = Point { x, y };
                for (sensor, dist) in &*dists {
                    if pos.dist(sensor) <= *dist {
                        return;
                    }
                }
                
                tx.send(x * 4000000 + y).unwrap();
            });
        }
    }

    pool.join();
    let frequency: isize = rx.iter().next().unwrap();

    println!("Frequency: {}", frequency);
}
