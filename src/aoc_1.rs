use std::fs;

pub fn aoc_1() {
    println!("Day 1");
    let content = fs::read_to_string("aoc-1.txt").expect("Something went wrong while reading file");
    let number_strings = content.split("\n");
    let mut number_vec: Vec<usize> = Vec::new();
    for num_str in number_strings.into_iter() {
        let num: usize = num_str.parse().unwrap();
        number_vec.push(num);
    }
    let max = *number_vec.iter().max().unwrap();
    let mut numbers = vec![false; max + 1];

    for value in number_vec.iter() {
        numbers[*value] = true;
    }

    for number in number_vec.iter() {
        let searched = 2020 - *number;
        if numbers[searched] {
            println!("Part 1: {} * {} = {}", number, searched, number * searched);
            break
        }
    }

    for number_1 in number_vec.iter() {
        for number_2 in number_vec.iter() {
            for number_3 in number_vec.iter() {
                if number_1 + number_2 + number_3 == 2020 {
                    println!("Part 2: {} * {} * {} = {}", number_1, number_2, number_3, number_1 * number_2 * number_3);
                    return;
                }
            }
        }
    }
}