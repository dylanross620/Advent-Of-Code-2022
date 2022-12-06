use std::fs;
use regex::Regex;
use std::collections::LinkedList;

fn main() {
    let lines = fs::read_to_string("puzzle_input").unwrap();
    let mut lines: LinkedList<&str> = lines.split("\n").collect();

    let mut crate_lines: Vec<&str> = Vec::new();
    let crate_line_regex = Regex::new(r"^(?:\s[0-9]\s{1,2})+$").unwrap();
    loop {
        let line = lines.pop_front().unwrap();

        if crate_line_regex.is_match(line) {
            // Pop the empty line
            let _ = lines.pop_front();
            break
        }

        crate_lines.push(line);
    }

    let mut piles: Vec<LinkedList<char>> = Vec::new();
    let mut piles_2: Vec<LinkedList<char>> = Vec::new();

    for line in crate_lines {
        for (i, item) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            if piles.len() <= i {
                piles.push(LinkedList::new()); 
                piles_2.push(LinkedList::new());
            } 

            if item[0] == '[' {
                piles[i].push_front(item[1]);
                piles_2[i].push_front(item[1]);
            }
        }
    }

    println!("----- Puzzle 1 -----");
    puzzle_1(&mut piles, &lines);

    println!("----- Puzzle 2 -----");
    puzzle_2(&mut piles_2, &lines);
}

fn puzzle_1(piles: &mut Vec<LinkedList<char>>, lines: &LinkedList<&str>) {
    let move_regex = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();
    for line in lines {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }

        let caps = move_regex.captures(line).unwrap();

        let count: usize = caps.get(1).map_or(0, |c| c.as_str().parse().unwrap());
        let src: usize = caps.get(2).map_or(0, |c| c.as_str().parse().unwrap());
        let dest: usize = caps.get(3).map_or(0, |c| c.as_str().parse().unwrap());

        for _ in 0..count {
            let tmp = piles[src - 1].pop_back();
            if let Some(c) = tmp {
                piles[dest - 1].push_back(c);
            }
        }
    }

    for pile in piles {
        print!("{}", pile.back().unwrap());
    }
    println!("");
}

fn puzzle_2(piles: &mut Vec<LinkedList<char>>, lines: &LinkedList<&str>) {
    let move_regex = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();
    for line in lines {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }


        let caps = move_regex.captures(line).unwrap();

        let count: usize = caps.get(1).map_or(0, |c| c.as_str().parse().unwrap());
        let src: usize = caps.get(2).map_or(0, |c| c.as_str().parse().unwrap());
        let dest: usize = caps.get(3).map_or(0, |c| c.as_str().parse().unwrap());

        let mut moving: LinkedList<char> = LinkedList::new();
        for _ in 0..count {
            let tmp = piles[src - 1].pop_back();
            if let Some(c) = tmp {
                moving.push_front(c);
            }
        }

        for c in moving {
            piles[dest - 1].push_back(c);
        }
    }

    for pile in piles {
        print!("{}", pile.back().unwrap_or(&' '));
    }
    println!("");
}
