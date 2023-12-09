advent_of_code::solution!(3);

use regex::Regex;

#[derive(Debug)]
struct PartNumber {
    value: usize,
    row: usize,
    min_col: usize,
    max_col: usize,
}

#[derive(Debug)]
struct Symbol {
    c: char,
    row: usize,
    col: usize,
}

impl PartNumber {
    fn is_adjacent(&self, s: &Symbol) -> bool {
        let l = self.min_col.saturating_sub(1);
        let r = self.max_col + 1;
        let x = (l..=r).contains(&s.col);

        let u = self.row.saturating_sub(1);
        let b = self.row + 1;
        let y = (u..=b).contains(&s.row);

        x & y
    }

    fn has_adjacent_part(&self, symbols: &[Symbol]) -> bool {
        for s in symbols.iter() {
            if self.is_adjacent(s) {
                return true;
            }
        }
        false
    }
}

impl Symbol {
    fn gear_power(&self, parts: &[PartNumber]) -> Option<usize> {
        let mut p = 1;
        let mut i: u8 = 0;

        for part in parts.iter() {
            if part.is_adjacent(self) {
                i += 1;
                if i > 2 {
                    return None;
                }

                p *= part.value
            }
        }

        match i {
            2 => Some(p),
            _ => None,
        }
    }
}

fn get_parts(input: &str) -> Vec<PartNumber> {
    let re = Regex::new(r"\d+").unwrap();

    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();
    let mut numbers = Vec::<PartNumber>::new();
    for (row, line) in lines.iter().enumerate() {
        for cap in re.captures_iter(line) {
            let m = cap.get(0).unwrap();
            let value: usize = m.as_str().parse().unwrap();
            let (min_col, max_col) = (m.start(), m.end() - 1);

            numbers.push(PartNumber {
                value,
                row,
                min_col,
                max_col,
            })
        }
    }

    numbers
}

fn get_symbols(input: &str) -> Vec<Symbol> {
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();
    let numberless: Vec<Vec<char>> = lines
        .iter()
        .map(|f| {
            f.chars()
                .map(|c| match (c.is_numeric()) | (c == '.') {
                    true => ' ',
                    false => c,
                })
                .collect()
        })
        .collect();

    let mut symbols = Vec::<Symbol>::new();

    for (i, v) in numberless.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            match c {
                ' ' => {}
                _ => symbols.push(Symbol {
                    c: *c,
                    row: i,
                    col: j,
                }),
            }
        }
    }
    symbols
}

pub fn part_one(input: &str) -> Option<usize> {
    let numbers = get_parts(input);
    let symbols = get_symbols(input);

    let s: usize = numbers
        .iter()
        .filter(|f| f.has_adjacent_part(&symbols))
        .map(|f| f.value)
        .sum::<usize>();

    Some(s)
}

pub fn part_two(input: &str) -> Option<usize> {
    let numbers = get_parts(input);
    let symbols: Vec<Symbol> = get_symbols(input);
    let gears: Vec<&Symbol> = symbols.iter().filter(|f| f.c == '*').collect();

    // Ideal optimisation here would be some proxy variable for adjacency in space to limit number of parts checked
    // Quadtree or space filling curve maybe?

    let powers: usize = gears.iter().filter_map(|f| f.gear_power(&numbers)).sum();

    Some(powers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
