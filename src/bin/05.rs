use itertools::Itertools;
use std::{ops::Range, usize};

advent_of_code::solution!(5);

#[derive(Debug, PartialEq)]
struct Almanac {
    mappings: Vec<TransferFunction>,
}

impl Almanac {
    fn from_input(input: &str) -> (Vec<usize>, Almanac) {
        let mut lines = input.split("\n\n");
        let seedline: &str = lines.next().unwrap();

        let seeds: Vec<usize> = seedline
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|f| f.parse().unwrap())
            .collect();

        let mappings: Vec<TransferFunction> = lines.map(TransferFunction::from_lines).collect();

        (seeds, Almanac { mappings })
    }

    fn from_input_range(input: &str) -> (SeedRanges, Almanac) {
        let mut lines = input.split("\n\n");
        let seedline: &str = lines.next().unwrap();

        let seeds: SeedRanges = SeedRanges::from_line(seedline);

        let mappings: Vec<TransferFunction> = lines.map(TransferFunction::from_lines).collect();

        (seeds, Almanac { mappings })
    }

    fn map_seed(&self, i: usize) -> usize {
        let mut i = i;

        for mapping in self.mappings.iter() {
            i = mapping.map(i);
        }

        i
    }
}
#[derive(Debug)]
struct SeedRanges {
    ranges: Vec<Range<usize>>,
    endpoints: Vec<usize>,
}

impl SeedRanges {
    fn from_line(line: &str) -> SeedRanges {
        let mut ranges: Vec<Range<usize>> = vec![];
        let mut endpoints: Vec<usize> = vec![];

        let nums: Vec<usize> = line
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|f| f.parse().unwrap())
            .collect();

        for s in nums.chunks(2) {
            ranges.push((s[0])..(s[0] + s[1]));
            endpoints.push(s[0]);
            endpoints.push(s[0] + s[1] - 1)
        }
        endpoints.sort();

        SeedRanges { ranges, endpoints }
    }

    fn contains(&self, i: usize) -> bool {
        for r in self.ranges.iter() {
            if r.contains(&i) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, PartialEq)]
struct TransferFunction {
    map_components: Vec<MapComponent>,
}

impl TransferFunction {
    fn from_lines(lines: &str) -> TransferFunction {
        let mut line_iter = lines.split('\n').filter(|f| !f.is_empty());
        line_iter.next();

        let mut map_components: Vec<MapComponent> = Vec::new();

        for line in line_iter {
            map_components.push(MapComponent::from_line(line))
        }

        TransferFunction { map_components }
    }

    fn discontinuities(&self, external: Vec<usize>) -> Vec<usize> {
        let mut d: Vec<usize> = self
            .map_components
            .iter()
            .flat_map(|f| f.discontinuities)
            .collect();

        let external_inverted: Vec<usize> = external.iter().map(|f| self.inverse_map(*f)).collect();

        d.extend(external_inverted);

        d.iter().sorted().unique().copied().collect()
    }

    fn map(&self, i: usize) -> usize {
        for r in self.map_components.iter() {
            if r.source.contains(&i) {
                return (i as isize + r.offset) as usize;
            }
        }
        i
    }
    fn inverse_map(&self, i: usize) -> usize {
        for r in self.map_components.iter() {
            if r.dest.contains(&i) {
                return (i as isize - r.offset) as usize;
            }
        }
        i
    }
}

#[derive(Debug, PartialEq)]
struct MapComponent {
    source: Range<usize>,
    dest: Range<usize>,
    offset: isize,
    discontinuities: [usize; 4],
}

impl MapComponent {
    fn from_line(line: &str) -> MapComponent {
        let mut l = line.split_whitespace();
        let dest: usize = l.next().unwrap().parse().unwrap();
        let source: usize = l.next().unwrap().parse().unwrap();

        let range_len: usize = l.next().unwrap().parse().unwrap();

        MapComponent {
            source: source..(source + range_len),
            dest: dest..(dest + range_len),
            offset: (dest as isize) - (source as isize),
            discontinuities: [
                source.saturating_sub(1),
                source,
                (source + range_len - 1),
                (source + range_len),
            ],
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (seeds, almanac) = Almanac::from_input(input);

    let locations: Vec<usize> = seeds.iter().map(|f| almanac.map_seed(*f)).collect();

    Some(*locations.iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (seeds, almanac) = Almanac::from_input_range(input);

    let mut dc = vec![0, usize::MAX];

    for map in almanac.mappings.iter().rev() {
        dc = map.discontinuities(dc);
    }

    dc.extend(&seeds.endpoints);

    let possible_seeds: Vec<usize> = dc.iter().copied().filter(|f| seeds.contains(*f)).collect();

    let locations: Vec<usize> = possible_seeds
        .iter()
        .map(|f| almanac.map_seed(*f))
        .collect();

    Some(*locations.iter().min().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
