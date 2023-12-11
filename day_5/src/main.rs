fn parse_map(map: &str) -> Vec<(u32, u32, u32)> {
    map.split_once('\n')
        .map(|(_, values)| values)
        .unwrap()
        .split('\n')
        .map(|line| {
            let [destination_start, source_start, range_length] = line
                .splitn(3, ' ')
                .filter_map(|value| value.parse::<u32>().ok())
                .collect::<Vec<_>>()[..]
            else {
                todo!()
            };
            (destination_start, source_start, range_length)
        })
        .collect::<Vec<_>>()
}

fn find_row_with_range_definition(
    needle: u32,
    haystack: &[(u32, u32, u32)],
) -> Option<(u32, u32, u32)> {
    let mut filtered_haystack = haystack
        .iter()
        .filter(|(_, candidate, range_length)| {
            candidate <= &needle && candidate + range_length > needle
        })
        .collect::<Vec<_>>();
    filtered_haystack.sort_unstable_by_key(|tuple| tuple.1);
    if let Some((x, y, z)) = filtered_haystack.last() {
        Some((*x, *y, *z))
    } else {
        None
    }
}

fn map_to_destination_range(needle: u32, haystack: &[(u32, u32, u32)]) -> u32 {
    if let Some((destination_range_start, source_range_start, _)) =
        find_row_with_range_definition(needle, haystack)
    {
        if destination_range_start > source_range_start {
            needle + (destination_range_start - source_range_start)
        } else {
            needle - (source_range_start - destination_range_start)
        }
    } else {
        needle
    }
}

fn part_1(input: &str) -> u32 {
    let maps = input
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let mut maps_iter = maps.iter();

    let seeds = maps_iter
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|seed| seed.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let seed_to_soil_string = parse_map(maps_iter.next().unwrap());
    let soil_to_fertilizer_string = parse_map(maps_iter.next().unwrap());
    let fertilizer_to_water_string = parse_map(maps_iter.next().unwrap());
    let water_to_light_string = parse_map(maps_iter.next().unwrap());
    let light_to_temperature_string = parse_map(maps_iter.next().unwrap());
    let temperature_to_humidity_string = parse_map(maps_iter.next().unwrap());
    let humidity_to_location_string = parse_map(maps_iter.next().unwrap());

    let mut locations = seeds
        .iter()
        .map(|&needle| map_to_destination_range(needle, &seed_to_soil_string))
        .map(|needle| map_to_destination_range(needle, &soil_to_fertilizer_string))
        .map(|needle| map_to_destination_range(needle, &fertilizer_to_water_string))
        .map(|needle| map_to_destination_range(needle, &water_to_light_string))
        .map(|needle| map_to_destination_range(needle, &light_to_temperature_string))
        .map(|needle| map_to_destination_range(needle, &temperature_to_humidity_string))
        .map(|needle| map_to_destination_range(needle, &humidity_to_location_string))
        .collect::<Vec<_>>();
    locations.sort_unstable();
    *locations.first().unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path).unwrap();

    let part_1_result = part_1(&input);
    println!("Day 5 Part 1: {}", part_1_result);
}
