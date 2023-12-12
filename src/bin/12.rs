advent_of_code::solution!(12);

use std::str::FromStr;


use itertools::Itertools;


#[derive(Debug)]
enum SpringError {
    InvalidChar
}

#[derive(Debug, PartialEq)]
enum Spring {
    Unknown,
    Damaged,
    Operational
}

impl TryFrom<char> for Spring {
    type Error = SpringError;
    fn try_from(value: char) -> Result<Self, Self::Error> {

        match value {
            '.' => Ok(Spring::Operational),
            '?' => Ok(Spring::Unknown),
            '#' => Ok(Spring::Damaged),
            _ => Err(SpringError::InvalidChar)
        }
        
    }
}

#[derive(Debug)]
enum RowError {

}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    contiguous_damaged: Vec<usize>,
    unknown: usize,
    missing: usize
}

impl FromStr for Row {
    type Err = RowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (spring_string, damage) = s.split_once(' ').unwrap();

        let springs: Vec<Spring> = spring_string.chars().map(|f| Spring::try_from(f).unwrap()).collect();
        let contiguous_damaged: Vec<usize> = damage.split(',').map(|f| f.parse().unwrap()).collect();

        let unknown: usize = springs.iter().filter(|f| **f == Spring::Unknown).count();
        let missing: usize = &contiguous_damaged.iter().sum() - springs.iter().filter(|f| **f == Spring::Damaged).count();

        Ok(Row {springs, contiguous_damaged, unknown, missing})
    }
}

impl Row {
    fn validate_ordering(&self, order: Vec<bool>) -> bool {

        let mut unknown_idx: usize = 0;
        let mut contiguous_count: usize = 0;
        let mut contiguous: Vec<usize> = vec![];

        for spring in self.springs.iter() {
            match spring {
                Spring::Damaged => {contiguous_count += 1},
                Spring::Unknown if order[unknown_idx] => {contiguous_count += 1; unknown_idx += 1}, //damaged
                Spring::Unknown => {
                    if contiguous_count != 0 {
                        contiguous.push(contiguous_count);
                        contiguous_count = 0
                    }
                    unknown_idx += 1}, //operational
                Spring::Operational => {
                    if contiguous_count != 0 {
                        contiguous.push(contiguous_count);
                        contiguous_count = 0
                    }
                }                
            }
        }

        if contiguous_count != 0 {contiguous.push(contiguous_count);}

        contiguous == self.contiguous_damaged

    }
    
    fn valid_orderings(&self) -> usize {
        fn combinations(n: usize, m: usize) -> impl Iterator<Item = Vec<bool>> {
            (0..n).combinations(m).map(move |indices| {
                (0..n).map(|i| indices.contains(&i)).collect::<Vec<_>>()
            })
        }



        combinations(self.unknown, self.missing).map(|f| self.validate_ordering(f)).filter(|f| *f).count()


    }


}


pub fn part_one(input: &str) -> Option<usize> {

    let a: Vec<Row> = input.split('\n').filter(|f| !f.is_empty()).map(|f| Row::from_str(f).unwrap()).collect();

    
    Some(a.iter().map(|f| f.valid_orderings()).sum())
    
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
