use std::{str::FromStr, num::{ParseIntError}};

#[derive(Debug)]
enum Operation {
    Add(Option<i64>),
    Sub(Option<i64>),
    Mult(Option<i64>),
    Div(Option<i64>),
}

#[derive(Debug)]
struct Monkey {
    worry: Vec<i64>,
    operation: Operation,
    test: Operation,
    to_monkas: (usize, usize)
}

impl FromStr for Monkey {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(str::trim).collect::<Vec<&str>>();

        let (_, worry) = lines[1].split_once(":").unwrap();
        let worry = worry.split(",")
            .map(|w| w.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let operation = lines[2].split_whitespace()
            .skip(4)
            .collect::<Vec<&str>>();

        let operation = match operation[0] {
            "+" => Operation::Add(operation[1].parse::<i64>().ok()),
            "-" => Operation::Sub(operation[1].parse::<i64>().ok()),
            "*" => Operation::Mult(operation[1].parse::<i64>().ok()),
            "/" => Operation::Div(operation[1].parse::<i64>().ok()),
            _ => panic!("Invalid Operation!")
        };

        let test = lines[3].split_whitespace().collect::<Vec<&str>>();
        let test = Operation::Div(
            test.last().unwrap()
            .trim()
            .parse::<i64>().ok());

        let to_monkas_1 = lines[4].split_whitespace().collect::<Vec<&str>>();
        let to_monkas_1 = to_monkas_1.last().unwrap().trim().parse::<usize>().unwrap();

        let to_monkas_2 = lines[5].split_whitespace().collect::<Vec<&str>>();
        let to_monkas_2 = to_monkas_2.last().unwrap().trim().parse::<usize>().unwrap();

        Ok(Monkey {
            worry,
            operation,
            test,
            to_monkas: (to_monkas_1, to_monkas_2)
        })
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input.split("\n\n")
        .map(|monka| monka.parse::<Monkey>().unwrap())
        .collect::<Vec<Monkey>>()
}

pub fn part_one(input: &str) -> String {
    let mut monkeys = parse_input(input);
    let mut num_observations = vec![0; monkeys.len()];

    for _ in 1..=20 {
        for idx in 0..monkeys.len() {
            let moves = monkeys[idx].worry.iter().map(|w| {
                match monkeys[idx].operation {
                    Operation::Add(Some(x)) => w + x,
                    Operation::Sub(Some(x)) => w - x,
                    Operation::Mult(Some(x)) => w * x,
                    Operation::Div(Some(x)) => w / x,
                    Operation::Add(None) => w + w,
                    Operation::Sub(None) => w - w,
                    Operation::Mult(None) => w * w,
                    Operation::Div(None) => w / w,
                }
            })
            .map(|w| w / 3)
            .map(|w| {
                let result = match monkeys[idx].test {
                    Operation::Div(Some(x)) => w % x == 0,
                    _ => panic!("Aaaaaa")
                };

                match result {
                    true => (w, monkeys[idx].to_monkas.0),
                    false => (w, monkeys[idx].to_monkas.1)
                }
            }).collect::<Vec<(i64, usize)>>();

            num_observations[idx] += moves.len();
            for i in 0..moves.len() {
                monkeys[moves[i].1].worry.push(moves[i].0);
            }

            monkeys[idx].worry.clear();
        }
    }
    
    num_observations.sort_by(|a, b| b.cmp(a));
    (num_observations[0] * num_observations[1]).to_string()
}

pub fn part_two(input: &str) -> String {
    let mut monkeys = parse_input(input);
    let mut num_observations = vec![0; monkeys.len()];

    let modulo = monkeys.iter()
        .map(|monka| {
            match monka.test {
                Operation::Div(Some(x)) => x,
                _ => panic!("oops")
            }
        }).product::<i64>();

    for _ in 1..=10000 {
        for idx in 0..monkeys.len() {
            let moves = monkeys[idx].worry.iter().map(|w| {
                match monkeys[idx].operation {
                    Operation::Add(Some(x)) => (w + x) % modulo,
                    Operation::Sub(Some(x)) => (w - x + modulo) % modulo,
                    Operation::Mult(Some(x)) => (w * x) % modulo,
                    Operation::Div(Some(x)) => (w / x) % modulo,
                    Operation::Add(None) => (w + w) % modulo,
                    Operation::Sub(None) => (w - w + modulo) % modulo,
                    Operation::Mult(None) => (w * w) % modulo,
                    Operation::Div(None) => (w / w) % modulo,
                }
            })
            .map(|w| {
                let result = match monkeys[idx].test {
                    Operation::Div(Some(x)) => w % x == 0,
                    _ => panic!("Aaaaaa")
                };

                match result {
                    true => (w, monkeys[idx].to_monkas.0),
                    false => (w, monkeys[idx].to_monkas.1)
                }
            }).collect::<Vec<(i64, usize)>>();

            num_observations[idx] += moves.len();
            for i in 0..moves.len() {
                monkeys[moves[i].1].worry.push(moves[i].0);
            }

            monkeys[idx].worry.clear();
        }
    }
    
    dbg!(&num_observations);
    num_observations.sort_by(|a, b| b.cmp(a));
    (num_observations[0] * num_observations[1]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part_one() {
        let result = part_one(&INPUT);
        assert_eq!(result, "10605");
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&INPUT);
        assert_eq!(result, "2713310158");
    }
}
