#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn part_1(input: &str) -> u32 {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.split_once(':'))
        .filter_map(|(game_id_part, game_values)| {
            game_id_part
                .split_once(' ')
                .and_then(|(_, id)| id.parse::<u32>().map(|id| (id, game_values)).ok())
        })
        .map(|(id, game_values)| {
            (
                id,
                game_values
                    .split(';')
                    .map(|round| {
                        round
                            .split(", ")
                            .filter_map(|cube_values| {
                                cube_values
                                    .trim()
                                    .split_once(' ')
                                    .and_then(|(count, color)| {
                                        if let Ok(count) = count.parse::<u32>() {
                                            Some((count, color))
                                        } else {
                                            None
                                        }
                                    })
                                    .and_then(|(count, color)| match color {
                                        "red" => Some((count, Color::Red)),
                                        "green" => Some((count, Color::Green)),
                                        "blue" => Some((count, Color::Blue)),
                                        _ => None,
                                    })
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .filter_map(|(game_id, rounds)| {
            if rounds.iter().flatten().any(|(count, color)| match color {
                Color::Red => count > &12,
                Color::Green => count > &13,
                Color::Blue => count > &14,
            }) {
                None
            } else {
                Some(game_id)
            }
        })
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path).unwrap();
    let part_1_result = part_1(&input);
    println!("Day 2 Part 1: {}", part_1_result);
}
