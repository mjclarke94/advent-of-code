advent_of_code::solution!(9);

fn parse_input(input: &str) -> Vec<Box<Sequence>> {
    let input_lines: Vec<&str> = input.split('\n').filter(|f| f.len() > 0).collect();
    let vectors: Vec<Vec<isize>> = input_lines
        .iter()
        .map(|f| {
            f.split_ascii_whitespace()
                .map(|f| f.parse::<isize>().unwrap())
                .collect()
        })
        .collect();
    let sequences: Vec<Box<Sequence>> = vectors
        .iter()
        .map(|f| Sequence::from_vec(f.to_vec()))
        .collect();
    sequences
}

pub fn part_one(input: &str) -> Option<isize> {
    let sequences = parse_input(input);
    let next: Vec<isize> = sequences.iter().map(|f| f.next_value()).collect();

    Some(next.iter().sum())
}

pub fn part_two(input: &str) -> Option<isize> {
    let sequences = parse_input(input);
    let previous: Vec<isize> = sequences.iter().map(|f| f.previous_value()).collect();

    Some(previous.iter().sum())
}

#[allow(dead_code)]
#[derive(Debug)]
enum Sequence {
    NonZero {
        values: Vec<isize>,
        depth: usize,
        differential: Box<Sequence>,
    },
    Zero {
        depth: usize,
    },
}

impl Sequence {
    fn from_vec(values: Vec<isize>) -> Box<Sequence> {
        if values.iter().all(|f| *f == 0) {
            Box::new(Sequence::Zero { depth: 0 })
        } else {
            let diff = Self::get_differential(&values, 0);
            Box::new(Sequence::NonZero {
                values,
                depth: 0,
                differential: diff,
            })
        }
    }

    fn get_differential(values: &Vec<isize>, depth: usize) -> Box<Sequence> {
        let diff: Vec<isize> = values
            .windows(2)
            .filter_map(|f| match f {
                [f, g] => Some(g - f),
                _ => None,
            })
            .collect();

        match &diff.iter().all(|f| *f == 0) {
            true => Box::new(Sequence::Zero { depth: depth + 1 }),
            false => Box::new(Sequence::NonZero {
                values: diff.clone(),
                depth: depth + 1,
                differential: Self::get_differential(&diff, depth + 1),
            }),
        }
    }

    fn next_value(&self) -> isize {
        match self {
            Sequence::Zero { .. } => 0,
            Sequence::NonZero {
                values: val,
                differential: diff,
                ..
            } => {
                // dbg!(&val);
                // dbg!(&diff);
                val.iter().last().unwrap() + diff.next_value()
            }
        }
    }

    fn previous_value(&self) -> isize {
        match self {
            Sequence::Zero { .. } => 0,
            Sequence::NonZero {
                values: val,
                differential: diff,
                ..
            } => {
                // dbg!(&val);
                // dbg!(&diff);
                val.iter().next().unwrap() - diff.previous_value()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
