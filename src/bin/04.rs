advent_of_code::solution!(4);

use anyhow::Result;

use std::collections::HashSet;

#[derive(Debug)]
struct Scratchcard {
    winning_numbers: HashSet<u8>,
    card_numbers: HashSet<u8>,
    count: usize,
}

impl Scratchcard {
    fn score(&self) -> usize {
        let n: usize = self.matches();
        match n {
            1.. => 2_usize.pow((n - 1) as u32),
            _ => 0,
        }
    }

    fn matches(&self) -> usize {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .count()
    }

    fn incr_count(&mut self, n: usize) {
        self.count += n;
    }
}

fn parse_input(input: &str) -> Result<Vec<Scratchcard>> {
    let lines: Vec<&str> = input
        .split('\n')
        .filter(|f| !f.is_empty())
        .map(|f| f.strip_prefix("Card ").unwrap())
        .collect();
    let mut scratchcards: Vec<Scratchcard> = Vec::new();

    for line in lines.iter() {
        let (_, c) = line.split_once(':').unwrap();
        let (win, nums) = c.split_once('|').unwrap();

        let w: HashSet<u8> = win.split_whitespace().map(|f| f.parse().unwrap()).collect();
        let n: HashSet<u8> = nums
            .split_whitespace()
            .map(|f| f.parse().unwrap())
            .collect();

        scratchcards.push(Scratchcard {
            count: 1,
            winning_numbers: w,
            card_numbers: n,
        })
    }
    Ok(scratchcards)
}

pub fn part_one(input: &str) -> Option<usize> {
    let cards = parse_input(input).unwrap();
    let scores: Vec<usize> = cards.iter().map(|f| f.score()).collect();

    Some(scores.iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cards: Vec<Scratchcard> = parse_input(input).unwrap();

    for i in 0..(cards.len()) {
        let c = cards.get(i).unwrap();

        let score = c.matches();
        let count = c.count;

        for j in i + 1..i + score + 1 {
            cards.get_mut(j).unwrap().incr_count(count);
        }
    }
    let total_cards: usize = cards.iter().map(|f| f.count).sum();

    Some(total_cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
