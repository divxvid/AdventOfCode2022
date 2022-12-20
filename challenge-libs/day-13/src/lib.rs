use std::{str::FromStr, num::ParseIntError};

use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::IResult;
use nom::sequence::delimited;
use nom::multi::separated_list0;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Element {
    List(Vec<Element>),
    Single(u32)
}

fn parse_single_element(input: &str) -> IResult<&str, Element> {
    let (input, x) = nom::character::complete::u32(input)?;
    Ok((input, Element::Single(x)))
}

fn parse_list_element(input: &str) -> IResult<&str, Element> {
    let (input, v) = delimited(
        tag("["),
        separated_list0(tag(","), alt((parse_list_element, parse_single_element))),
        tag("]")
    )(input)?;
    
    Ok((input, Element::List(v)))
}

fn parse_element(input: &str) -> IResult<&str, Vec<Element>> {
    delimited(
        tag("["),
        separated_list0(
            tag(","),
            alt((parse_list_element, parse_single_element))
        ),
        tag("]")
    )(input)
}

#[derive(Debug)]
struct Pair {
    fst: Vec<Element>,
    snd: Vec<Element>
}

impl FromStr for Pair {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.split_once("\n").unwrap();
        let (_, fst) = parse_element(p1).unwrap();
        let (_, snd) = parse_element(p2).unwrap();
        Ok(Pair {
            fst,
            snd
        })
    }
}

fn parse_input(input: &str) -> Vec<Pair> {
    input.split("\n\n")
        .map(|x| x.parse::<Pair>().unwrap())
        .collect::<Vec<Pair>>()
}

pub fn part_one(input: &str) -> String {
    parse_input(&input).iter()
        .enumerate()
        .filter(|(_, pair)| pair.fst <= pair.snd)
        .map(|(i, _)| i+1)
        .sum::<usize>()
        .to_string()
}

pub fn part_two(_input: &str) -> String {
    "unsolved".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    #[ignore]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn testing_random() {
        let e1 = Element::List(
            vec![
                Element::Single(1),
                Element::Single(2),
                Element::Single(3),
                ]
        );

        let e2 = Element::Single(4);
        let res = e1 <= e2;
        assert_eq!(res, false);
    }
}
