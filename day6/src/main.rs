use std::fs;
use std::collections::HashSet;

fn main() {
    let lines = fs::read_to_string("puzzle_input").unwrap();
    let line = lines.split("\n").next().unwrap().trim();

    println!("----- Puzzle 1 -----");
    puzzle_1(&line);

    println!("----- Puzzle 2 -----");
    puzzle_2(&line);
}

fn puzzle_1(line: &str) {
    let mut start = 0;
    let vec: Vec<char> = line.chars().collect();

    for win in (&vec[..]).windows(4) {
        let mut seen: HashSet<char> = HashSet::new();

        let mut unique = true;
        for chr in win {
            if !seen.insert(*chr) {
                unique = false;
                break;
            }
        }

        if unique {
            break; 
        }
        start += 1;
    }

    println!("{}", start + 4);
}

fn puzzle_2(line: &str) {
    let mut start = 0;
    let vec: Vec<char> = line.chars().collect();

    for win in (&vec[..]).windows(14) {
        let mut seen: HashSet<char> = HashSet::new();

        let mut unique = true;
        for chr in win {
            if !seen.insert(*chr) {
                unique = false;
                break;
            }
        }

        if unique {
            break; 
        }
        start += 1;
    }

    println!("{}", start + 14);
}
