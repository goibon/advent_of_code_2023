use regex::Regex;

#[derive(Debug)]
struct PartNumber {
    line_index: usize,
    start: usize,
    end: usize,
    value: u32,
}

fn parse_input_to_2d_vec(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn check_neighbors(part_number: &PartNumber, schematic: &Vec<Vec<char>>) -> bool {
    let row_start = part_number.line_index.checked_sub(1).unwrap_or_default();
    let column_start = part_number.start.checked_sub(1).unwrap_or_default();
    for row in schematic
        .iter()
        .take(part_number.line_index + 2)
        .skip(row_start)
    {
        for character in row.iter().take(part_number.end + 1).skip(column_start) {
            if !character.is_ascii_digit() && character != &'.' {
                return true;
            }
        }
    }

    false
}

fn part_1(input: &str) -> u32 {
    let regex = Regex::new(r"(\d+)").unwrap();
    let schematic = parse_input_to_2d_vec(input);
    input
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
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .filter(|part_number| check_neighbors(part_number, &schematic))
        .map(|part_number| part_number.value)
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path).unwrap();
    let part_1_result = part_1(&input);
    println!("Day 3 Part 1: {}", part_1_result);
}
