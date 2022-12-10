use std::{str::FromStr, num::ParseIntError};

#[derive(Debug)]
enum Operation {
    Addx(i32),
    Noop
}

impl FromStr for Operation {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.split_once(" ");
        match val {
            Some((_, b)) => {
                let b = b.parse::<i32>()?;
                Ok(Operation::Addx(b))
            },
            None => Ok(Operation::Noop)
        }
    }
}

pub fn part_one(input: &str) -> String {
    let mut total: i32 = 0;
    let mut cycle: i32 = 0;
    let mut value: i32 = 1;

    input.lines()
        .map(|l| l.parse::<Operation>().unwrap())
        .for_each(|operation| {       
            match operation {
                Operation::Noop => {
                    cycle += 1;
                    if (cycle - 20) % 40 == 0 {
                        total += value * cycle;
                    }
                },
                Operation::Addx(x) => {
                    cycle += 1;
                    if (cycle - 20) % 40 == 0 {
                        total += value * cycle;
                    }
                    cycle += 1;
                    if (cycle - 20) % 40 == 0 {
                        total += value * cycle;
                    }
                    value += x;
                }
            }
        }
    );

    total.to_string()
}

pub fn part_two(input: &str) -> String {
    let mut cycle: i32 = 0;
    let mut position: i32 = 1;
    let mut crt: String = String::new();

    input.lines()
        .map(|l| l.parse::<Operation>().unwrap())
        .for_each(|operation| {       
            match operation {
                Operation::Noop => {
                    let check_index = cycle % 40;
                    if ((position-1)..=(position+1)).contains(&check_index) {
                        crt.push('#');
                    } else {
                        crt.push('.');
                    }
                    cycle += 1;
                    if cycle % 40 == 0 {
                        crt.push('\n');
                    }
                },
                Operation::Addx(x) => {
                    let check_index = cycle % 40;
                    if ((position-1)..=(position+1)).contains(&check_index) {
                        crt.push('#');
                    } else {
                        crt.push('.');
                    }
                    cycle += 1;
                    if cycle % 40 == 0 {
                        crt.push('\n');
                    }
                    let check_index = cycle % 40;
                    if ((position-1)..=(position+1)).contains(&check_index) {
                        crt.push('#');
                    } else {
                        crt.push('.');
                    }
                    cycle += 1;
                    if cycle % 40 == 0 {
                        crt.push('\n');
                    }
                    position += x;
                }
            }
        }
    );

    crt.pop();
    crt
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, "13140");
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        const ACTUAL: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(result, ACTUAL);
    }
}
