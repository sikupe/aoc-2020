use std::fs;

pub fn aoc_3() {
    println!("Day 3");
    let lines = fs::read_to_string("res/aoc-3.txt").unwrap();

    let mut board = [[false; 31]; 323];
    for (l_i, line) in lines.split("\n").into_iter().enumerate() {
        for (c_i, c) in line.chars().into_iter().enumerate() {
            if c == '#' {
                board[l_i][c_i] = true;
            }
        }
    }

    let config = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut all_trees = [0; 5];
    for (i, conf) in config.iter().enumerate() {
        let (right, down) = *conf;
        let mut right_counter = right;
        for down_counter in (down..323).step_by(down) {
            if board[down_counter][right_counter % 31] {
                all_trees[i] += 1;
            }
            right_counter += right;
        }
    }

    let mut product: u128 = 1;
    for trees in all_trees.iter() {
        product *= *trees;
    }
    println!("Part 1: Trees = {}", all_trees[1]);
    println!("Part 2: Tree product = {}", product);
}