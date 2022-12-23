use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Vec<String>> {
    input.split("$")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|cmd| {
            cmd.lines().map(str::to_string)
            .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

pub fn part_one(input: &str) -> String {
    let commands = parse_input(input);
    let mut current_path: Vec<&str> = Vec::new();
    let mut spaces: HashMap<String, u64> = HashMap::new();

    commands.iter().for_each(|cmd| {
        match cmd[0].as_str() {
            "ls" => {
                let path = current_path.join("-");
                cmd.iter().skip(1).for_each(|c| {
                    let (a, _) = c.split_once(" ").expect("ls directory split went wrong");
                    match a {
                        "dir" => {},
                        _ => {
                            let a = a.parse::<u64>().expect("file size gone wrong");
                            *spaces.entry(path.clone()).or_insert(0) += a;
                        }
                    }
                });
            },
            _ => {
                let (_, dir) = cmd[0].split_once(" ").expect("invalid cd structure");
                match dir {
                    ".." => {
                        let old_path = current_path.join("-");
                        let old_path_size = spaces.get(&old_path).unwrap();
                        current_path.pop().unwrap();
                        let new_path = current_path.join("-");
                        *spaces.entry(new_path.clone()).or_insert(0) += *old_path_size;
                    },
                    _ => current_path.push(dir)
                };
            }
        }
    });
    
    while current_path.len() > 1 {
        let old_path = current_path.join("-");
        let old_path_size = spaces.get(&old_path).unwrap();
        current_path.pop().unwrap();
        let new_path = current_path.join("-");
        *spaces.entry(new_path.clone()).or_insert(0) += *old_path_size;
    }

    spaces.iter()
        .filter(|(_, v)| **v <= 100_000)
        .map(|(_, v)| *v)
        .sum::<u64>()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    let commands = parse_input(input);
    let mut current_path: Vec<&str> = Vec::new();
    let mut spaces: HashMap<String, u64> = HashMap::new();

    commands.iter().for_each(|cmd| {
        match cmd[0].as_str() {
            "ls" => {
                let path = current_path.join("-");
                cmd.iter().skip(1).for_each(|c| {
                    let (a, _) = c.split_once(" ").expect("ls directory split went wrong");
                    match a {
                        "dir" => {},
                        _ => {
                            let a = a.parse::<u64>().expect("file size gone wrong");
                            *spaces.entry(path.clone()).or_insert(0) += a;
                        }
                    }
                });
            },
            _ => {
                let (_, dir) = cmd[0].split_once(" ").expect("invalid cd structure");
                match dir {
                    ".." => {
                        let old_path = current_path.join("-");
                        let old_path_size = spaces.get(&old_path).unwrap();
                        current_path.pop().unwrap();
                        let new_path = current_path.join("-");
                        *spaces.entry(new_path.clone()).or_insert(0) += *old_path_size;
                    },
                    _ => current_path.push(dir)
                };
            }
        }
    });
    
    while current_path.len() > 1 {
        let old_path = current_path.join("-");
        let old_path_size = spaces.get(&old_path).unwrap();
        current_path.pop().unwrap();
        let new_path = current_path.join("-");
        *spaces.entry(new_path.clone()).or_insert(0) += *old_path_size;
    }

    let root_size = *spaces.get("/").unwrap();
    let free_space = 70_000_000 - root_size;
    let space_to_free = 30_000_000 - free_space;

    spaces.iter()
        .filter(|(_, v)| **v >= space_to_free)
        .map(|(_, v)| *v)
        .min().unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, "95437");
    }
    
    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, "24933642");
    }
}
