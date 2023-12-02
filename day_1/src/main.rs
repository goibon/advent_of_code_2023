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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path).unwrap();
    let part_1_result = part_1(&input);
    println!("Day 1 Part 1: {}", part_1_result);
}
