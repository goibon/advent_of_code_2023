use std::collections::HashSet;

fn part_1(input: &str) -> u32 {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(_, game_values)| game_values.split_once(" | "))
        .map(|(winning_numbers, your_numbers)| {
            (
                winning_numbers
                    .split(' ')
                    .filter_map(|number| number.parse().ok())
                    .collect::<HashSet<u32>>(),
                your_numbers,
            )
        })
        .map(|(winning_numbers, your_numbers)| {
            (
                winning_numbers,
                your_numbers
                    .split(' ')
                    .filter_map(|number| number.parse::<u32>().ok()),
            )
        })
        .map(|(winning_numbers, your_numbers)| {
            your_numbers
                .filter(move |your_number| winning_numbers.contains(your_number))
                .fold(
                    0,
                    |accumulator, _| if accumulator == 0 { 1 } else { accumulator * 2 },
                )
        })
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path).unwrap();

    let part_1_result = part_1(&input);
    println!("Day 4 Part 1: {}", part_1_result);
}
