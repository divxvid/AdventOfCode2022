use std::{str::FromStr, num::ParseIntError};

#[derive(Debug, Clone, Copy)]
struct Segment {
    left: u32,
    right: u32
}

impl FromStr for Segment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bounds: Vec<&str> = s.split("-").collect();
        let left = bounds[0].parse::<u32>()?;
        let right = bounds[1].parse::<u32>()?;
        return Ok(Segment {
            left,
            right
        });
    }
}

impl Segment {
    fn is_inside(&self, other: &Self) -> bool {
        self.left >= other.left && self.right <= other.right
    }
    fn is_outside(&self, other: &Self) -> bool {
        self.left > other.right || self.right <  other.left  
    }
}

pub fn part_one(input: &str) -> String {
    input.lines()
        .map(|line| {
            line.split(",")
            .map(|x| x.parse::<Segment>().unwrap())
            .collect::<Vec<Segment>>()})
        .filter(|v| (v[0].is_inside(&v[1]) || v[1].is_inside(&v[0])))
        .count()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    input.lines()
        .map(|line| {
            line.split(",")
            .map(|x| x.parse::<Segment>().unwrap())
            .collect::<Vec<Segment>>()})
        .filter(|v| !(v[0].is_outside(&v[1]) && v[1].is_outside(&v[0])))
        .count()
        .to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, "4");
    }
}
