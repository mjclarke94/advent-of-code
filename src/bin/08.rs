advent_of_code::solution!(8);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u128> {
    let (instructions, map) = parse_input(input).unwrap();

    let i = walk_network(&instructions, &map);

    Some(i)
}

pub fn part_two(input: &str) -> Option<u128> {
    let (instructions, map) = parse_input(input).unwrap();

    let mut keys: Vec<&&str> = map.keys().filter(|f| f.ends_with('A')).collect();

    let mut lengths: Vec<u128> = vec![];
    let mut i;
    for key in keys.iter_mut() {
        i = 0;

        for dir in instructions.iter().cycle() {
            if key.ends_with('Z') {
                break;
            }
            i += 1;
            *key = match *dir {
                Direction::Left => &map.get(*key).unwrap()[0],
                Direction::Right => &map.get(*key).unwrap()[1],
            }
        }
        // dbg!(key, i);
        lengths.push(i);
    }

    Some(lengths.iter().fold(lengths[0], |acc, ins| lcm(acc, *ins)))

    // Some(i);
}

fn lcm(first: u128, second: u128) -> u128 {
    first * second / gcd(first, second)
}

fn gcd(first: u128, second: u128) -> u128 {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[derive(Debug)]
enum Errors {
    MappingParsingError,
}

fn mapping_from_str(line: &str) -> Result<(&str, [&str; 2]), Errors> {
    let parts: Vec<&str> = line
        .split(|c| c == '=' || c == '(' || c == ',' || c == ')')
        .filter(|word| !word.trim().is_empty())
        .collect();

    if parts.len() != 3 {
        return Err(Errors::MappingParsingError);
    }

    let source = parts[0].trim();
    let left = parts[1].trim();
    let right = parts[2].trim();

    Ok((source, [left, right]))
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> Result<(Vec<Direction>, HashMap<&str, [&str; 2]>), Errors> {
    let lines: Vec<&str> = input.split('\n').collect();

    let mut line_it = lines.iter();

    let instructions: Vec<Direction> = line_it
        .next()
        .unwrap()
        .trim()
        .chars()
        .filter_map(|f| match f {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        })
        .collect();

    let nodes: Vec<(&str, [&str; 2])> = line_it
        .skip(1)
        .map(|f| mapping_from_str(f).unwrap())
        .collect();

    let mut m: HashMap<&str, [&str; 2]> = HashMap::new();

    for (s, d) in nodes.iter() {
        m.entry(*s).or_insert(*d);
    }

    Ok((instructions, m))
}

fn walk_network(instructions: &[Direction], map: &HashMap<&str, [&str; 2]>) -> u128 {
    let mut loc = "AAA";
    let mut i: u128 = 0;

    for dir in instructions.iter().cycle() {
        i += 1;
        loc = match dir {
            Direction::Left => map.get(loc).unwrap()[0],
            Direction::Right => map.get(loc).unwrap()[1],
        };

        if loc == "ZZZ" {
            break;
        }
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 1);

        let (instructions, map) = parse_input(&input).unwrap();

        assert_eq!(instructions, vec![Direction::Right, Direction::Left]);
        assert_eq!(*map.get("AAA").unwrap(), ["BBB", "CCC"]);
    }
    #[test]
    fn test_part_one_direct() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_repeats() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
