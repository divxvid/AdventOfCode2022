use std::collections::HashMap;
use std::str::FromStr;
use std::num::ParseIntError;

#[allow(dead_code)]
#[derive(Debug)]
struct File {
    name: String,
    size: u64
}

impl FromStr for File {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        return Ok(File {
            name: parts[1].to_string(),
            size: parts[0].parse::<u64>()?
        });
    }
}

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

fn process_ls(
    directory_map: &mut HashMap<String, Vec<String>>,
    parent_directory: &mut HashMap<String, String>,
    current_directory: &String,
    directory_files: &mut HashMap<String, Vec<File>>,
    cmd: &Vec<String>
) {
    cmd.iter().skip(1)
        .for_each(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            if parts[0] == "dir" {
                let directory_name = parts[1];
                if !directory_map.contains_key(directory_name) {
                    let children_vector : Vec<String> = Vec::new();
                    directory_map.insert(directory_name.to_string(),  children_vector);
                }
                parent_directory.insert(directory_name.to_string(), current_directory.to_string());
                let children =  directory_map.get_mut(current_directory);
                match children {
                    Some(cv) => cv.push(directory_name.to_string()),
                    None => ()
                }
            } else {
                let file: File = line.parse::<File>().unwrap();
                if !directory_files.contains_key(current_directory) {
                    let vn: Vec<File> = vec![file];
                    directory_files.insert(current_directory.clone(), vn);
                } else {
                    let vc = directory_files.get_mut(current_directory);
                    match vc {
                        Some(v) => v.push(file),
                        None => ()
                    }
                }
            }
        });
}

fn get_directory_sizes(
    directory_files: &HashMap<String, Vec<File>>
) -> HashMap<String, u64> {
    directory_files.iter()
        .map(|(a, b)| {
            (a.clone(), b.iter().map(|f| f.size).sum::<u64>())
        })
        .collect::<HashMap<String, u64>>()
}

fn dfs(
    directory_map: &HashMap<String, Vec<String>>,
    directory_sizes: &mut HashMap<String, u64>,
    current_directory: String
) -> u64 {
    if directory_map.get(&current_directory).unwrap().is_empty() {
        return 0;
    }
    dbg!(&current_directory);
    let dir_size = directory_sizes.get(&current_directory);
    let mut total: u64 = 0;
    match dir_size {
        Some(x) => total += *x,
        _ => ()
    }
    // let mut total: u64 = directory_sizes.get(&current_directory).unwrap().clone();
    for child in directory_map.get(&current_directory).unwrap().iter() {
        total += dfs(directory_map, directory_sizes, child.clone());
    }

    directory_sizes.entry(current_directory).and_modify(|x| *x = total);

    total
}

fn process_cd(
    cmd: &String,
    current_directory: &mut String,
    parent_directory: &HashMap<String, String>,
) {
    let parts = cmd.split_whitespace().collect::<Vec<&str>>();
    if parts[1] == ".." {
        let parent = parent_directory.get(current_directory).unwrap();
        current_directory.clear();
        current_directory.push_str(parent);
    } else {
        current_directory.clear();
        current_directory.push_str(parts[1]);
    }
}

pub fn part_one(input: &str) -> String {
    let mut directory_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_directory : String = "/".to_string();
    let mut parent_directory: HashMap<String, String> = HashMap::new();
    let mut directory_files: HashMap<String, Vec<File>> = HashMap::new();
    directory_map.insert("/".to_string(), Vec::new());

    let commands = parse_input(input);
    commands.iter()
        .for_each(|cmd| {
            if cmd[0].contains("ls") {
                process_ls(
                    &mut directory_map,
                    &mut parent_directory,
                    &current_directory,
                    &mut directory_files,
                    cmd
                );
            } else {
                process_cd(
                    &cmd[0],
                    &mut current_directory,
                    &parent_directory
                );
            }
        });

    let mut directory_sizes = get_directory_sizes(&directory_files);
    dfs(&directory_map, &mut directory_sizes, "/".to_string());

    directory_sizes.iter()
        .map(|(_, sz)| sz)
        .filter(|sz| *sz <= &100000)
        .sum::<u64>()
        .to_string()

}

pub fn part_two(_input: &str) -> String {
    "unsolved".to_string()
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
    #[ignore]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, "");
    }
}
