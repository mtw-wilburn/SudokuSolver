use board::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
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

    let mut board = create_board(&vals);
    let mut hs: HashMap<u8, HashSet<u8>> = HashMap::new();

    for i in 0..81 {
        if board[i] == 0 {
            hs.insert(i as u8, posibilities(i, &board));
        }
    }

    while hs.len() > 0 {
        for (k, h) in &hs {
            if h.len() == 1 {
                let v = h.into_iter().collect::<Vec<_>>();
                board[*k as usize] = *v[0];
            }
        }
        hs.clear();

        for i in 0..81 {
            if board[i] == 0 {
                hs.insert(i as u8, posibilities(i, &board));
            }
        }
    }

    for i in &vals {
        println!("{:?}", i);
    }
    println!("");

    let out = output(&board);
    for i in &out {
        println!("{:?}", i);
    }
}
