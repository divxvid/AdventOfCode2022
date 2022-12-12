use std::collections::VecDeque;

const INF: i32 = 10_000_000;
const DIR: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

pub fn part_one(input: &str) -> String {
    let mut grid = input.lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut x = 0;
    let mut y = 0;
    let mut ex = 0;
    let mut ey = 0;

    for i in 0..n_rows {
        for j in 0..n_cols {
            if grid[i][j] == 'S' {
                x = i;
                y = j;
            } else if grid[i][j] == 'E' {
                ex = i;
                ey = j;
            }
        }
    }

    grid[x][y] = 'a';
    grid[ex][ey] = 'z';

    let mut dist = vec![vec![INF; n_cols]; n_rows];
    dist[x][y] = 0;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((x, y));

    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();

        if x == ex && y == ey {
            break;
        }

        for (dx, dy) in DIR {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx < 0 || nx == n_rows as i32 || ny < 0 || ny == n_cols as i32 {
                continue;
            } 

            let nx = nx as usize;
            let ny = ny as usize;
            let distance_check = dist[nx][ny] > (dist[x][y] + 1);
            let height_check = grid[nx][ny] as i32 - grid[x][y] as i32 <= 1;

            if distance_check && height_check {
                dist[nx][ny] = dist[x][y] + 1;
                q.push_back((nx, ny));
            }
        }
    }

    dist[ex][ey].to_string()
}

pub fn part_two(input: &str) -> String {
    let mut grid = input.lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut ex = 0;
    let mut ey = 0;

    'outer: for i in 0..n_rows {
        for j in 0..n_cols {
            if grid[i][j] == 'E' {
                ex = i;
                ey = j;
                break 'outer;
            }
        }
    }

    grid[ex][ey] = 'z';

    let mut dist = vec![vec![INF; n_cols]; n_rows];
    dist[ex][ey] = 0;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((ex, ey));

    let mut min_dist = INF;

    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();

        if grid[x][y] == 'a' { 
            if dist[x][y] < min_dist {
                min_dist = dist[x][y];
            }
            continue;
        }

        for (dx, dy) in DIR {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx < 0 || nx == n_rows as i32 || ny < 0 || ny == n_cols as i32 {
                continue;
            } 

            let nx = nx as usize;
            let ny = ny as usize;
            let distance_check = dist[nx][ny] > (dist[x][y] + 1);
            let height_check = grid[nx][ny] as i32 - grid[x][y] as i32 >= -1;

            if distance_check && height_check {
                dist[nx][ny] = dist[x][y] + 1;
                q.push_back((nx, ny));
            }
        }
    }

    min_dist.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, "31");
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, "29");
    }
}
