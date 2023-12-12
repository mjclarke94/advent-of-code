advent_of_code::solution!(12);

use std::str::FromStr;




#[derive(Debug)]
enum SpringError {
    InvalidChar
}

#[derive(Debug, PartialEq, Clone)]
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
    groups: Vec<usize>,
}

impl FromStr for Row {
    type Err = RowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (spring_string, damage) = s.split_once(' ').unwrap();

        let springs: Vec<Spring> = spring_string.chars().map(|f| Spring::try_from(f).unwrap()).collect();
        let groups: Vec<usize> = damage.split(',').map(|f| f.parse().unwrap()).collect();

        Ok(Row {springs, groups})
    }
}

impl Row {
    fn valid_orderings(&self) -> usize {
        Self::recurse_ordering(&self.springs[..], &self.groups[..])
    }

    fn recurse_ordering(springs: &[Spring], groups: &[usize]) -> usize {

        if springs.is_empty() {
            return match groups.is_empty() {
                true => 1,
                false => 0
            }
        }

        match springs[0] {
            Spring::Operational => Row::recurse_ordering(&springs[1..], groups),
            Spring::Unknown => {

                let mut vec_working: Vec<Spring> = Vec::with_capacity(springs.len());
                let mut vec_damaged: Vec<Spring> = vec_working.clone();

                vec_working.push(Spring::Operational);
                vec_damaged.push(Spring::Damaged);  

                vec_damaged.extend_from_slice(&springs[1..]);
                vec_working.extend_from_slice(&springs[1..]);


                let working = Row::recurse_ordering(&vec_working[..], groups);
                let damaged = Row::recurse_ordering(&vec_damaged[..], groups);
                working + damaged
            },
            Spring::Damaged => {
                match (springs.len(), groups.len()) {
                    // Check if out of DoF
                    (_, 0) => 0, // No groups remaining to consume damaged springs - # | ()
                    (s, _) if s < groups[0]  => 0, // Insufficient springs to satisfy next group # e.g. "##" (3, ..)
                    (_, _) if springs[..groups[0]].iter().any(|f| f == &Spring::Operational)=> 0, // Non broken spring in group e.g. "##." (3, 1)

                    // Ensure group doesn't overrun
                    // If Damaged, zero. If 

                    (s, 2..) if ((s < groups[0] + 1) || (springs[groups[0]] == Spring::Damaged)) => 0, // || short circuits
                    (_, 2..) => Self::recurse_ordering(&springs[groups[0] + 1..], &groups[1..]),
                    (_,_) => Self::recurse_ordering(&springs[groups[0]..], &groups[1..])
                }
            }
            
        }

    
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
