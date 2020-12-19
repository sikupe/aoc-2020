use std::fs;

pub(crate) fn aoc_2() {
    println!("Day 2");
    let input = fs::read_to_string("aoc-2.txt").unwrap();
    let lines = input.split("\n");

    let mut correct_part_1 = 0;
    let mut corrupted_part_1 = 0;

    let mut correct_part_2 = 0;
    let mut corrupted_part_2 = 0;

    for line in lines.into_iter() {
        let parts: Vec<&str> = line.split(":").collect();
        let policy: &str = parts.first().unwrap();
        let password = parts.last().unwrap().trim();
        let policy_parts: Vec<&str> = policy.split(" ").collect();
        let range: &str = policy_parts.first().unwrap();
        let range_parts: Vec<&str> = range.split("-").collect();
        let start: usize = range_parts.first().unwrap().parse().unwrap();
        let end: usize = range_parts.last().unwrap().parse().unwrap();

        let character: char = policy_parts.last().unwrap().trim().parse().unwrap();

        let mut count = 0;
        for c in password.chars() {
            if c == character {
                count += 1;
            }
        }

        if start <= count && count <= end {
            correct_part_1 += 1;
        } else {
            corrupted_part_1 += 1;
        }

        // Logical XOR
        let password_chars: Vec<char> = password.chars().collect();
        if (password_chars[start - 1] == character) != (password_chars[end - 1] == character) {
            correct_part_2 += 1;
        } else {
            corrupted_part_2 += 2;
        }
    }

    println!("Part 1: Correct: {}; Corrupted: {}", correct_part_1, corrupted_part_1);

    println!("Part 2: Correct: {}; Corrupted: {}", correct_part_2, corrupted_part_2);
}