use std::collections::HashSet;

#[derive(Debug)]
struct Line {
    start: (u32, u32),
    end: (u32, u32)
}

fn parse_input(input: &str) -> Vec<Vec<Line>> {
    input.lines()
        .map(|line| {
            let l: Vec<&str> = line.split("->").collect();
            l.iter().zip(l.iter().skip(1))
                .map(|(x, y)| {
                    let start = x.trim().split_once(",").unwrap();
                    let end = y.trim().split_once(",").unwrap();
                    let start = (start.0.parse::<u32>().unwrap(), start.1.parse::<u32>().unwrap());
                    let end = (end.0.parse::<u32>().unwrap(), end.1.parse::<u32>().unwrap());
                    Line {
                        start,
                        end
                    }
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> String {
    let lines = parse_input(input);
    let h_max = lines.iter().map(|line| {
        line.iter().map(|l| l.start.1.max(l.end.1)).max().unwrap()
    }).max().unwrap();

    let mut grid: HashSet<(u32, u32)> = HashSet::new();

    lines.iter().for_each(|line| {
        line.iter().for_each(|l| {
            if l.start.0 == l.end.0 {
                let mn = l.start.1.min(l.end.1);
                let mx = l.start.1.max(l.end.1);
                (mn..=mx).for_each(|v| {
                    grid.insert((l.start.0, v));
                });
            } else {
                let mn = l.start.0.min(l.end.0);
                let mx = l.start.0.max(l.end.0);
                (mn..=mx).for_each(|v| {
                    grid.insert((v, l.start.1));
                });
            }
        })
    });

    let mut total_sand: u32 = 0;
    loop {
        let mut current: (u32, u32) = (500, 0);
        if grid.contains(&current) {
            break;
        }
        while current.1 < h_max {
            if !grid.contains(&(current.0, current.1+1)) {
                current.1 += 1;
            } else if !grid.contains(&(current.0-1, current.1+1)) {
                current.0 -= 1;
                current.1 += 1;
            } else if !grid.contains(&(current.0+1, current.1+1)) {
                current.0 += 1;
                current.1 += 1;
            } else {
                break;
            }
        }

        if current.1 == h_max {
            break;
        }
        grid.insert(current);
        total_sand += 1;
    }

    total_sand.to_string()
}

pub fn part_two(input: &str) -> String {
    let lines = parse_input(input);
    let h_max = 2 + lines.iter().map(|line| {
        line.iter().map(|l| l.start.1.max(l.end.1)).max().unwrap()
    }).max().unwrap();

    let mut grid: HashSet<(u32, u32)> = HashSet::new();

    lines.iter().for_each(|line| {
        line.iter().for_each(|l| {
            if l.start.0 == l.end.0 {
                let mn = l.start.1.min(l.end.1);
                let mx = l.start.1.max(l.end.1);
                (mn..=mx).for_each(|v| {
                    grid.insert((l.start.0, v));
                });
            } else {
                let mn = l.start.0.min(l.end.0);
                let mx = l.start.0.max(l.end.0);
                (mn..=mx).for_each(|v| {
                    grid.insert((v, l.start.1));
                });
            }
        })
    });

    let mut total_sand: u32 = 0;
    loop {
        let mut current: (u32, u32) = (500, 0);
        if grid.contains(&current) {
            break;
        }
        loop {
            if current.1+1 == h_max {
                break;
            } else if !grid.contains(&(current.0, current.1+1)) {
                current.1 += 1;
            } else if !grid.contains(&(current.0-1, current.1+1)) {
                current.0 -= 1;
                current.1 += 1;
            } else if !grid.contains(&(current.0+1, current.1+1)) {
                current.0 += 1;
                current.1 += 1;
            } else {
                break;
            }
        }

        grid.insert(current);
        total_sand += 1;
    }

    total_sand.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, "24");
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, "93");
    }
}
