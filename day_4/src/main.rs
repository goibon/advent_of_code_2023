use std::collections::{HashMap, HashSet};

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

fn part_2(input: &str) -> u32 {
    let original_cards = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(card_id_part, game_values)| {
            if let Some((_, game_id)) = card_id_part.split_once(' ') {
                game_id
                    .trim()
                    .parse::<u32>()
                    .ok()
                    .map(|card_id| (card_id, game_values))
            } else {
                None
            }
        })
        .filter_map(|(card_id, game_values)| {
            game_values
                .split_once(" | ")
                .map(|(winning_numbers, your_numbers)| (card_id, winning_numbers, your_numbers))
        })
        .map(|(card_id, winning_numbers, your_numbers)| {
            (
                card_id,
                winning_numbers
                    .split(' ')
                    .filter_map(|number| number.parse().ok())
                    .collect::<HashSet<u32>>(),
                your_numbers,
            )
        })
        .map(|(card_id, winning_numbers, your_numbers)| {
            (
                card_id,
                your_numbers
                    .split(' ')
                    .filter_map(|number| number.parse::<u32>().ok())
                    .filter(|number| winning_numbers.contains(number))
                    .count() as u32,
            )
        })
        .collect::<Vec<_>>();

    // key=card id, value=count of how many copies (including original) we own of a given card_id
    let mut card_count_map: HashMap<u32, u32> = HashMap::new();

    original_cards
        .iter()
        .for_each(|(card_id, number_of_winnings)| {
            card_count_map
                .entry(*card_id)
                .and_modify(|entry| *entry += 1)
                .or_insert(1);

            if let Some(card_multiplier) = card_count_map.get(card_id) {
                let to_add = *card_multiplier;
                for x in card_id + 1..=card_id + number_of_winnings {
                    card_count_map
                        .entry(x)
                        .and_modify(|entry| *entry += to_add)
                        .or_insert(to_add);
                }
            }
        });

    card_count_map.values().sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path).unwrap();
    let part_1_result = part_1(&input);
    println!("Day 4 Part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("Day 4 Part 2: {}", part_2_result);
}
