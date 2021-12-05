use std::fmt;

pub struct Board {
    nums: Vec<Vec<i32>>,
    selected: Vec<Vec<bool>>,
}

impl Clone for Board {
    fn clone(&self) -> Board {
        let mut nums: Vec<Vec<i32>> = Vec::new();
        for row in self.nums.iter() {
            let mut row_copy: Vec<i32> = Vec::new();
            for elt in row.iter() {
                row_copy.push(*elt);
            }
            nums.push(row_copy);
        }

        let mut selected: Vec<Vec<bool>> = Vec::new();
        for row in self.selected.iter() {
            let mut row_copy: Vec<bool> = Vec::new();
            for elt in row.iter() {
                row_copy.push(*elt);
            }
            selected.push(row_copy);
        }

        Board { nums, selected }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: Vec<String> = Vec::new();
        output.push("Board {".to_string());

        for row in self.nums.iter() {
            output.push(
                row.iter()
                    .map(|&x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        output.push("}".to_string());
        write!(f, "{}", output.join("\n"))
    }
}

impl Board {
    pub fn new(nums: Vec<Vec<i32>>) -> Board {
        let mut selected = Vec::new();
        for _ in 0..nums.len() {
            let mut row: Vec<bool> = Vec::new();
            for _ in 0..nums[0].len() {
                row.push(false);
            }
            selected.push(row);
        }

        Self { nums, selected }
    }

    pub fn mark_number(self, num: i32) -> Board {
        let mut selected = self.selected;
        for row in 0..self.nums.len() {
            for col in 0..self.nums[0].len() {
                if self.nums[row][col] == num {
                    selected[row][col] = true;
                }
            }
        }
        Board {
            nums: self.nums,
            selected,
        }
    }

    pub fn check_win(&self) -> bool {
        for row in 0..self.selected.len() {
            if self.selected[row].iter().all(|&x| x) {
                return true;
            }
        }

        for col in 0..self.selected.len() {
            if self.selected.iter().map(|x| x[col]).all(|x| x) {
                return true;
            }
        }
        false
    }

    pub fn get_unmarked_sum(&self) -> i32 {
        let mut sum = 0;
        for row in 0..self.nums.len() {
            for col in 0..self.nums[0].len() {
                if !self.selected[row][col] {
                    sum += self.nums[row][col];
                }
            }
        }
        sum
    }
}

pub fn find_first_winning_board(boards: Vec<Board>, nums: Vec<i32>) -> i32 {
    let mut b = boards;
    for num in nums.iter() {
        let mut marked_boards: Vec<Board> = Vec::new();
        for board in b.iter() {
            let marked_board = board.clone().mark_number(*num);
            if marked_board.check_win() {
                return *num * marked_board.get_unmarked_sum();
            }
            marked_boards.push(marked_board);
        }
        b = marked_boards;
    }
    0
}

pub fn find_last_winning_board(boards: Vec<Board>, nums: Vec<i32>) -> i32 {
    let mut b = boards;
    for num in nums.iter() {
        let mut marked_boards: Vec<Board> = Vec::new();
        let mut last_win: Option<Board> = None;
        for board in b.iter() {
            let marked_board = board.clone().mark_number(*num);
            if !marked_board.check_win() {
                marked_boards.push(marked_board);
            } else {
                last_win = Some(marked_board.clone());
            }
        }
        if marked_boards.len() == 0 {
            if let Some(last_board) = last_win.take() {
                return last_board.get_unmarked_sum() * *num;
            }
        }
        b = marked_boards;
    }
    0
}

#[cfg(test)]
mod test {
    use super::{find_first_winning_board, find_last_winning_board, Board};

    #[test]
    fn test_board() {
        let nums = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let mut board = Board::new(nums);
        board = board.mark_number(1);
        board = board.mark_number(5);
        board = board.mark_number(9);
        board = board.mark_number(4);
        assert_eq!(false, board.check_win());
        board = board.mark_number(7);
        assert_eq!(true, board.check_win());
        assert_eq!(19, board.get_unmarked_sum());
    }

    #[test]
    fn test_first_winning_board() {
        let boards: Vec<Board> = vec![
            Board::new(vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ]),
            Board::new(vec![
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6],
            ]),
            Board::new(vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ]),
        ];
        let nums = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        assert_eq!(4512, find_first_winning_board(boards, nums));
    }

    #[test]
    fn test_last_winning_board() {
        let boards: Vec<Board> = vec![
            Board::new(vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ]),
            Board::new(vec![
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6],
            ]),
            Board::new(vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ]),
        ];
        let nums = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        assert_eq!(1924, find_last_winning_board(boards, nums));
    }
}
