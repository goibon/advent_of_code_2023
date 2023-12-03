use regex::Regex;

fn part_1(input: &str) -> u32 {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let filtered_chars = line
                .chars()
                .filter(|character| character.is_ascii_digit())
                .collect::<Vec<char>>();
            if let Some(first) = filtered_chars.first() {
                filtered_chars.last().map(|last| (*first, *last))
            } else {
                None
            }
        })
        .filter_map(|(first, last)| format!("{}{}", first, last).parse::<u32>().ok())
        .sum()
}

fn parse_number_string_to_digit(string: &str) -> &str {
    match string {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => string,
    }
}

fn part_2(input: &str) -> u32 {
    let regex = Regex::new(r"(?mi)(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();

    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut first_value = "";
            let mut first_value_index = None;
            let mut last_value = "";

            for start in 0..line.len() {
                if let Some(Some(first)) =
                    regex.captures_at(line, start).map(|capture| capture.get(1))
                {
                    if first_value.is_empty() && first_value_index.is_none() {
                        first_value = first.as_str();
                        first_value_index = Some(first.start());
                    } else if first.start() > first_value_index.unwrap() {
                        last_value = first.as_str();
                    }
                }
            }
            (first_value, last_value)
        })
        .map(|(first, last)| {
            (
                parse_number_string_to_digit(first),
                parse_number_string_to_digit(last),
            )
        })
        .filter_map(|(first, last)| {
            if last.is_empty() {
                format!("{}{}", first, first).parse::<u32>().ok()
            } else {
                format!("{}{}", first, last).parse::<u32>().ok()
            }
        })
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path).unwrap();
    let part_1_result = part_1(&input);
    println!("Day 1 Part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("Day 1 Part 2: {}", part_2_result);
}
