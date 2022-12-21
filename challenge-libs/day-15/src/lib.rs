use std::{str::FromStr, num::ParseIntError};
use std::collections::HashSet;

#[derive(Debug)]
struct Sensor {
    location: (i32, i32),
    closest_beacon: (i32, i32),
    beacon_distance: u32
}

impl FromStr for Sensor {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(":").unwrap();

        let (cml, cmr) = left.split_once(",").unwrap();
        let sx = cml.split_once("=").unwrap().1.parse::<i32>()?;
        let sy = cmr.split_once("=").unwrap().1.parse::<i32>()?;

        let (cml, cmr) = right.split_once(",").unwrap();
        let bx = cml.split_once("=").unwrap().1.parse::<i32>()?;
        let by = cmr.split_once("=").unwrap().1.parse::<i32>()?;
        let beacon_distance = sx.abs_diff(bx) + sy.abs_diff(by);

        Ok(Sensor {
            location: (sx, sy),
            closest_beacon: (bx, by),
            beacon_distance
        })
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    input.lines()
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect()
}

pub fn part_one(input: &str, y: i32) -> String {
    let sensors = parse_input(input);
    let mut valid_points: HashSet<i32> = HashSet::new();

    sensors.iter().for_each(|sensor| {
        let delta = sensor.location.1.abs_diff(y);
        if delta > sensor.beacon_distance {
            return;
        }

        let remaining = sensor.beacon_distance - delta;
        let start = sensor.location.0 - remaining as i32;
        let end = sensor.location.0 + remaining as i32;
        (start..=end).for_each(|x| {
            valid_points.insert(x);
        });
    });

    sensors.iter()
        .filter(|sensor| sensor.closest_beacon.1 == y)
        .for_each(|sensor| {
            valid_points.remove(&sensor.closest_beacon.0);
        });

    
    valid_points.len().to_string()
}

pub fn part_two(input: &str, y_limit: i32) -> String {
    let sensors = parse_input(input);
    let mut ans: Option<i64> = None;

    for y in 0..=y_limit {
        let mut ranges: Vec<(i32, i32)> = Vec::new();
        sensors.iter().for_each(|sensor| {
            let delta = sensor.location.1.abs_diff(y);
            if delta > sensor.beacon_distance {
                return;
            }

            let remaining = sensor.beacon_distance - delta;
            let start = sensor.location.0 - remaining as i32;
            let end = sensor.location.0 + remaining as i32;
            ranges.push((start, end));
        });

        ranges.sort();
        let mut current_range = ranges[0];

        for r in ranges.iter().skip(1) {
            if r.0 - current_range.1 > 1 {
                if r.0 < 0 {
                    current_range = *r;
                } else if r.0 - current_range.1 == 2 {
                    ans = Some((r.0 as i64 - 1) * 4_000_000 + y as i64);
                    break;
                } else {
                    panic!("what the? current:{current_range:?}\nnext:{r:?}\nranges:{ranges:?}");
                }
            } else {
                current_range.1 = current_range.1.max(r.1);
            }
        }
        if ans.is_some() {
            break;
        }
    }

    ans.expect("No Answer found ?!").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT, 10);
        assert_eq!(result, "26");
    }
    #[test]
    fn test_part_two() {
        let result = part_two(INPUT, 20);
        assert_eq!(result, "56000011");
    }
}
