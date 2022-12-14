use std::{fs, collections::HashMap, cmp::{min, max}};
use regex::Regex;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let lines = fs::read_to_string("puzzle_input").unwrap();
    let lines = lines.split("\n").map(|l| l.trim()).filter(|l| l.len() > 0);

    let re = Regex::new(r"([0-9]+),([0-9]+)").unwrap();
    let mut line_points: Vec<Vec<Point>> = Vec::new();
    for (i, line) in lines.enumerate() {
        line_points.push(Vec::new());

        for cap in re.captures_iter(line) {
            line_points[i].push(Point { x: cap[1].parse().unwrap(), y: cap[2].parse().unwrap() });
        }
    }

    let mut map: HashMap<Point, bool> = HashMap::new();
    let mut max_y = 0;
    for points in line_points {
        for pair in points.windows(2) {
            for x in min(pair[0].x, pair[1].x)..=max(pair[0].x, pair[1].x) {
                for y in min(pair[0].y, pair[1].y)..=max(pair[0].y, pair[1].y) {
                    map.insert(Point { x, y }, true);
                    if y > max_y {
                        max_y = y;
                    }
                }
            }
        }
    }

    println!("----- Puzzle 1 -----");
    puzzle_1(map.clone(), max_y);

    println!("----- Puzzle 2 -----");
    puzzle_2(map, max_y);
}

fn puzzle_1(mut map: HashMap<Point, bool>, max_y: usize) {
    let mut settled = 0;

    let mut cur_pos = Point { x: 500, y: 0 };
    loop {
        if !map.get(&Point { x: cur_pos.x, y: cur_pos.y + 1 }).unwrap_or(&false) {
            cur_pos.y += 1;

            if cur_pos.y > max_y {
                break;
            }
        }
        else if !map.get(&Point { x: cur_pos.x - 1, y: cur_pos.y + 1 }).unwrap_or(&false) {
            cur_pos.y += 1;
            cur_pos.x -= 1;
        }
        else if !map.get(&Point { x: cur_pos.x + 1, y: cur_pos.y + 1 }).unwrap_or(&false) {
            cur_pos.y += 1;
            cur_pos.x += 1;
        }
        else {
            settled += 1;
            map.insert(cur_pos, true);
            cur_pos = Point { x: 500, y: 0 };
        }
    }

    println!("Settled: {}", settled);
}

fn puzzle_2(mut map: HashMap<Point, bool>, max_y: usize) {
    let mut settled = 0;

    let mut cur_pos = Point { x: 500, y: 0 };
    loop {
        if cur_pos.y >= max_y + 1 {
            settled += 1;
            map.insert(cur_pos, true);
            cur_pos = Point { x: 500, y: 0 };
        }

        if !map.get(&Point { x: cur_pos.x, y: cur_pos.y + 1 }).unwrap_or(&false) {
            cur_pos.y += 1;
        }
        else if !map.get(&Point { x: cur_pos.x - 1, y: cur_pos.y + 1 }).unwrap_or(&false) {
            cur_pos.y += 1;
            cur_pos.x -= 1;
        }
        else if !map.get(&Point { x: cur_pos.x + 1, y: cur_pos.y + 1 }).unwrap_or(&false) {
            cur_pos.y += 1;
            cur_pos.x += 1;
        }
        else {
            settled += 1;
            map.insert(cur_pos, true);

            if cur_pos.x == 500 && cur_pos.y == 0 {
                break;
            }

            cur_pos = Point { x: 500, y: 0 };
        }
    }

    println!("Settled: {}", settled);
}
