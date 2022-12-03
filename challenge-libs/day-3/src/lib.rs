fn map_priority(character: char) -> u32 {
    if character.is_lowercase() {
        return (character as u32) - ('a' as u32) + 1;
    } else {
        return (character as u32) - ('A' as u32) + 27;
    }
}

pub fn part_one(input: &str) -> String {
    input.lines()
        .map(|line| line.split_at(line.len()/2))
        .map(|(a, b)| {
            for ch_a in a.chars() {
                for ch_b in b.chars() {
                    if ch_a == ch_b {
                        return map_priority(ch_a);
                    }
                }
            }
            return 0;
        })
        .sum::<u32>()
        .to_string()
}

fn find_same_element(a: &str, b: &str, c: &str) -> char {
    const MAX: usize = 150;
    let mut ccounta: [bool; MAX] = [false; MAX];
    let mut ccountb: [bool; MAX] = [false; MAX];
    let mut ccountc: [bool; MAX] = [false; MAX];

    for x in a.chars() {
        ccounta[x as usize] = true;
    }
    for x in b.chars() {
        ccountb[x as usize] = true;
    }
    for x in c.chars() {
        ccountc[x as usize] = true;
    }

    for i in 0..MAX {
        if ccounta[i] && ccountb[i] && ccountc[i] {
            return char::from_u32(i as u32).unwrap();
        }
    }

    return '?';
}

pub fn part_two(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let length = lines.len();

    let mut total_ans = 0u32;
    let mut i = 0;
    while i < length {
        let same_element = find_same_element(lines[i], lines[i+1], lines[i+2]);
        total_ans += map_priority(same_element);
        i += 3;
    }

    return total_ans.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn test_map_priority() {
        let result1 = map_priority('d');
        assert_eq!(result1, 4);

        let result2 = map_priority('Z');
        assert_eq!(result2, 52);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, "70");
    }

    #[test]
    fn test_find_same_element() {
        let lines: Vec<&str> = INPUT.lines().take(3).collect();
        let result = find_same_element(lines[0], lines[1], lines[2]);
        assert_eq!(result, 'r');
    }
}
