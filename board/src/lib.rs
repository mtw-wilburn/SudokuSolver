use std::collections::HashMap;
use std::collections::HashSet;

pub struct Puzzle {
    input: Vec<Vec<char>>,
    output: Vec<Vec<char>>,
    board: Vec<usize>,
    scratch: HashMap<usize, HashSet<usize>>,
}

impl Puzzle {
    pub fn new(input: &Vec<Vec<char>>) -> Self {
        let mut b: Vec<usize> = Vec::new();
        for row in 0..9 {
            for col in 0..9 {
                let val = input[row][col];
                let val = match val {
                    '.' => 0,
                    _ => (val.to_string()).parse::<usize>().unwrap(),
                };
                b.push(val);
            }
        }

        Self {
            input: input.clone(),
            output: Vec::new(),
            board: b,
            scratch: HashMap::new(),
        }
    }

    pub fn prn_in(&self) {
        for i in &self.input {
            println!("{:?}", i);
        }
    }

    pub fn prn_out(&self) {
        for i in &self.output {
            println!("{:?}", i);
        }
    }

    pub fn prn_board(&mut self) {
        self.gen_out();
        self.prn_out();
    }

    pub fn prn_scratch(&self) {
        for i in 0..81 {
            if self.scratch.contains_key(&i) {
                let mut s: String = "Idx".to_string();
                s += &format!("[{}] : ", i);

                let item: &HashSet<usize> = self.scratch.get(&i).unwrap();
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

    fn gen_out(&mut self) {
        let mut b: Vec<Vec<char>> = Vec::new();
        for i in 0..9 {
            let v = self.row_data(i * 9);
            let x: Vec<char> = v
                .iter()
                .map(|c| {
                    let mut ch: char = '.';
                    if *c != 0 {
                        ch = char::from_digit(*c as u32, 10).unwrap();
                    }
                    ch
                })
                .collect::<Vec<_>>();
            b.push(x);
        }
        self.output = b;
    }

    fn row_data(&self, idx: usize) -> Vec<usize> {
        let mut v: Vec<usize> = Vec::new();
        for i in self.row_indexes(idx) {
            v.push(self.board[i]);
        }
        v
    }

    fn col_data(&self, idx: usize) -> Vec<usize> {
        let mut v: Vec<usize> = Vec::new();
        for i in self.col_indexes(idx) {
            v.push(self.board[i])
        }
        v
    }

    fn box_data(&self, idx: usize) -> Vec<usize> {
        let mut v: Vec<usize> = Vec::new();
        for i in self.box_indexes(idx) {
            v.push(self.board[i]);
        }
        v
    }

    fn possibilities(&self, idx: usize) -> HashSet<usize> {
        let mut hs: HashSet<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
        for x in self.row_data(idx) {
            hs.remove(&x);
        }

        for x in self.col_data(idx) {
            hs.remove(&x);
        }

        for x in self.box_data(idx) {
            hs.remove(&x);
        }
        hs
    }

    fn rebuild_scratch(&mut self) {
        self.scratch.clear();
        for i in 0..81 {
            if self.board[i] == 0 {
                self.scratch.insert(i, self.possibilities(i));
            }
        }
    }

    fn row_indexes(&self, idx: usize) -> Vec<usize> {
        let b = idx - (idx % 9);
        let e = b + 9;
        let mut v: Vec<usize> = Vec::new();
        for i in b..e {
            v.push(i);
        }
        v
    }

    fn col_indexes(&self, idx: usize) -> Vec<usize> {
        let offset = idx % 9;
        let mut v: Vec<usize> = Vec::new();
        for i in 0..9 {
            v.push(offset + (i * 9));
        }
        v
    }

    fn box_indexes(&self, idx: usize) -> Vec<usize> {
        let mut offset = idx - (idx % 3);
        offset = offset - (((offset / 9) % 3) * 9);
        let mut v: Vec<usize> = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                v.push((offset + j) + (i * 9));
            }
        }
        v
    }

    // Updates the scratch pad for a cell being updated with a particular value.
    fn update_scratch(&mut self, idx: usize, val: usize, del_idx: bool) {
        if del_idx {
            self.scratch.remove(&idx);
        }
        for i in self.row_indexes(idx) {
            let mut empty = false;
            if let Some(x) = self.scratch.get_mut(&i) {
                x.remove(&val);
                if x.len() == 0 {
                    empty = true;
                }
            }
            if empty {
                self.scratch.remove(&i);
            }
        }

        for i in self.col_indexes(idx) {
            let mut empty = false;
            if let Some(x) = self.scratch.get_mut(&i) {
                x.remove(&val);
                if x.len() == 0 {
                    empty = true;
                }
            }
            if empty {
                self.scratch.remove(&i);
            }
        }

        for i in self.box_indexes(idx) {
            let mut empty = false;
            if let Some(x) = self.scratch.get_mut(&i) {
                x.remove(&val);
                if x.len() == 0 {
                    empty = true;
                }
            }
            if empty {
                self.scratch.remove(&i);
            }
        }
    }

    pub fn get_indexes_with_only_two_solutions(&self) -> Vec<usize> {
        let mut v: Vec<usize> = Vec::new();
        for i in 0..81 {
            if let Some(x) = self.scratch.get(&i) {
                if x.len() == 2 {
                    v.push(i);
                }
            }
        }
        v
    }

    // fn get_row_number(&self, idx: usize) -> usize {}
    // fn get_col_number(&self, idx: usize) -> usize {}
    // fn get_box_number(&self, idx: usize) -> usize {}
    pub fn update_scratch_with_two_solutions(&mut self) -> bool {
        let mut changed = false;
        let mut retry = true;
        while retry {
            retry = false;
            let list = self.get_indexes_with_only_two_solutions();
            for i in 0..list.len() {
                let a = self.scratch.get(&list[i]).unwrap().clone();
                for j in 0..list.len() {
                    if i == j {
                        continue;
                    }
                    let b = self.scratch.get(&list[j]).unwrap().clone();
                    let diff: HashSet<_> = a.difference(&b).collect();
                    if diff.len() == 0 {
                        if self.row_indexes(list[i]).contains(&list[j]) {
                            for x in self.row_indexes(list[i]) {
                                if x == list[i] || x == list[j] {
                                    continue;
                                }
                                if let Some(set) = self.scratch.get_mut(&x) {
                                    for val in a.iter() {
                                        retry = set.remove(val);
                                        if retry {
                                            changed = true;
                                        }
                                    }
                                }
                            }
                        }

                        if self.col_indexes(list[i]).contains(&list[j]) {
                            for x in self.col_indexes(list[i]) {
                                if x == list[i] || x == list[j] {
                                    continue;
                                }
                                if let Some(set) = self.scratch.get_mut(&x) {
                                    for val in a.iter() {
                                        retry = set.remove(val);
                                        if retry {
                                            changed = true;
                                        }
                                    }
                                }
                            }
                        }

                        if self.box_indexes(list[i]).contains(&list[j]) {
                            for x in self.box_indexes(list[i]) {
                                if x == list[i] || x == list[j] {
                                    continue;
                                }
                                if let Some(set) = self.scratch.get_mut(&x) {
                                    for val in a.iter() {
                                        retry = set.remove(val);
                                        if retry {
                                            changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        changed
    }

    pub fn solve(&mut self) -> bool {
        let mut solved = true;
        self.rebuild_scratch();

        while self.scratch.len() > 0 {
            if !self.solve_method_1() {
                if !self.solve_method_2() {
                    if !self.solve_method_3() {
                        if !self.update_scratch_with_two_solutions() {
                            solved = false;
                            break;
                        }
                    }
                }
            }
        }

        solved
    }

    // Just looks for cells with only one possible solution
    fn solve_method_1(&mut self) -> bool {
        // let mut found = false;
        // for (k, v) in &self.scratch {
        //     if v.len() == 1 {
        //         let vals = v.into_iter().collect::<Vec<_>>();
        //         self.board[*k] = *vals[0];
        //         found = true;
        //     }
        // }
        // found

        let mut found = false;
        for i in 0..81 {
            if self.scratch.contains_key(&i) {
                if self.scratch.get(&i).unwrap().len() == 1 {
                    let v = self
                        .scratch
                        .get(&i)
                        .unwrap()
                        .into_iter()
                        .collect::<Vec<_>>()[0];
                    self.board[i] = *v;
                    self.update_scratch(i, *v, true);
                    found = true;
                }
            }
        }
        found
    }

    // examine each possible solution for a cell, if the empty cells for that row
    // cannot have that value because their respective cols already contain that value
    // we've found the only possible value for the cell
    fn solve_method_2(&mut self) -> bool {
        let mut found = true;
        for i in 0..81 {
            if self.scratch.contains_key(&i) {
                let b = i - (i % 9);
                let e = b + 9;
                let mut r = None;
                for v in self.scratch.get(&i).unwrap().iter() {
                    found = true;
                    for j in b..e {
                        if j == i {
                            continue;
                        } else {
                            if self.board[j] == 0 {
                                if self.possibilities(j).contains(v) {
                                    found = false;
                                    break;
                                }
                            }
                        }
                    }
                    if found {
                        r = Some(v);
                        break;
                    }
                }

                match r {
                    Some(x) => {
                        self.board[i] = *Some(x).unwrap();
                        self.update_scratch(i, *Some(x).unwrap(), true);
                        break;
                    }
                    None => continue,
                }
            }
        }
        found
    }

    // examine each possible solution for a cell, if the empty cells for that col
    // cannot have that value because their respective rows already contain that value
    // we've found the only possible value for the cell
    fn solve_method_3(&mut self) -> bool {
        let mut found = true;
        for i in 0..81 {
            if self.scratch.contains_key(&i) {
                let offset = i % 9;
                let mut r = None;
                for v in self.scratch.get(&i).unwrap().iter() {
                    found = true;
                    for j in 0..9 {
                        if i == offset + (j * 9) {
                            continue;
                        } else {
                            if self.board[offset + (j * 9)] == 0 {
                                if self.possibilities(offset + (j * 9)).contains(v) {
                                    found = false;
                                    break;
                                }
                            }
                        }
                    }
                    if found {
                        r = Some(v);
                        break;
                    }
                }

                match r {
                    Some(x) => {
                        self.board[i] = *Some(x).unwrap();
                        self.update_scratch(i, *Some(x).unwrap(), true);
                        break;
                    }
                    None => continue,
                }
            }
        }
        found
    }
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
        let want = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];

        let p = Puzzle::new(&data());

        assert_eq!(81, p.board.len());
        assert_eq!(p.board, want);
    }

    #[test]
    fn rows() {
        let p = Puzzle::new(&data());
        let want = vec![0, 9, 8, 0, 0, 0, 0, 6, 0];

        let row = p.row_data(18);
        assert_eq!(row, want);

        let row = p.row_data(20);
        assert_eq!(row, want);

        let row = p.row_data(26);
        assert_eq!(row, want);
    }

    #[test]
    fn cols() {
        let p = Puzzle::new(&data());
        let want = vec![5, 6, 0, 8, 4, 7, 0, 0, 0];

        let col = p.col_data(0);
        assert_eq!(col, want);

        let col = p.col_data(9);
        assert_eq!(col, want);

        let col = p.col_data(72);
        assert_eq!(col, want);

        let want = vec![7, 9, 0, 6, 0, 2, 0, 1, 8];
        let col = p.col_data(31);
        assert_eq!(col, want);
    }

    #[test]
    fn subbox() {
        let p = Puzzle::new(&data());

        let want = vec![5, 3, 0, 6, 0, 0, 0, 9, 8];
        let sbox = p.box_data(0);
        assert_eq!(sbox, want);

        let sbox = p.box_data(2);
        assert_eq!(sbox, want);

        let sbox = p.box_data(9);
        assert_eq!(sbox, want);

        let sbox = p.box_data(10);
        assert_eq!(sbox, want);

        let sbox = p.box_data(19);
        assert_eq!(sbox, want);

        let sbox = p.box_data(20);
        assert_eq!(sbox, want);

        let want = vec![0, 0, 0, 4, 1, 9, 0, 8, 0];
        let sbox = p.box_data(77);
        assert_eq!(sbox, want);
    }

    #[test]
    fn posible() {
        let p = Puzzle::new(&data());

        let want = vec![3, 4];
        let pos = p.possibilities(22);
        let v: Vec<usize> = pos.into_iter().collect();
        for x in want {
            assert!(v.contains(&x));
        }
    }
}
