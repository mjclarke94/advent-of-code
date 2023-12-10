advent_of_code::solution!(10);

use grid::Grid;
use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn update_index(&self, i: &mut (usize, usize)) {
        match self {
            Direction::Down => i.0 += 1,
            Direction::Up => i.0 -= 1,
            Direction::Right => i.1 += 1,
            Direction::Left => i.1 -= 1,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Horizontal,
    Vertical,
    UR,
    UL,
    DL,
    DR,
    Start,
    Empty,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '-' => Tile::Horizontal,
            '|' => Tile::Vertical,
            'L' => Tile::UR,
            'J' => Tile::UL,
            '7' => Tile::DL,
            'F' => Tile::DR,
            'S' => Tile::Start,
            _ => Tile::Empty,
        }
    }

    fn update_direction(&self, dir: &mut Direction) {
        match (self, &dir) {
            (Tile::UL, Direction::Down) | (Tile::DL, Direction::Up) => *dir = Direction::Left,
            (Tile::UR, Direction::Down) | (Tile::DR, Direction::Up) => *dir = Direction::Right,
            (Tile::UL, Direction::Right) | (Tile::UR, Direction::Left) => *dir = Direction::Up,
            (Tile::DL, Direction::Right) | (Tile::DR, Direction::Left) => *dir = Direction::Down,
            (Tile::Horizontal, _) | (Tile::Vertical, _) => {} // No need to change direction
            _ => {
                dbg!((self, &dir));
                todo!()
            }
        }
    }
}

fn parse_to_grid(input: &str) -> Grid<Tile> {
    let n: usize = input.split('\n').next().unwrap().len();
    let chars: Vec<Tile> = input
        .chars()
        .filter(|f| f != &'\n')
        .map(Tile::from_char)
        .collect();
    Grid::<Tile>::from_vec(chars, n)
}

fn find_start(g: &Grid<Tile>) -> Option<(usize, usize)> {
    for (i, mut row) in g.iter_rows().enumerate() {
        if let Some((j, _)) = &row.find_position(|f| **f == Tile::Start) {
            return Some((i, *j));
        }
    }
    None
}

fn find_initial_direction(g: &Grid<Tile>, idx: (usize, usize)) -> Option<Direction> {
    match g.get(idx.0 + 1, idx.1) {
        // Down
        Some(Tile::Vertical) | Some(Tile::UL) | Some(Tile::UR) => return Some(Direction::Down),
        _ => {}
    };

    match g.get(idx.0, idx.1 + 1) {
        // Right
        Some(Tile::Horizontal) | Some(Tile::DL) | Some(Tile::UL) => return Some(Direction::Right),
        _ => {}
    };

    match g.get(idx.0 - 1, idx.1) {
        // Up
        Some(Tile::Vertical) | Some(Tile::DL) | Some(Tile::DR) => return Some(Direction::Up),
        _ => {}
    };

    match g.get(idx.0, idx.1 - 1) {
        // Left
        Some(Tile::Vertical) | Some(Tile::UR) | Some(Tile::DR) => return Some(Direction::Left),
        _ => {}
    };

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let g = parse_to_grid(input);
    let mut i: u32 = 0;
    let mut idx = find_start(&g).unwrap();
    let mut tile: Tile;
    let mut dir: Direction = find_initial_direction(&g, idx).unwrap();

    loop {
        dir.update_index(&mut idx);
        i += 1;
        tile = g[idx];
        if tile == Tile::Start {
            break;
        }

        tile.update_direction(&mut dir);
    }

    Some(i.div_ceil(2))
}

fn shoelace(x: Vec<usize>, y: Vec<usize>) -> usize {
    let mut x_rot = x.clone();
    x_rot.rotate_left(1);

    let mut y_rot = y.clone();
    y_rot.rotate_left(1);

    let lhs: usize = x.iter().zip(y_rot.iter()).map(|(lx, ly)| lx * ly).sum();
    let rhs: usize = x_rot.iter().zip(y.iter()).map(|(rx, ry)| rx * ry).sum();

    lhs.abs_diff(rhs) / 2
}

pub fn part_two(input: &str) -> Option<usize> {
    let g = parse_to_grid(input);
    let mut idx = find_start(&g).unwrap();
    let mut tile: Tile;
    let mut dir: Direction = find_initial_direction(&g, idx).unwrap();

    let mut x: Vec<usize> = vec![idx.0];
    let mut y: Vec<usize> = vec![idx.1];

    loop {
        dir.update_index(&mut idx);
        tile = g[idx];
        if tile == Tile::Start {
            break;
        }
        x.push(idx.0);
        y.push(idx.1);

        tile.update_direction(&mut dir);
    }

    let boundary_points = &x.len();

    let area = shoelace(x, y);

    let interior = 1 + area - boundary_points / 2;

    Some(interior)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_simple() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }
    #[test]
    fn test_part_one_complex() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_small() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_med() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_large() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(8));
    }
    #[test]
    fn test_part_two_xl() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(10));
    }
}
