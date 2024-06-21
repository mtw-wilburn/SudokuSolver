use std::collections::HashSet;

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

pub fn output(board: &Vec<u8>) -> Vec<Vec<char>> {
    let mut out: Vec<Vec<char>> = Vec::new();
    for i in 0..9 {
        let v = get_row(i * 9, board);
        let x: Vec<char> = v
            .iter()
            .map(|c| char::from_digit(*c as u32, 10).unwrap())
            .collect::<Vec<_>>();
        out.push(x);
    }
    out
}

pub fn get_row(idx: usize, board: &Vec<u8>) -> Vec<u8> {
    let b = idx - (idx % 9);
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

pub fn get_box(idx: usize, board: &Vec<u8>) -> Vec<u8> {
    let mut offset = idx - (idx % 3);
    offset = offset - (((offset / 9) % 3) * 9);
    let mut v: Vec<u8> = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            v.push(board[(offset + j) + (i * 9)]);
        }
    }
    v
}

pub fn posibilities(idx: usize, board: &Vec<u8>) -> HashSet<u8> {
    let mut hs: HashSet<u8> = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
    for x in &get_row(idx, board) {
        hs.remove(x);
    }

    for x in &get_col(idx, board) {
        hs.remove(x);
    }

    for x in &get_box(idx, board) {
        hs.remove(x);
    }
    hs
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

    #[test]
    fn subbox() {
        let board = create_board(&data());

        let want = vec![5, 3, 0, 6, 0, 0, 0, 9, 8];
        let sbox = get_box(0, &board);
        assert_eq!(sbox, want);

        let sbox = get_box(2, &board);
        assert_eq!(sbox, want);

        let sbox = get_box(9, &board);
        assert_eq!(sbox, want);

        let sbox = get_box(10, &board);
        assert_eq!(sbox, want);

        let sbox = get_box(19, &board);
        assert_eq!(sbox, want);

        let sbox = get_box(20, &board);
        assert_eq!(sbox, want);

        let want = vec![0, 0, 0, 4, 1, 9, 0, 8, 0];
        let sbox = get_box(77, &board);
        assert_eq!(sbox, want);
    }

    #[test]
    fn posible() {
        let board = create_board(&data());

        let want = vec![3, 4];
        let p = posibilities(22, &board);
        let v: Vec<u8> = p.into_iter().collect();
        assert_eq!(v, want);
    }
}
