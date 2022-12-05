pub fn part_one(input: &str) -> String {
    let stacks_str = input.lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let moves_str = input.lines()
        .filter(|line| line.chars().nth(0) == Some('m'))
        .collect::<Vec<&str>>();

    let num_stacks = (stacks_str[0].len() + 1) / 4; //[X]<space> total 4 chars for a stack

    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];

    stacks_str.iter().take(stacks_str.len()-1).rev()
        .for_each(|row| {
            for i in 0..num_stacks {
                let index = i * 4 + 1;
                let char_to_add: char = row.chars().nth(index).unwrap();
                if char_to_add != ' ' {
                    stacks[i].push(char_to_add);
                }
            }
        });

    moves_str.iter()
        .for_each(|mv| {
            let splits = mv.split_whitespace().collect::<Vec<&str>>();
            let mut amount: u32 = splits[1].parse::<u32>().unwrap();
            let from: usize = splits[3].parse::<usize>().unwrap()-1;
            let to: usize = splits[5].parse::<usize>().unwrap()-1;

            while amount > 0 {
                let last_element: Option<char> = stacks[from].pop();
                match last_element {
                    Some(x) => stacks[to].push(x),
                    None => (),
                };
                amount -= 1;
            }
        });

    stacks.iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

pub fn part_two(input: &str) -> String {
    let stacks_str = input.lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let moves_str = input.lines()
        .filter(|line| line.chars().nth(0) == Some('m'))
        .collect::<Vec<&str>>();

    let num_stacks = (stacks_str[0].len() + 1) / 4; //[X]<space> total 4 chars for a stack

    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];

    stacks_str.iter().take(stacks_str.len()-1).rev()
        .for_each(|row| {
            for i in 0..num_stacks {
                let index = i * 4 + 1;
                let char_to_add: char = row.chars().nth(index).unwrap();
                if char_to_add != ' ' {
                    stacks[i].push(char_to_add);
                }
            }
        });

    moves_str.iter()
        .for_each(|mv| {
            let splits = mv.split_whitespace().collect::<Vec<&str>>();
            let mut amount: u32 = splits[1].parse::<u32>().unwrap();
            let from: usize = splits[3].parse::<usize>().unwrap()-1;
            let to: usize = splits[5].parse::<usize>().unwrap()-1;

            let mut reversal_stk: Vec<char> = Vec::new();
            while amount > 0 {
                let last_element: Option<char> = stacks[from].pop();
                match last_element {
                    Some(x) => reversal_stk.push(x),
                    None => (),
                };
                amount -= 1;
            }

            while !reversal_stk.is_empty() {
                let last_element: Option<char> = reversal_stk.pop();
                match last_element {
                    Some(x) => stacks[to].push(x),
                    None => (),
                };
            }
        });

    stacks.iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, "MCD");
    }
}
