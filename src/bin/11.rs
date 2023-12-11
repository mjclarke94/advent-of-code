advent_of_code::solution!(11);

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn expand(&mut self, x: &[usize], y: &[usize], n: usize) {
        self.x += (n - 1) * (x.iter().filter(|n| n < &&self.x).count());
        self.y += (n - 1) * (y.iter().filter(|n| n < &&self.y).count());
    }

    fn distance(&self, other: Galaxy) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Galaxy>,
}

impl Universe {
    fn from_input(input: &str) -> Universe {
        let lines: Vec<&str> = input.split('\n').filter(|f| !f.is_empty()).collect();

        let mut galaxies: Vec<Galaxy> = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, _) in line.chars().enumerate().filter(|(_, c)| c == &'#') {
                galaxies.push(Galaxy { x, y });
            }
        }

        Universe { galaxies }
    }

    fn empty_row(&self, y: usize) -> bool {
        for g in self.galaxies.iter() {
            if g.y == y {
                return false;
            }
        }
        true
    }

    fn empty_col(&self, x: usize) -> bool {
        for g in self.galaxies.iter() {
            if g.x == x {
                return false;
            }
        }
        true
    }

    fn expand(&mut self, n: usize) {
        let max_x = self.galaxies.iter().map(|f| f.x).max().unwrap();
        let max_y = self.galaxies.iter().map(|f| f.y).max().unwrap();

        let unoccupied_x: Vec<usize> = (0..=max_x).filter(|x| self.empty_col(*x)).collect();
        let unoccupied_y: Vec<usize> = (0..=max_y).filter(|y| self.empty_row(*y)).collect();

        for galaxy in self.galaxies.iter_mut() {
            galaxy.expand(&unoccupied_x, &unoccupied_y, n);
        }
    }

    fn sum_of_distances(&self) -> usize {
        self.galaxies
            .iter()
            .combinations(2)
            .map(|v| (*v[0]).distance(*v[1]))
            .sum()
    }
}

pub fn solve_general(input: &str, n: usize) -> Option<usize> {
    let mut universe = Universe::from_input(input);
    universe.expand(n);
    Some(universe.sum_of_distances())
}

pub fn part_one(input: &str) -> Option<usize> {
    solve_general(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve_general(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = solve_general(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(1030));
        let result = solve_general(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, Some(8410));
    }
}
