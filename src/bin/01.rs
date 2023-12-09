advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split('\n').filter(|f| !f.is_empty()).collect();
    let numbers: Vec<String> = lines
        .iter()
        .map(|f| f.chars().filter(|f| f.is_numeric()).collect())
        .collect();
    let first_last: Vec<String> = numbers
        .iter()
        .map(|f| format!("{}{}", f.chars().next().unwrap(), f.chars().last().unwrap()))
        .collect();

    let s: u32 = first_last.iter().map(|f| f.parse::<u32>().unwrap()).sum();

    Some(s)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines: Vec<String> = input
        .split('\n')
        .filter(|f| !f.is_empty())
        .map(|f| f.to_string())
        .collect();

    let replacements = [
        ["zero", "0o"],
        ["one", "o1e"],
        ["two", "t2"],
        ["three", "t3e"],
        ["four", "4"],
        ["five", "5e"],
        ["six", "6"],
        ["seven", "7n"],
        ["eight", "e8t"],
        ["nine", "9e"],
    ];

    for [old, new] in replacements.iter() {
        lines = lines.iter().map(|f| f.replace(old, new)).collect()
    }

    let numbers: Vec<String> = lines
        .iter()
        .map(|f| f.chars().filter(|f| f.is_numeric()).collect())
        .collect();

    let first_last: Vec<String> = numbers
        .iter()
        .map(|f| format!("{}{}", f.chars().next().unwrap(), f.chars().last().unwrap()))
        .collect();

    let s: u32 = first_last.iter().map(|f| f.parse::<u32>().unwrap()).sum();

    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
