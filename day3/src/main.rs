use std::fs;
use std::collections::HashSet;

struct Rucksack {
    left: HashSet<u8>,
    right: HashSet<u8>,
}

fn main() {
    let lines = fs::read_to_string("puzzle_input").unwrap();
    let lines: Vec<&[u8]> = lines.split_terminator("\n").map(|l| l.trim().as_bytes()).collect();

    let sacks: Vec<Rucksack> = lines.iter().map(|line| {
        let mid = (line.len() / 2) as usize;
        Rucksack {
            left: line[..mid].iter().map(|c| c.clone()).collect(),
            right: line[mid..].iter().map(|c| c.clone()).collect(),
        }
    }).collect();

    println!("------ Puzzle 1 -----");
    puzzle_1(&sacks);

    println!("------ Puzzle 2 -----");
    puzzle_2(&sacks);
}

fn get_point(c: u8) -> u8 {
    if c >= 'a' as u8 {
        c - ('a' as u8) + 1
    }
    else {
        c - ('A' as u8) + 27
    }
}

fn puzzle_1(sacks: &Vec<Rucksack>) {
   let mut sum: usize = 0;

   for sack in sacks {
       for item in &sack.left {
           if sack.right.contains(&item) {
               sum += get_point(*item) as usize;
           }
       }
   }

   println!("The sum is {}", sum);
}

fn puzzle_2(sacks: &Vec<Rucksack>) {
   let mut sum: usize = 0;

   for group in sacks.chunks(3) {
       let sack = group[0].left.union(&group[0].right);
       let badge = sack.into_iter()
           .filter(|item| group[1].left.contains(item) || group[1].right.contains(item))
           .filter(|item| group[2].left.contains(item) || group[2].right.contains(item))
           .next()
           .unwrap();

       sum += get_point(*badge) as usize;
   }

   println!("Sum: {}", sum);
}
