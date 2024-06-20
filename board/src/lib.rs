pub fn create_board(input: &Vec<Vec<char>>) -> Vec<u8> {
    let mut board: Vec<u8> = Vec::new();

    for row in 0..9 {
        for col in 0..9 {
            let val = input[row][col];
            let val = match val {
                '.' => 0,
                _ => (val.to_string()).parse::<u8>().unwrap(),
            };
            board.push(val);
        }
    }
    board
}

pub fn get_row(idx: usize, board: &Vec<u8>) -> Vec<u8> {
    let offset = idx % 9;
    let b = idx - offset;
    let e = b + 9;
    board[b..e].to_vec()
}

pub fn get_col(idx: usize, board: &Vec<u8>) -> Vec<u8> {
    let offset = idx % 9;
    let mut v: Vec<u8> = Vec::new();
    for i in 0..9 {
        v.push(board[offset + (i * 9)])
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> Vec<Vec<char>> {
        let vals = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        vals
    }

    #[test]
    fn create() {
        let board = create_board(&data());
        let want = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];

        assert_eq!(81, board.len());
        assert_eq!(board, want);
    }

    #[test]
    fn rows() {
        let board = create_board(&data());
        let want = vec![0, 9, 8, 0, 0, 0, 0, 6, 0];

        let row = get_row(18, &board);
        assert_eq!(row, want);

        let row = get_row(20, &board);
        assert_eq!(row, want);

        let row = get_row(26, &board);
        assert_eq!(row, want);
    }

    #[test]
    fn cols() {
        let board = create_board(&data());
        let want = vec![5, 6, 0, 8, 4, 7, 0, 0, 0];

        let col = get_col(0, &board);
        assert_eq!(col, want);

        let col = get_col(9, &board);
        assert_eq!(col, want);

        let col = get_col(72, &board);
        assert_eq!(col, want);

        let want = vec![7, 9, 0, 6, 0, 2, 0, 1, 8];
        let col = get_col(31, &board);
        assert_eq!(col, want);
    }
}
