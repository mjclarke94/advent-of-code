use std::{cmp, ops::Add, str::FromStr};

advent_of_code::solution!(2);

#[derive(Debug)]
enum RoundErrors {}

#[derive(Debug, Clone, Copy)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Round {
    type Err = RoundErrors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut red, mut green, mut blue): (u32, u32, u32) = (0, 0, 0);

        for substring in s.split(", ") {
            match substring.split_once(' ').unwrap() {
                (i, "red") => red += i.parse::<u32>().unwrap(),
                (i, "green") => green += i.parse::<u32>().unwrap(),
                (i, "blue") => blue += i.parse::<u32>().unwrap(),
                _ => {}
            }
        }

        Ok(Round { red, green, blue })
    }
}

impl Add for Round {
    type Output = Round;
    fn add(self, rhs: Self) -> Self::Output {
        Round {
            red: cmp::max(self.red, rhs.red),
            green: cmp::max(self.green, rhs.green),
            blue: cmp::max(self.blue, rhs.blue),
        }
    }
}

impl Round {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    round_id: u32,
    optimal: Round,
}

impl FromStr for Game {
    type Err = RoundErrors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, rounds) = s.split_once(": ").unwrap();

        let round_id: u32 = game.split_once(' ').unwrap().1.parse().unwrap();
        let rounds: Vec<Round> = rounds
            .split("; ")
            .map(|f| Round::from_str(f).unwrap())
            .collect();

        let optimal = rounds.iter().fold(
            Round {
                red: 0,
                green: 0,
                blue: 0,
            },
            |a, b| a + *b,
        );

        Ok(Game { round_id, optimal })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split('\n').filter(|f| !f.is_empty()).collect();
    let rounds: Vec<Game> = lines.iter().map(|f| Game::from_str(f).unwrap()).collect();

    let winning_rounds: Vec<&Game> = rounds
        .iter()
        .filter(|f| (f.optimal.red < 13) & (f.optimal.green < 14) & (f.optimal.blue < 15))
        .collect();

    let s: u32 = winning_rounds.iter().map(|f| f.round_id).sum();

    Some(s)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split('\n').filter(|f| !f.is_empty()).collect();
    let rounds: Vec<Game> = lines.iter().map(|f| Game::from_str(f).unwrap()).collect();
    let power: u32 = rounds.iter().map(|f| f.optimal.power()).sum();

    Some(power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
