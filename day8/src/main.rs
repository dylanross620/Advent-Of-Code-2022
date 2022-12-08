use std::fs;

#[derive(Clone, Debug)]
struct Tree {
    height: usize,
    visible: bool,
}

fn main() {
    let lines = fs::read_to_string("puzzle_input").unwrap();
    let lines: Vec<&str> = lines.split("\n").map(|l| l.trim()).filter(|l| l.len() > 0).collect();

    let mut trees: Vec<Vec<Tree>> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        trees.push(Vec::new());

        let heights: Vec<usize> = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        for (j, height) in heights.iter().enumerate() {
            let on_edge = i == 0 || i == lines.len() - 1 || j == 0 || j == heights.len() - 1;
            trees[i].push(Tree {
                height: *height,
                visible: on_edge,
            });
        }
    }

    println!("----- Puzzle 1 -----");
    puzzle_1(&trees);

    println!("----- Puzzle 2 -----");
    puzzle_2(&trees);
}

fn puzzle_1(trees: &Vec<Vec<Tree>>) {
    let mut trees = trees.clone();

    // Loop through rows
    for row in 1..trees.len() - 1 {
        let mut largest_height = 0;

        for col in 0..trees[row].len() {
            if trees[row][col].height > largest_height {
                largest_height = trees[row][col].height;
                trees[row][col].visible = true;
            }
        }

        largest_height = 0;

        for col in (0..trees[row].len()).rev() {
            if trees[row][col].height > largest_height {
                largest_height = trees[row][col].height;
                trees[row][col].visible = true;
            }
        }
    }

    // Loop through cols
    for col in 1..trees[0].len() - 1 {
        let mut largest_height = 0;

        for row in 0..trees.len() {
            if trees[row][col].height > largest_height {
                largest_height = trees[row][col].height;
                trees[row][col].visible = true;
            }
        }

        largest_height = 0;

        for row in (0..trees.len()).rev() {
            if trees[row][col].height > largest_height {
                largest_height = trees[row][col].height;
                trees[row][col].visible = true;
            }
        }
    }

    // Count
    let mut sum = 0;
    for row in &trees {
        for tree in row {
            if tree.visible {
                sum += 1;
            }
        }
    }

    println!("Visible trees: {}", sum);
}

fn puzzle_2(trees: &Vec<Vec<Tree>>) {
    let mut largest_score = 0;

    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            let cur_tree = &trees[row][col];

            let mut score = 1;
            let mut count = 0;

            // Check left
            for check in (0..col).rev() {
                count += 1;
                if trees[row][check].height >= cur_tree.height {
                    break;
                }
            }

            score *= count;
            count = 0;

            // Check right
            for check in col + 1..trees[row].len() {
                count += 1;
                if trees[row][check].height >= cur_tree.height {
                    break;
                }
            }

            score *= count;
            count = 0;

            // Check up
            for check in (0..row).rev() {
                count += 1;
                if trees[check][col].height >= cur_tree.height {
                    break;
                }
            }

            score *= count;
            count = 0;

            // Check down
            for check in row + 1..trees.len() {
                count += 1;
                if trees[check][col].height >= cur_tree.height {
                    break;
                }
            }

            score *= count;

            if score > largest_score {
                largest_score = score;
            }
        }
    }

    println!("Largest score: {}", largest_score);
}
