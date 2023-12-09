advent_of_code::solution!(7);

use counter::Counter;
use std::str::FromStr;

#[derive(Debug)]
enum Errors {
    ParsingError,
    CardCountError,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
enum Card {
    WildJack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Result<Card, Errors> {
        match c {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            '1' => Ok(Card::WildJack),
            _ => Err(Errors::ParsingError),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Rank {
    HighCard(Card, Card, Card, Card, Card),
    Pair(Card, Card, Card, Card, Card),
    TwoPair(Card, Card, Card, Card, Card),
    ThreeOfAKind(Card, Card, Card, Card, Card),
    FullHouse(Card, Card, Card, Card, Card),
    FourOfAKind(Card, Card, Card, Card, Card),
    FiveOfAKind(Card, Card, Card, Card, Card),
}

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq)]
struct Hand {
    rank: Rank,
    bid: usize,
}

impl FromStr for Hand {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_c, bid) = s.split_once(' ').unwrap();
        let cards: Vec<Card> = card_c
            .chars()
            .map(|f| Card::from_char(f).unwrap())
            .collect();
        let counts: Counter<&Card> = cards.iter().collect::<Counter<_>>();

        let rank: Result<Rank, Errors> = match counts.most_common_ordered()[..] {
            [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => Ok(Rank::HighCard(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            )),
            [(_, 2), (_, 1), (_, 1), (_, 1)] => {
                Ok(Rank::Pair(cards[0], cards[1], cards[2], cards[3], cards[4]))
            }
            [(_, 2), (_, 2), (_, 1)] => Ok(Rank::TwoPair(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            )),
            [(_, 3), (_, 1), (_, 1)] => Ok(Rank::ThreeOfAKind(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            )),
            [(_, 3), (_, 2)] => Ok(Rank::FullHouse(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            )),
            [(_, 4), (_, 1)] => Ok(Rank::FourOfAKind(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            )),
            [(_, 5)] => Ok(Rank::FiveOfAKind(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            )),
            _ => Err(Errors::CardCountError),
        };

        Ok(Hand {
            rank: rank.unwrap(),
            bid: bid.parse().unwrap(),
        })
    }
}
impl Hand {
    fn from_str_wild(s: &str) -> Result<Self, Errors> {
        let (card_c, bid) = s.split_once(' ').unwrap();
        let cards: Vec<Card> = card_c
            .chars()
            .map(|f| match f {
                'J' => '1',
                _ => f,
            })
            .map(|f| Card::from_char(f).unwrap())
            .collect();

        let counts: Counter<&Card> = cards
            .iter()
            .filter(|f| **f != Card::WildJack)
            .collect::<Counter<_>>();
        let rank: Result<Rank, Errors> = match counts.most_common_ordered()[..] {
            [] | [(_, _)] => Ok(Rank::FiveOfAKind(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            )),
            [(_, _), (_, 1)] => Ok(Rank::FourOfAKind(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            )),
            [(_, _), (_, 2)] => Ok(Rank::FullHouse(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            )),
            [(_, _), (_, 1), (_, 1)] => Ok(Rank::ThreeOfAKind(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            )),
            [(_, 2), (_, 2), (_, 1)] => Ok(Rank::TwoPair(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            )),
            [(_, 2), (_, 1), (_, 1), (_, 1)] | [(_, 1), (_, 1), (_, 1), (_, 1)] => {
                Ok(Rank::Pair(cards[0], cards[1], cards[2], cards[3], cards[4]))
            }
            [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => Ok(Rank::HighCard(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            )),
            _ => Err(Errors::CardCountError),
        };

        Ok(Hand {
            rank: rank.unwrap(),
            bid: bid.parse().unwrap(),
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut cards: Vec<Hand> = input
        .split('\n')
        .filter(|f| !f.is_empty())
        .map(|f| Hand::from_str(f).unwrap())
        .collect();

    cards.sort();

    let s: usize = cards
        .iter()
        .enumerate()
        .map(|(index, value)| (value.bid * (index + 1)))
        .sum();

    Some(s)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cards: Vec<Hand> = input
        .split('\n')
        .filter(|f| !f.is_empty())
        .map(|f| Hand::from_str_wild(f).unwrap())
        .collect();

    cards.sort();

    let s: usize = cards
        .iter()
        .enumerate()
        .map(|(index, value)| (value.bid * (index + 1)))
        .sum();

    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = &advent_of_code::template::read_file("examples", DAY);

        let cards: Vec<Hand> = input
            .split('\n')
            .filter(|f| !f.is_empty())
            .map(|f| Hand::from_str(f).unwrap())
            .collect();

        assert_eq!(
            cards[0],
            Hand {
                rank: Rank::HighCard(Card::Two, Card::Three, Card::Four, Card::Five, Card::Ace),
                bid: 1
            }
        );
    }

    #[test]
    fn test_parse_wild() {
        let input = &advent_of_code::template::read_file("examples", DAY);

        let cards: Vec<Hand> = input
            .split('\n')
            .filter(|f| !f.is_empty())
            .map(|f| Hand::from_str_wild(f).unwrap())
            .collect();

        assert_eq!(
            cards[1],
            Hand {
                rank: Rank::ThreeOfAKind(
                    Card::Queen,
                    Card::Two,
                    Card::King,
                    Card::WildJack,
                    Card::WildJack
                ),
                bid: 13
            }
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6592));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6839));
    }
}
