pub fn part_one(input: &str) -> String {
    let rows = input.lines().collect::<Vec<&str>>();
    let n_rows = rows.len();
    let n_cols = input.lines().map(|line| line.len()).max().unwrap();

    let mut row_left = vec![vec![false; n_cols]; n_rows];
    let mut row_right = vec![vec![false; n_cols]; n_rows];
    let mut col_top = vec![vec![false; n_cols]; n_rows];
    let mut col_bot = vec![vec![false; n_cols]; n_rows];

    //fill row_left
    for r in 0..n_rows {
        let mut last_big = rows[r].chars().nth(0).unwrap();
        row_left[r][0] = true;
        for c in 1..n_cols {
            let ch = rows[r].chars().nth(c).unwrap();
            if ch > last_big {
                last_big = ch;
                row_left[r][c] = true;
            } else {
                row_left[r][c] = false;
            }
        }
    }

    //fill row right
    for r in 0..n_rows {
        let mut last_big = rows[r].chars().nth(n_cols-1).unwrap();
        row_right[r][n_cols-1] = true;
        for c in (0..(n_cols-1)).rev() {
            let ch = rows[r].chars().nth(c).unwrap();
            if ch > last_big {
                last_big = ch;
                row_right[r][c] = true;
            } else {
                row_right[r][c] = false;
            }
        }
    }

    //fill col top
    for c in 0..n_cols {
        let mut last_big = rows[0].chars().nth(c).unwrap();
        col_top[0][c] = true;
        for r in 1..n_rows {
            let ch = rows[r].chars().nth(c).unwrap();
            if ch > last_big {
                last_big = ch;
                col_top[r][c] = true;
            } else {
                col_top[r][c] = false;
            }
        }
    }
    //fill col bot
    for c in 0..n_cols {
        let mut last_big = rows[n_rows-1].chars().nth(c).unwrap();
        col_bot[n_rows-1][c] = true;
        for r in (0..(n_rows-1)).rev() {
            let ch = rows[r].chars().nth(c).unwrap();
            if ch > last_big {
                last_big = ch;
                col_bot[r][c] = true;
            } else {
                col_bot[r][c] = false;
            }
        }
    }

    let mut ans = 0;
    for r in 0..n_rows {
        for c in 0..n_cols {
            if row_left[r][c] || row_right[r][c] || col_bot[r][c] || col_top[r][c] {
                ans += 1;
            }
        }
    }

    ans.to_string()
}
pub fn part_two(input: &str) -> String {
    let rows = input.lines().collect::<Vec<&str>>();
    let n_rows = rows.len();
    let n_cols = input.lines().map(|line| line.len()).max().unwrap();

    let mut ans = 0;
    for r in 0..n_rows {
        for c in 0..n_cols {
            let ch = rows[r].chars().nth(c).unwrap();
            //look left
            let mut left = 0;
            for cc in (0..c).rev() {
                left += 1;
                if rows[r].chars().nth(cc).unwrap() >= ch {
                    break;
                }
            }

            let mut right = 0;
            for cc in (c+1)..n_cols {
                right += 1;
                if rows[r].chars().nth(cc).unwrap() >= ch {
                    break;
                }
            }

            let mut up = 0;
            for rr in (0..r).rev() {
                up += 1;
                if rows[rr].chars().nth(c).unwrap() >= ch {
                    break;
                }
            }

            let mut down = 0;
            for rr in (r+1)..n_rows {
                down += 1;
                if rows[rr].chars().nth(c).unwrap() >= ch {
                    break;
                }
            }

            let val = left * right * up * down;
            if val > ans {
                ans = val;
            }
        }
    }
    ans.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, "21");
    }
    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, "8");
    }
}
