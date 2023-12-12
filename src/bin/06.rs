use num_integer::sqrt;
use std::iter::zip;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<IntSize> {
    let races: Vec<Race> = parse_bad_kerning(input);
    Some(races.iter().map(|f| f.win_chances()).product())
}

pub fn part_two(input: &str) -> Option<IntSize> {
    let race = parse_correctly(input);
    Some(race.win_chances())
}

#[derive(Debug, PartialEq)]
struct Race {
    race_time: IntSize,
    record_distance: IntSize,
}

impl Race {
    fn get_draws(&self) -> (IntSize, IntSize) {
        let high = (self.race_time
            + sqrt(self.race_time * self.race_time - (4 * self.record_distance)))
            / 2;
        (self.race_time - high, high)
    }

    fn get_distance(&self, holdtime: IntSize) -> IntSize {
        (self.race_time - holdtime) * holdtime
    }

    fn win_chances(&self) -> IntSize {
        let (mut low, mut high) = self.get_draws();

        if self.get_distance(low) == self.record_distance {
            low += 1
        }

        if self.get_distance(high) == self.record_distance {
            high -= 1
        }

        (high - low) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(71503));
    }
    #[test]
    fn test_part_one_parse() {
        let races = parse_bad_kerning(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            *races.iter().last().unwrap(),
            Race {
                race_time: 30,
                record_distance: 200
            }
        );
    }

    #[test]
    fn test_part_two_parse() {
        let race = parse_correctly(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            race,
            Race {
                race_time: 71530,
                record_distance: 940200
            }
        );
    }
}

type IntSize = u128;

fn parse_bad_kerning(input: &str) -> Vec<Race> {
    let lines: Vec<Vec<&str>> = input
        .split('\n')
        .filter(|f| !f.is_empty())
        .map(|f| f.split_ascii_whitespace().collect::<Vec<&str>>())
        .collect();

    let times = &lines[0][1..];
    let distances = &lines[1][1..];
    let races: Vec<Race> = zip(times, distances)
        .map(|(t, d)| (t.parse::<IntSize>().unwrap(), d.parse::<IntSize>().unwrap()))
        .map(|(t, d)| Race {
            race_time: (t),
            record_distance: (d),
        })
        .collect();
    races
}

fn parse_correctly(input: &str) -> Race {
    let lines: Vec<Vec<&str>> = input
        .split('\n')
        .filter(|f| !f.is_empty())
        .map(|f| f.split_ascii_whitespace().collect::<Vec<&str>>())
        .collect();

    let time_parts = &lines[0][1..];
    let distance_parts = &lines[1][1..];

    let time = time_parts
        .iter()
        .fold(String::new(), |l, r| l + r)
        .parse::<IntSize>()
        .unwrap();
    let distance = distance_parts
        .iter()
        .fold(String::new(), |l, r| l + r)
        .parse::<IntSize>()
        .unwrap();

    Race {
        race_time: time,
        record_distance: distance,
    }
}
