use std::collections::HashMap;

use arrayvec::ArrayVec;

// - Parse the whole input into the number list and the boards
// - For each board, iterate through the rows and columns
//      For each row or column, find the highest-numbered index to which some number in the row/column corresponds
//      This is when that row/column is solved
// - For each board, find the row or column that is solved first
// - Among all the boards, the first board wins. Calculate its score
//
// Potential optimization: store boards as the index of the number at each spot
// instead of the number itself. This could save hashmap lookups later on and
// simplify the score logic.

type Board = ArrayVec<ArrayVec<u32, 5>, 5>;

pub fn part1(input: String) {
    let (numbers, boards) = parse_input(&input);

    let number_idx_lookup: HashMap<u32, usize> = numbers
        .iter()
        .copied()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect();

    let (quickest_board, solved_idx) = boards
        .iter()
        .map(|board| find_board_solved_idx(board, &number_idx_lookup))
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    let winning_score = calc_board_score(
        &boards[quickest_board],
        &number_idx_lookup,
        &numbers,
        solved_idx,
    );

    println!("Winning Score: {}", winning_score);
}

pub fn part2(input: String) {
    let (numbers, boards) = parse_input(&input);

    let number_idx_lookup: HashMap<u32, usize> = numbers
        .iter()
        .copied()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect();

    let (last_board, solved_idx) = boards
        .iter()
        .map(|board| find_board_solved_idx(board, &number_idx_lookup))
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    let losing_score = calc_board_score(
        &boards[last_board],
        &number_idx_lookup,
        &numbers,
        solved_idx,
    );

    println!("Losin score: {}", losing_score);
}

fn calc_board_score(
    board: &Board,
    number_lookup: &HashMap<u32, usize>,
    number_list: &[u32],
    winning_idx: usize,
) -> u32 {
    let sum: u32 = board
        .iter()
        .flatten()
        .filter(|n| *number_lookup.get(n).unwrap() > winning_idx)
        .sum();

    sum * number_list[winning_idx]
}

fn find_board_solved_idx(board: &Board, numbers: &HashMap<u32, usize>) -> usize {
    let min_row_solved_idx = board
        .iter()
        .map(|row| find_group_solved_idx(row, numbers))
        .min()
        .unwrap();

    let min_col_solved_idx = rotate_board(board)
        .iter()
        .map(|col| find_group_solved_idx(col, numbers))
        .min()
        .unwrap();

    usize::min(min_row_solved_idx, min_col_solved_idx)
}

fn rotate_board(board: &Board) -> Board {
    let mut new_board: Board = (0..5)
        .map(|_| (0..5).map(|_| 0).collect::<ArrayVec<_, 5>>())
        .collect();

    for (y, row) in board.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            new_board[x][y] = *val
        }
    }

    new_board
}

fn find_group_solved_idx(group: &[u32], numbers: &HashMap<u32, usize>) -> usize {
    group
        .iter()
        .map(|n| *numbers.get(n).unwrap())
        .max()
        .unwrap()
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let numbers = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let boards = input
        .lines()
        .skip(2)
        .map(|l| l.split_ascii_whitespace().map(|s| s.parse().unwrap()))
        .flatten()
        .collect::<Vec<u32>>()
        .chunks(25)
        .map(|board_raw| {
            board_raw
                .chunks(5)
                .map(|row| row.iter().copied().collect())
                .collect()
        })
        .collect();

    (numbers, boards)
}
