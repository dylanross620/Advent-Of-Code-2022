use std::{fs, collections::HashSet};

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

fn main() {
    let lines = fs::read_to_string("puzzle_input").unwrap();
    let lines: Vec<(char, u8)> = lines.split_terminator("\n").map(|l| {
        let tmp: Vec<&str> = l.trim().split(" ").collect();
        (tmp[0].chars().next().unwrap(), tmp[1].parse::<u8>().unwrap())
    }).collect();

    println!("----- Puzzle 1 -----" );
    puzzle_1(&lines);

    println!("----- Puzzle 2 -----" );
    puzzle_2(&lines);
}

fn puzzle_1(instructions: &Vec<(char, u8)>) {
    let mut head_pos = Point { x: 0, y: 4 };
    let mut tail_pos = Point { x: 0, y: 4 };
    let mut tail_set: HashSet<Point> = HashSet::new();
    tail_set.insert(tail_pos);

    for (dir, dist) in instructions {
        for _ in 0..*dist {
            let delta = match *dir {
                'U' => (0, -1),
                'D' => (0, 1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _ => panic!("Unknown direction {}", *dir)
            };

            head_pos.x += delta.0;
            head_pos.y += delta.1;

            let x_dist = head_pos.x - tail_pos.x;
            let y_dist = head_pos.y - tail_pos.y;

            if x_dist.abs() > 1 || y_dist.abs() > 1 {
                if x_dist.abs() > 0 {
                    tail_pos.x += x_dist.signum();
                }
                if y_dist.abs() > 0 {
                    tail_pos.y += y_dist.signum();
                }
            }

            tail_set.insert(tail_pos);
        }
    }

    println!("Visited count: {}", tail_set.len());
}

fn puzzle_2(instructions: &Vec<(char, u8)>) {
    let mut knots = vec![Point { x: 0, y: 4 }; 10];
    let mut tail_set: HashSet<Point> = HashSet::new();
    tail_set.insert(knots[9]);

    for (dir, dist) in instructions {
        for _ in 0..*dist {
            let delta = match *dir {
                'U' => (0, -1),
                'D' => (0, 1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _ => panic!("Unknown direction {}", *dir)
            };

            knots[0].x += delta.0;
            knots[0].y += delta.1;

            for i in 1..knots.len() {
                let x_dist = knots[i-1].x - knots[i].x;
                let y_dist = knots[i-1].y - knots[i].y;

                if x_dist.abs() > 1 || y_dist.abs() > 1 {
                    if x_dist.abs() > 0 {
                        knots[i].x += x_dist.signum();
                    }
                    if y_dist.abs() > 0 {
                        knots[i].y += y_dist.signum();
                    }
                }
            }

            tail_set.insert(knots[9]);
        }
    }

    println!("Visited count: {}", tail_set.len());
}
