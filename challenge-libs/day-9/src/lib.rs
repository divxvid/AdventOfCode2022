use std::collections::BTreeSet;

#[derive(Debug, Clone)]
enum HeadPosition {
    ABOVE,
    BELOW,
    LEFT,
    RIGHT,
    AboveLeft,
    AboveRight,
    BelowLeft,
    BelowRight,
    SAME
}

#[derive(Debug, Clone)]
struct Rope {
    head: (i32, i32),
    tail_position: (i32, i32),
    tail: HeadPosition,
    tail_movement: BTreeSet<(i32, i32)>
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NOP
}

impl Default for Rope {
    fn default() -> Self {
        let mut positions: BTreeSet<(i32, i32)> = BTreeSet::new();
        positions.insert((0, 0));
        Self {
            head: (0, 0),
            tail_position: (0, 0),
            tail: HeadPosition::SAME,
            tail_movement: positions
        }
    }
}

impl Rope {
    fn move_head(&mut self, new_position: &Move, recordable: bool) -> Option<(Move, Move)> {
        match new_position {
            Move::UP => {
                self.head.0 = self.head.0 + 1;
                return self.move_tail_up(recordable);
            },
            Move::DOWN => {
                self.head.0 = self.head.0 - 1;
                return self.move_tail_down(recordable);
            },
            Move::LEFT => {
                self.head.1 = self.head.1 - 1;
                return self.move_tail_left(recordable);
            },
            Move::RIGHT => {
                self.head.1 = self.head.1 + 1;
                return self.move_tail_right(recordable);
            },
            Move::NOP => None
        }
    }

    fn move_tail_up(&mut self, recordable: bool) -> Option<(Move, Move)> {
        let mut ret_val: Option<(Move, Move)> = None;
        match self.tail {
            HeadPosition::SAME => self.tail = HeadPosition::ABOVE,
            HeadPosition::ABOVE => {
                self.tail_position = (self.head.0-1, self.head.1);
                ret_val = Some((Move::NOP, Move::UP));
            },
            HeadPosition::BELOW => self.tail = HeadPosition::SAME,
            HeadPosition::LEFT => self.tail = HeadPosition::AboveLeft,
            HeadPosition::RIGHT => self.tail = HeadPosition::AboveRight,
            HeadPosition::AboveLeft => {
                self.tail = HeadPosition::ABOVE;
                self.tail_position = (self.head.0-1, self.head.1);
                ret_val = Some((Move::LEFT, Move::UP));
            },
            HeadPosition::AboveRight => {
                self.tail = HeadPosition::ABOVE;
                self.tail_position = (self.head.0-1, self.head.1);
                ret_val = Some((Move::RIGHT, Move::UP));
            },
            HeadPosition::BelowLeft => {
                self.tail = HeadPosition::LEFT
            },
            HeadPosition::BelowRight => {
                self.tail = HeadPosition::RIGHT
            }
        }

        if recordable {
            self.tail_movement.insert(self.tail_position);
        }
        return ret_val;
    }

    fn move_tail_down(&mut self, recordable: bool) -> Option<(Move, Move)> {
        let mut ret_val: Option<(Move, Move)> = None;
        match self.tail {
            HeadPosition::SAME => self.tail = HeadPosition::BELOW,
            HeadPosition::ABOVE => self.tail = HeadPosition::SAME,
            HeadPosition::BELOW => {
                self.tail_position = (self.head.0+1, self.head.1);
                ret_val = Some((Move::NOP, Move::DOWN));
            }
            HeadPosition::LEFT => self.tail = HeadPosition::BelowLeft,
            HeadPosition::RIGHT => self.tail = HeadPosition::BelowRight,
            HeadPosition::AboveLeft => {
                self.tail = HeadPosition::LEFT
            },
            HeadPosition::AboveRight => {
                self.tail = HeadPosition::RIGHT
            },
            HeadPosition::BelowLeft => {
                self.tail = HeadPosition::BELOW;
                self.tail_position = (self.head.0+1, self.head.1);
                ret_val = Some((Move::LEFT, Move::DOWN));
            },
            HeadPosition::BelowRight => {
                self.tail = HeadPosition::BELOW;
                self.tail_position = (self.head.0+1, self.head.1);
                ret_val = Some((Move::RIGHT, Move::DOWN));
            }
        }
        if recordable {
            self.tail_movement.insert(self.tail_position);
        }
        return ret_val;
    }

