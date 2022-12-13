use std::fs;
use pathfinding::prelude::bfs;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug, Ord, PartialOrd)]
struct Point {
    x: isize,
    y: isize
}

impl Point {
    fn neighbors(&self, map: &Vec<Vec<u8>>) -> Vec<Point> {
        let cur_height = map[self.y as usize][self.x as usize];

        let mut neighbors: Vec<Point> = Vec::new();
        let to_check = [Point { x: self.x, y: self.y - 1 }, Point { x: self.x, y: self.y + 1 }, Point { x: self.x - 1, y: self.y }, Point { x: self.x + 1, y: self.y }];
        for check in to_check {
            if let Some(row) = map.get(check.y as usize) {
                if let Some(height) = row.get(check.x as usize) {
                    if *height <= cur_height + 1 {
                        neighbors.push(check);
                    }
                }
            }
        }

        neighbors
    }
}

fn main() {
    let lines = fs::read_to_string("puzzle_input").unwrap();
    let lines = lines.split("\n").map(|l| l.trim()).filter(|l| l.len() > 0);

    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut start_point = Point { x: 0, y: 0 };
    let mut end_point = Point { x: 0, y: 0 };
    
    for (i, line) in lines.enumerate() {
        map.push(Vec::new());

        for (j, c) in line.chars().enumerate() {
            map[i].push(match c {
                'S' => { start_point = Point { x: j as isize, y: i as isize }; 0 },
                'E' => { end_point = Point { x: j as isize, y: i as isize }; 'z' as u8 - 'a' as u8 },
                chr => chr as u8 - 'a' as u8
            });
        }
    }

    println!("----- Puzzle 1 -----");
    puzzle_1(start_point, end_point, &map);

    println!("----- Puzzle 2 -----");
    puzzle_2(end_point, &map);
}

fn puzzle_1(start: Point, end: Point, map: &Vec<Vec<u8>>) {
    let res = bfs(&start, |p| p.neighbors(map), |p| *p == end).unwrap();
    println!("Dist: {}", res.len() - 1);
}

fn puzzle_2(end: Point, map: &Vec<Vec<u8>>) {
    let mut min = usize::MAX;

    for (i, row) in map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 0 {
                if let Some(res) = bfs(&Point { x: j as isize, y: i as isize }, |p| p.neighbors(map), |p| *p == end) {
                    if res.len() - 1 < min {
                        min = res.len() - 1;
                    }
                }
            }
        }
    }

    println!("Min: {}", min);
}
