pub fn solve_sudoku(grid: &mut [[i32; 9]; 9]) -> bool {
    match find_empty(grid) {
        Some((row, col)) => {
            for num in 1..=9 {
                if is_valid(grid, row, col, num) {
                    grid[row][col] = num;

                    if solve_sudoku(grid) {
                        return true;
                    }

                    grid[row][col] = 0;
                }
            }
            false
        }
        None => true,
    }
}

fn is_valid(grid: &[[i32; 9]; 9], row: usize, col: usize, num: i32) -> bool {
    for i in 0..9 {
        if grid[row][i] == num || grid[i][col] == num {
            return false;
        }
    }

    let start_row = row / 3 * 3;
    let start_col = col / 3 * 3;
    for i in start_row..start_row + 3 {
        for j in start_col..start_col + 3 {
            if grid[i][j] == num {
                return false;
            }
        }
    }
    true
}

fn find_empty(grid: &[[i32; 9]; 9]) -> Option<(usize, usize)> {
    for row in 0..9 {
        for col in 0..9 {
            if grid[row][col] == 0 {
                return Some((row, col));
            }
        }
    }
    None
}

#[allow(dead_code)]
fn print_grid(grid: &[[i32; 9]; 9]) {
    for row in 0..9 {
        for col in 0..9 {
            print!("{} ", grid[row][col]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easy() {
        let mut grid = [
            [0, 1, 3, 8, 0, 0, 4, 0, 5],
            [0, 2, 4, 6, 0, 5, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 9, 3, 0],
            [4, 9, 0, 3, 0, 6, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 5, 0, 0],
            [0, 0, 0, 7, 0, 1, 0, 9, 3],
            [0, 6, 9, 0, 0, 0, 7, 4, 0],
            [0, 0, 0, 2, 0, 7, 6, 8, 0],
            [1, 0, 2, 0, 0, 8, 3, 5, 0],
        ];

        let expected_grid = [
            [6, 1, 3, 8, 7, 9, 4, 2, 5],
            [9, 2, 4, 6, 3, 5, 1, 7, 8],
            [5, 8, 7, 1, 2, 4, 9, 3, 6],
            [4, 9, 8, 3, 5, 6, 2, 1, 7],
            [7, 3, 1, 9, 8, 2, 5, 6, 4],
            [2, 5, 6, 7, 4, 1, 8, 9, 3],
            [8, 6, 9, 5, 1, 3, 7, 4, 2],
            [3, 4, 5, 2, 9, 7, 6, 8, 1],
            [1, 7, 2, 4, 6, 8, 3, 5, 9],
        ];

        assert_eq!(solve_sudoku(&mut grid), true);
        assert_eq!(grid, expected_grid);
    }

    #[test]
    fn test_hard() {
        let mut grid = [
            [0, 0, 2, 0, 0, 0, 0, 4, 1],
            [0, 0, 0, 0, 8, 2, 0, 7, 0],
            [0, 0, 0, 0, 4, 0, 0, 0, 9],
            [2, 0, 0, 0, 7, 9, 3, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 8, 0],
            [0, 0, 6, 8, 1, 0, 0, 0, 4],
            [1, 0, 0, 0, 9, 0, 0, 0, 0],
            [0, 6, 0, 4, 3, 0, 0, 0, 0],
            [8, 5, 0, 0, 0, 0, 4, 0, 0],
        ];

        let expected_grid = [
            [6, 3, 2, 9, 5, 7, 8, 4, 1],
            [4, 9, 1, 6, 8, 2, 5, 7, 3],
            [7, 8, 5, 3, 4, 1, 2, 6, 9],
            [2, 4, 8, 5, 7, 9, 3, 1, 6],
            [3, 1, 9, 2, 6, 4, 7, 8, 5],
            [5, 7, 6, 8, 1, 3, 9, 2, 4],
            [1, 2, 4, 7, 9, 5, 6, 3, 8],
            [9, 6, 7, 4, 3, 8, 1, 5, 2],
            [8, 5, 3, 1, 2, 6, 4, 9, 7],
        ];

        assert_eq!(solve_sudoku(&mut grid), true);
        assert_eq!(grid, expected_grid);
    }
}
