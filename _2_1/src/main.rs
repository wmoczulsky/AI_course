#![feature(slice_patterns)]

use std::io;
use std::io::BufRead;

type Board = Vec<Vec<bool>>;


const PPB1: usize = 100;
const PPB2: usize = 100;

fn rand() -> usize {
    static mut SEED: i64 = 123456789;
    unsafe {
        SEED = (1103515245 * SEED + 12345) % (1 << 31);
        return SEED as usize;
    }
}

fn generate_board(rows_n: usize, cols_n: usize) -> Board {
    let mut board = Vec::new();
    for i in 0..rows_n {
        board.push(Vec::new());
        for _ in 0..cols_n {
            board[i].push(rand() % 2 == 0);
        }
    }

    board
}

fn row_as_nums(row: &Vec<bool>) -> Vec<usize> {
    let mut nums = Vec::new();

    let mut prev = false;
    let mut streak = 0;

    for &color in row {
        if prev == color {
            streak += 1;
        } else {
            if streak > 0 && prev == true {
                nums.push(streak);
            }

            streak = 1;
        }

        prev = color;
    }

    if streak > 0 && prev == true {
        nums.push(streak);
    }

    nums
}

fn row_diff(row: &Vec<bool>, target_row: &Vec<usize>) -> usize {
    let as_nums = row_as_nums(&row);

    let mut diff = 0;

    for i in as_nums.len()..target_row.len() {
        diff += target_row[i] * target_row[i];
    }

    for i in target_row.len()..as_nums.len() {
        diff += as_nums[i] * as_nums[i];
    }

    diff += as_nums
        .iter()
        .zip(target_row.iter())
        .map(|(&a, &b)| ((a as isize - b as isize) * (a as isize - b as isize)) as usize)
        .sum::<usize>();

    diff
}

fn col_diff(board: &Board, col_n: usize, target_col: &Vec<usize>) -> usize {
    let mut as_row = Vec::new();

    for row_n in 0..board.len() {
        as_row.push(board[row_n][col_n]);
    }

    return row_diff(&as_row, target_col);
}

fn get_bad_rows(board: &mut Board, target_rows: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut bad = Vec::new();
    for row_n in 0..board.len() {
        if row_diff(&board[row_n], &target_rows[row_n]) > 0 {
            bad.push(row_n);
        }
    }
    bad
}

fn get_bad_cols(board: &mut Board, target_cols: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut bad = Vec::new();
    for col_n in 0..board[0].len() {
        if col_diff(&board, col_n, &target_cols[col_n]) > 0 {
            bad.push(col_n);
        }
    }
    bad
}

fn improvement(board: &mut Board, row_n: usize, col_n: usize, target_row: &Vec<usize>, target_col: &Vec<usize>) -> isize {
    let a: isize = row_diff(&board[row_n], &target_row) as isize + col_diff(&board, col_n, target_col) as isize;

    board[row_n][col_n] = !board[row_n][col_n];

    let b: isize = row_diff(&board[row_n], &target_row) as isize + col_diff(&board, col_n, target_col) as isize;

    board[row_n][col_n] = !board[row_n][col_n];

    return a - b;
}

fn fix_row(mut board: &mut Board, row_n: usize, target_rows: &Vec<Vec<usize>>, target_cols: &Vec<Vec<usize>>) {
    let mut max_imp = 0;
    let mut max_col_n = 0;

    for col_n in 0..board[row_n].len() {
        let imp = improvement(&mut board, row_n, col_n, &target_rows[row_n], &target_cols[col_n]);
        if imp > max_imp {
            max_imp = imp;
            max_col_n = col_n;
        }
    }

    if rand() % 1000 < PPB1 {
        max_col_n = rand() % board[row_n].len();
    }

    board[row_n][max_col_n] = !board[row_n][max_col_n];
}

fn fix_col(mut board: &mut Board, col_n: usize, target_rows: &Vec<Vec<usize>>, target_cols: &Vec<Vec<usize>>) {
    let mut max_imp = 0;
    let mut max_row_n = 0;

    for row_n in 0..board.len() {
        let imp = improvement(&mut board, row_n, col_n, &target_rows[row_n], &target_cols[col_n]);
        if imp > max_imp {
            max_imp = imp;
            max_row_n = row_n;
        }
    }

    if rand() % 1000 < PPB1 {
        max_row_n = rand() % board.len();
    }

    board[max_row_n][col_n] = !board[max_row_n][col_n];
}


fn play(mut board: &mut Board, target_rows: &Vec<Vec<usize>>, target_cols: &Vec<Vec<usize>>) {
    let mut step = 0;
    loop {
        step += 1;
        if step % 2 == 0 {
            let bad_rows = get_bad_rows(board, target_rows);

            if bad_rows.len() == 0 {
                if get_bad_cols(board, target_cols).len() == 0 {
                    return
                } else {
                    continue
                }
            }

            let mut row = bad_rows[rand() % bad_rows.len()];

            if rand() % 1000 < PPB2 {
                row = rand() % board.len();
            }

            fix_row(&mut board, row, target_rows, target_cols);
        } else {
            let bad_cols = get_bad_cols(board, target_cols);

            if bad_cols.len() == 0 {
                if get_bad_rows(board, target_rows).len() == 0 {
                    return
                } else {
                    continue
                }
            }

            let mut col = bad_cols[rand() % bad_cols.len()];

            if rand() % 1000 < PPB2 {
                col = rand() % board[0].len();
            }

            fix_col(&mut board, col, target_rows, target_cols);
        }

        if step % 1000 == 0 {
            print_board(board);
        }
    }
}


fn read_next_line_of_numbers() -> Vec<usize> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line
        .split(" ")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}


fn print_board(board: &Board){
    for r in 0..board.len() {
        for c in 0..board[0].len() {
            if board[r][c] == true {
                print!("#");
            }else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
    println!("");
}

fn main() {
    let nums = read_next_line_of_numbers();

    let rows_n = nums[0];
    let cols_n = nums[1];

    let mut rows = Vec::new();
    let mut cols = Vec::new();

    for _ in 0..rows_n {
        rows.push(read_next_line_of_numbers());
    }

    for _ in 0..cols_n {
        cols.push(read_next_line_of_numbers());
    }

    let mut board = generate_board(rows_n, cols_n);

    play(&mut board, &rows, &cols);

    print_board(&board);
}


#[cfg(test)]
mod tests {
    #[test]
    fn row_as_nums_test() {
        use row_as_nums;
        assert_eq!(row_as_nums(&vec![]), vec![]);
        assert_eq!(row_as_nums(&vec![true]), vec![1]);
        assert_eq!(row_as_nums(&vec![false, false, false]), vec![]);
        assert_eq!(row_as_nums(&vec![true, false, true]), vec![1, 1]);
        assert_eq!(row_as_nums(&vec![false, true, false]), vec![1]);
        assert_eq!(row_as_nums(&vec![true, false, false]), vec![1]);
        assert_eq!(row_as_nums(&vec![false, false, true]), vec![1]);
        assert_eq!(row_as_nums(&vec![false, true, true]), vec![2]);
        assert_eq!(row_as_nums(&vec![true, true, false]), vec![2]);
    }
}