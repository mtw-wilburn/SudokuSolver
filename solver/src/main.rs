use board::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    #![allow(unused_variables)]
    let vals1 = vec![
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

    let vals2 = vec![
        vec!['7', '.', '8', '.', '.', '.', '5', '.', '6'],
        vec!['.', '6', '5', '8', '.', '4', '.', '.', '.'],
        vec!['.', '.', '.', '6', '.', '.', '.', '8', '9'],
        vec!['.', '.', '7', '1', '.', '.', '.', '.', '.'],
        vec!['4', '.', '6', '.', '.', '9', '.', '.', '1'],
        vec!['1', '2', '.', '3', '.', '.', '8', '6', '.'],
        vec!['.', '.', '.', '.', '3', '1', '9', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '1', '.', '.'],
        vec!['3', '9', '1', '.', '.', '.', '6', '5', '4'],
    ];

    let vals3 = vec![
        vec!['7', '.', '8', '.', '.', '.', '5', '.', '6'],
        vec!['.', '6', '5', '8', '.', '4', '.', '.', '.'],
        vec!['.', '.', '.', '6', '.', '.', '.', '8', '9'],
        vec!['.', '.', '7', '1', '.', '.', '.', '.', '.'],
        vec!['4', '.', '6', '.', '.', '9', '.', '.', '1'],
        vec!['1', '2', '.', '3', '.', '.', '8', '6', '.'],
        vec!['.', '.', '.', '.', '3', '1', '9', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '1', '.', '.'],
        vec!['3', '9', '1', '.', '.', '.', '6', '5', '4'],
    ];

    let vals = vals3;

    let mut board = create_board(&vals);
    let mut hs: HashMap<u8, HashSet<u8>> = HashMap::new();

    for i in 0..81 {
        if board[i] == 0 {
            hs.insert(i as u8, posibilities(i, &board));
        }
    }

    let mut stuck: bool = false;
    while hs.len() > 0 {
        stuck = true;
        for (k, h) in &hs {
            if h.len() == 1 {
                let v = h.into_iter().collect::<Vec<_>>();
                board[*k as usize] = *v[0];
                stuck = false
            }
        }
        hs.clear();

        for i in 0..81 {
            if board[i] == 0 {
                hs.insert(i as u8, posibilities(i, &board));
            }
        }

        if stuck {
            for i in 0..81 {
                if hs.contains_key(&i) {
                    let item: &HashSet<u8> = hs.get(&i).unwrap();
                    let p = test_for_one_in_row(i as usize, item, &board);
                    match p {
                        Some(x) => {
                            //println!("{} : {}", i, Some(x).unwrap());
                            board[i as usize] = Some(x).unwrap();
                            stuck = false;
                            break;
                        }
                        None => continue,
                    }
                }
            }
        }

        if stuck {
            for i in 0..81 {
                if hs.contains_key(&i) {
                    let item: &HashSet<u8> = hs.get(&i).unwrap();
                    let p = test_for_one_in_col(i as usize, item, &board);
                    match p {
                        Some(x) => {
                            //println!("{} : {}", i, Some(x).unwrap());
                            board[i as usize] = Some(x).unwrap();
                            stuck = false;
                            break;
                        }
                        None => continue,
                    }
                }
            }
        }

        hs.clear();

        for i in 0..81 {
            if board[i] == 0 {
                hs.insert(i as u8, posibilities(i, &board));
            }
        }

        // if stuck {
        //     for i in 0..81 {
        //         if hs.contains_key(&i) {
        //             let item: &HashSet<u8> = hs.get(&i).unwrap();
        //             for v in item {
        //                 if test_col(i as usize, *v, &board) {
        //                     board[i as usize] = *v;
        //                     stuck = false;
        //                     break;
        //                 }
        //             }
        //         }
        //     }
        // }

        if stuck {
            break;
        }
    }

    if stuck {
        for i in &output(&board) {
            println!("{:?}", i);
        }

        for i in 0..81 {
            if hs.contains_key(&i) {
                let mut s: String = "Idx".to_string();
                s += &format!("[{}] : ", i);

                let item: &HashSet<u8> = hs.get(&i).unwrap();
                let mut v: Vec<String> = item
                    .iter()
                    .map(|c| char::from_digit(*c as u32, 10).unwrap().into())
                    .collect::<Vec<_>>();
                v.sort();
                s += &format!("{}", &v[..].join(", "));

                println!("{s}");
            }
        }
    }

    if !stuck {
        for i in &vals {
            println!("{:?}", i);
        }
        println!("");

        let out = output(&board);
        for i in &out {
            println!("{:?}", i);
        }
    }
}
