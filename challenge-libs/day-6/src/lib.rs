use itertools::{izip, enumerate};

pub fn part_one(input: &str) -> String {
    enumerate(izip!(
        input.chars(),
        input.chars().skip(1),
        input.chars().skip(2),
        input.chars().skip(3)
    )).into_iter()
    .filter(|(_, (a, b, c, d))| {
        if a == b || a == c || a == d {
            false
        } else if b == c || b == d {
            false
        } else if c == d {
            false
        } else {
            true
        }
    }).map(|(idx, (_, _, _, _))| idx+4)
    .take(1)
    .collect::<Vec<usize>>()
    .iter().last().unwrap()
    .to_string()
}

pub fn part_two(input: &str) -> String {
    let mut cnt = vec![0; 26];
    let length = input.len();

    for i in 0..14 {
        cnt[input.chars().nth(i).unwrap() as usize - 97] += 1;
    }

    if cnt.iter().filter(|c| *c >= &2).sum::<i32>() == 0 {
        return "14".to_string();
    }

    for i in 14..length {
        cnt[input.chars().nth(i).unwrap() as usize - 97] += 1;
        cnt[input.chars().nth(i-14).unwrap() as usize - 97] -= 1;
        
        if cnt.iter().filter(|c| *c >= &2).sum::<i32>() == 0 {
            return (i+1).to_string();
        }
    }

    "unsolved".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "7");
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"),"6");
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10");
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "19");
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23");
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"),"23");
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "29");
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "26");
    }
}
