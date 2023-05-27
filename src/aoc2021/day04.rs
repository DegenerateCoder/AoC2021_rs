#[derive(Copy, Clone)]
struct BingoNumber {
    number: u32,
    marked: bool,
}

struct BingoBoard {
    board: [[BingoNumber; 5]; 5],
    won: bool,
}

pub fn part01(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut file = file.lines();

    let draw_numbers = file.next().unwrap().split(',');
    let draw_numbers = draw_numbers.map(|x| x.parse::<u32>().unwrap());

    let file = file.filter(|x| !x.trim().is_empty());
    let mut bingo_boards: Vec<BingoBoard> = Vec::new();

    let mut i = 0;
    for line in file {
        i %= 5;
        if i == 0 {
            let number = BingoNumber {
                number: 0,
                marked: false,
            };
            let board = BingoBoard {
                board: [[number; 5]; 5],
                won: false,
            };
            bingo_boards.push(board);
        }
        let line = line.split(' ').filter(|x| !x.trim().is_empty());
        for (j, num) in line.enumerate() {
            bingo_boards.last_mut().unwrap().board[i][j].number = num.parse().unwrap();
        }
        i += 1;
    }

    let mut last_draw_num = 0;
    'draw: for draw_number in draw_numbers {
        for board in bingo_boards.iter_mut() {
            let rows_to_mark = board
                .board
                .iter_mut()
                .filter(|x| x.map(|x| x.number).contains(&draw_number));

            for row in rows_to_mark {
                let row = row.iter_mut().filter(|x| x.number == draw_number);
                row.for_each(|x| {
                    x.marked = true;
                });
            }
            if check_board(&board) {
                board.won = true;
                last_draw_num = draw_number;
                break 'draw;
            }
        }
    }

    let bingo_boards = bingo_boards.iter().filter(|x| x.won);
    let mut unmarked_sum = 0;
    for board in bingo_boards {
        let board = board.board.iter().flatten();
        let board = board.filter(|x| !x.marked);
        unmarked_sum = board.map(|x| x.number).sum();
    }

    unmarked_sum * last_draw_num
}

pub fn part02(file_path: &str) -> u32 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut file = file.lines();

    let draw_numbers = file.next().unwrap().split(',');
    let draw_numbers = draw_numbers.map(|x| x.parse::<u32>().unwrap());

    let file = file.filter(|x| !x.trim().is_empty());
    let mut bingo_boards: Vec<BingoBoard> = Vec::new();

    let mut i = 0;
    for line in file {
        i %= 5;
        if i == 0 {
            let number = BingoNumber {
                number: 0,
                marked: false,
            };
            let board = BingoBoard {
                board: [[number; 5]; 5],
                won: false,
            };
            bingo_boards.push(board);
        }
        let line = line.split(' ').filter(|x| !x.trim().is_empty());
        for (j, num) in line.enumerate() {
            bingo_boards.last_mut().unwrap().board[i][j].number = num.parse().unwrap();
        }
        i += 1;
    }

    let mut last_draw_num = 0;
    let bingo_board_num = bingo_boards.len();
    let mut boards_won = 0;
    'draw: for draw_number in draw_numbers {
        let bingo_boards = bingo_boards.iter_mut().filter(|x| !x.won);
        for board in bingo_boards {
            let rows_to_mark = board
                .board
                .iter_mut()
                .filter(|x| x.map(|x| x.number).contains(&draw_number));

            for row in rows_to_mark {
                let row = row.iter_mut().filter(|x| x.number == draw_number);
                row.for_each(|x| {
                    x.marked = true;
                });
            }
            if check_board(&board) {
                boards_won += 1;
                last_draw_num = draw_number;
                if boards_won == bingo_board_num {
                    break 'draw;
                }
                board.won = true;
            }
        }
    }

    let bingo_boards = bingo_boards.iter().filter(|x| !x.won);
    let mut unmarked_sum = 0;
    for board in bingo_boards {
        let board = board.board.iter().flatten();
        let board = board.filter(|x| !x.marked);
        unmarked_sum = board.map(|x| x.number).sum();
    }

    unmarked_sum * last_draw_num
}

fn check_board(board: &BingoBoard) -> bool {
    let mut marked_in_columns = [0; 5];

    for row in board.board {
        let mut marked_in_row = 0;
        for (i, num) in row.iter().enumerate() {
            if num.marked {
                marked_in_row += 1;
                marked_in_columns[i] += 1;
            }
            if marked_in_row == 5 {
                return true;
            }
        }
    }

    let won_columns = marked_in_columns.iter().filter(|x| **x == 5);
    for _ in won_columns {
        return true;
    }

    false
}
