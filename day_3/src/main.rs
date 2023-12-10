use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct PartNumber {
    line_index: usize,
    start: usize,
    end: usize,
    value: u32,
    gears: Vec<(usize, usize)>,
}

fn parse_input_to_2d_vec(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn check_neighbors(
    part_number: &PartNumber,
    schematic: &[Vec<char>],
) -> Option<Vec<(usize, usize)>> {
    let row_start = part_number.line_index.checked_sub(1).unwrap_or_default();
    let column_start = part_number.start.checked_sub(1).unwrap_or_default();
    let mut is_adjacent_to_a_symbol = false;
    let mut gears: Vec<(usize, usize)> = Vec::new();
    schematic
        .iter()
        .take(part_number.line_index + 2)
        .enumerate()
        .skip(row_start)
        .for_each(|(row_index, row)| {
            row.iter()
                .take(part_number.end + 1)
                .enumerate()
                .skip(column_start)
                .for_each(|(character_index, character)| {
                    if !character.is_ascii_digit() && character != &'.' {
                        is_adjacent_to_a_symbol = true;
                    }
                    if character == &'*' {
                        gears.push((row_index, character_index));
                    }
                });
        });
    if is_adjacent_to_a_symbol {
        Some(gears)
    } else {
        None
    }
}

fn part_1_and_2(input: &str) -> (u32, u32) {
    let regex = Regex::new(r"(\d+)").unwrap();
    let schematic = parse_input_to_2d_vec(input);
    let part_numbers_that_touch_symbols = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(line_index, line)| {
            regex.captures_iter(line).filter_map(move |captures| {
                if let Some(sub_match) = captures.get(0) {
                    if let Ok(value) = sub_match.as_str().parse() {
                        Some(PartNumber {
                            start: sub_match.start(),
                            end: sub_match.end(),
                            line_index,
                            value,
                            gears: Vec::new(),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .filter_map(|mut part_number| {
            if let Some(gears) = check_neighbors(&part_number, &schematic) {
                part_number.gears = gears;
                Some(part_number)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let part_1_result = part_numbers_that_touch_symbols
        .iter()
        .map(|part_number| part_number.value)
        .sum();

    let mut gears_touching_part_numbers: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    part_numbers_that_touch_symbols.iter().for_each(|part| {
        part.gears.iter().for_each(|&gear| {
            gears_touching_part_numbers
                .entry(gear)
                .or_default()
                .push(part.value);
        })
    });

    let part_2_result: u32 = gears_touching_part_numbers
        .values()
        .filter_map(|values| {
            if values.len() != 2 {
                None
            } else if let Some(first_value) = values.first() {
                values.get(1).map(|second_value| first_value * second_value)
            } else {
                None
            }
        })
        .sum();

    (part_1_result, part_2_result)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path).unwrap();
    let (part_1_result, part_2_result) = part_1_and_2(&input);
    println!("Day 3 Part 1: {}", part_1_result);
    println!("Day 3 Part 2: {}", part_2_result);
}
