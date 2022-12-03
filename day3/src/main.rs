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
       let mut common = HashSet::<u8>::new();

       for sack in group.iter() {
           if common.len() == 0 {
               common = sack.left.union(&sack.right).map(|i| i.clone()).collect();
           }
           else {
               let int = sack.left.union(&sack.right).map(|i| i.clone()).collect();
               common = common.intersection(&int).map(|i| i.clone()).collect();
           }
       }

       if common.len() == 1 {
            for badge in common.iter() {
                sum += get_point(*badge) as usize;
            }
       }
       else {
           println!("Error with common {:?}", common);
       }
   }

   println!("Sum: {}", sum);
}