    fn move_tail_left(&mut self, recordable: bool) -> Option<(Move, Move)> {
        let mut ret_val: Option<(Move, Move)> = None;
        match self.tail {
            HeadPosition::SAME => self.tail = HeadPosition::LEFT,
            HeadPosition::ABOVE => self.tail = HeadPosition::AboveLeft,
            HeadPosition::BELOW => self.tail = HeadPosition::BelowLeft,
            HeadPosition::LEFT => {
                self.tail_position = (self.head.0, self.head.1+1);
                ret_val = Some((Move::NOP, Move::LEFT));
            },
            HeadPosition::RIGHT => self.tail = HeadPosition::SAME,
            HeadPosition::AboveLeft => {
                self.tail = HeadPosition::LEFT;
                self.tail_position = (self.head.0, self.head.1+1);
                ret_val = Some((Move::LEFT, Move::UP));
            },
            HeadPosition::AboveRight => {
                self.tail = HeadPosition::ABOVE
            },
            HeadPosition::BelowLeft => {
                self.tail = HeadPosition::LEFT;
                self.tail_position = (self.head.0, self.head.1+1);
                ret_val = Some((Move::LEFT, Move::DOWN));
            },
            HeadPosition::BelowRight => {
                self.tail = HeadPosition::BELOW
            }
        }
        if recordable {
            self.tail_movement.insert(self.tail_position);
        }
        return ret_val;
    }

    fn move_tail_right(&mut self, recordable: bool) -> Option<(Move, Move)> {
        let mut ret_val: Option<(Move, Move)> = None;
        match self.tail {
            HeadPosition::SAME => self.tail = HeadPosition::RIGHT,
            HeadPosition::ABOVE => self.tail = HeadPosition::AboveRight,
            HeadPosition::BELOW => self.tail = HeadPosition::BelowRight,
            HeadPosition::LEFT => self.tail = HeadPosition::SAME,
            HeadPosition::RIGHT => {
                self.tail_position = (self.head.0, self.head.1-1);
                ret_val = Some((Move::NOP, Move::RIGHT));
            },
            HeadPosition::AboveLeft => {
                self.tail = HeadPosition::ABOVE;
            },
            HeadPosition::AboveRight => {
                self.tail = HeadPosition::RIGHT;
                self.tail_position = (self.head.0, self.head.1-1);
                ret_val = Some((Move::RIGHT, Move::UP));
            },
            HeadPosition::BelowLeft => {
                self.tail = HeadPosition::BELOW;
            },
            HeadPosition::BelowRight => {
                self.tail = HeadPosition::RIGHT;
                self.tail_position = (self.head.0, self.head.1-1);
                ret_val = Some((Move::RIGHT, Move::DOWN));
            }
        }
        if recordable {
            self.tail_movement.insert(self.tail_position);
        }
        return ret_val;
    }
}

pub fn part_one(input: &str) -> String {
    let mut rope: Rope = Rope::default();
    input.lines()
        .for_each(|line| {
            let (mv, amt) = line.split_once(" ").unwrap();
            let amt = amt.parse::<i32>().unwrap();
            let mv = match mv {
                "L" => Move::LEFT,
                "R" => Move::RIGHT,
                "U" => Move::UP,
                "D" => Move::DOWN,
                _ => panic!("omegalul")
            };
            for _ in 0..amt {
                rope.move_head(&mv, true);
            }
        });
    
    rope.tail_movement.len().to_string()
}

pub fn part_two(input: &str) -> String {
    const N_ROPES: usize = 9;
    let mut ropes: Vec<Rope> = vec![Rope::default(); N_ROPES];
    input.lines()
        .for_each(|line| {
            let (mv_main, amt) = line.split_once(" ").unwrap();
            let amt = amt.parse::<i32>().unwrap();
            let mv_main = match mv_main {
                "L" => Move::LEFT,
                "R" => Move::RIGHT,
                "U" => Move::UP,
                "D" => Move::DOWN,
                _ => panic!("omegalul")
            };
            for _a in 0..amt {
                let mut movements: Vec<(usize, Move, bool)> = vec![(0, mv_main, true)];
                while !movements.is_empty() {
                    let (idx, mv, recordable) = movements.pop().unwrap();
                    if idx == N_ROPES {
                        continue;
                    }
                    let tail_movement = ropes[idx].move_head(&mv, recordable);
                    match tail_movement {
                        Some((m1, m2)) => {
                            movements.push((idx+1, m2, recordable));
                            movements.push((idx+1, m1, false));
                        },
                        None => ()
                    }
                }
            }
        });
    
    ropes[N_ROPES-1].tail_movement.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        let result = part_one(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let result = part_two(INPUT);
        assert_eq!(result, "36");
    }
}
