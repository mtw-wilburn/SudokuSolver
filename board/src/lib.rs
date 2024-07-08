use std::collections::HashMap;
use std::collections::HashSet;

pub struct Puzzle {
    prn_methods: bool,
    input: Vec<Vec<char>>,
    output: Vec<Vec<char>>,
    board: Vec<usize>,
    scratch: HashMap<usize, HashSet<usize>>,
}

impl Puzzle {
    pub fn new(input: &Vec<Vec<char>>, pm: bool) -> Self {
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
            prn_methods: pm,
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

    fn sub_row_indexes(&self, idx: usize) -> Vec<usize> {
        let mut v: Vec<usize> = Vec::new();
        let mut row = self.row_indexes(idx);
        row.sort();
        for i in 0..9 {
            if row[i] == idx {
                let offset = i - (i % 3);
                for j in 0..3 {
                    v.push(row[offset + j]);
                }
            }
        }
        v
    }

    fn sub_col_indexes(&self, idx: usize) -> Vec<usize> {
        let mut v: Vec<usize> = Vec::new();
        let mut col = self.col_indexes(idx);
        col.sort();
        for i in 0..9 {
            if col[i] == idx {
                let offset = i - (i % 3);
                for j in 0..3 {
                    v.push(col[offset + j]);
                }
            }
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

    fn update_scratch_row_with_exceptions(
        &mut self,
        idx: usize,
        val: usize,
        except: &Vec<usize>,
    ) -> bool {
        let mut changed: bool = false;
        for i in self.row_indexes(idx) {
            if except.contains(&i) {
                continue;
            }
            let mut empty = false;
            if let Some(x) = self.scratch.get_mut(&i) {
                changed = x.remove(&val);
                if x.len() == 0 {
                    empty = true;
                }
            }
            if empty {
                self.scratch.remove(&i);
            }
        }
        changed
    }

    fn update_scratch_col_with_exceptions(
        &mut self,
        idx: usize,
        val: usize,
        except: &Vec<usize>,
    ) -> bool {
        let mut changed: bool = false;
        for i in self.col_indexes(idx) {
            if except.contains(&i) {
                continue;
            }
            let mut empty = false;
            if let Some(x) = self.scratch.get_mut(&i) {
                changed = x.remove(&val);
                if x.len() == 0 {
                    empty = true;
                }
            }
            if empty {
                self.scratch.remove(&i);
            }
        }
        changed
    }

    fn update_scratch_box_with_exceptions(
        &mut self,
        idx: usize,
        val: usize,
        except: &Vec<usize>,
    ) -> bool {
        let mut changed: bool = false;
        for i in self.box_indexes(idx) {
            if except.contains(&i) {
                continue;
            }
            let mut empty = false;
            if let Some(x) = self.scratch.get_mut(&i) {
                changed = x.remove(&val);
                if x.len() == 0 {
                    empty = true;
                }
            }
            if empty {
                self.scratch.remove(&i);
            }
        }
        changed
    }

    fn get_indexes_with_solutions(&self) -> Vec<usize> {
        let mut v: Vec<usize> = Vec::new();
        for i in 0..81 {
            if let Some(_x) = self.scratch.get(&i) {
                v.push(i);
            }
        }
        v
    }

    fn get_indexes_with_two_item_solutions(&self) -> Vec<usize> {
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

    pub fn solve(&mut self) -> bool {
        let mut solved = true;
        self.rebuild_scratch();

        while self.scratch.len() > 0 {
            if !self.solve_method_1() {
                if !self.solve_method_2() {
                    if !self.solve_method_3() {
                        if !self.solve_method_4() {
                            if !self.solve_method_5() {
                                if !self.solve_method_6() {
                                    if !self.solve_method_7() {
                                        solved = false;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        solved
    }

    // Just looks for cells with a solution set with only one value
    fn solve_method_1(&mut self) -> bool {
        if self.prn_methods {
            println!("Calling Solve Method 1");
        }
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
        if self.prn_methods {
            println!("Calling Solve Method 2");
        }
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
        if self.prn_methods {
            println!("Calling Solve Method 3");
        }
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

    //Determine if any of the unsolved cells in a sub-box can be solved
    //by examining all the cells in that sub-box.  By checking if out of
    //all possibilities is there a cell with one unique possible solution
    fn solve_method_4(&mut self) -> bool {
        if self.prn_methods {
            println!("Calling Solve Method 4");
        }
        let mut changed = false;
        let mut retry = true;
        while retry {
            retry = false;
            let list = self.get_indexes_with_solutions();
            'search: for i in 0..list.len() {
                // println!("examine {}", list[i]);
                let a = self.scratch.get(&list[i]).unwrap().clone();
                let subbox = self.box_indexes(list[i]);
                for val in a.iter() {
                    let mut found = false;
                    for j in 0..subbox.len() {
                        if list[i] == subbox[j] {
                            continue;
                        }
                        if let Some(b) = self.scratch.get(&subbox[j]) {
                            // println!("looking fo {} in subbox[{}] : {:?}", val, subbox[j], b);
                            if b.contains(&val) {
                                found = true;
                                break;
                            }
                        }
                    }

                    if !found {
                        let idx = list[i];
                        // println!("idx {} set to value {}", idx, *val);
                        self.board[idx] = *val;
                        self.update_scratch(idx, *val, true);
                        changed = true;
                        retry = true;
                        break 'search;
                    }
                }
            }
        }
        changed
    }

    //Try and reduce the possible solutions in unsolved cells by examining
    //rows/cols/sub-boxes.  By checking if either of them have two cells with
    //only two possibilities which are the same.
    fn solve_method_5(&mut self) -> bool {
        if self.prn_methods {
            println!("Calling Solve Method 5");
        }
        let mut changed = false;
        let mut retry = true;
        while retry {
            retry = false;
            let list = self.get_indexes_with_two_item_solutions();
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

    //Similar to method_5.  Except we will try and reduce the possible solution set by examining
    //sub-boxes where we can determine the three values for sub-rows or sub-cols.
    fn solve_method_6(&mut self) -> bool {
        if self.prn_methods {
            println!("Calling Solve Method 6");
        }
        //TODO:
        //For each sub-row if each cell is empty and the total of possible solutions is 5
        //  - Then do a union on the three cells, if the count is 3 then these three cells have to
        //    have those values so the remainder of the row can have them removed from their
        //    solution sets.
        //For each sub-col if each cell is empty and the total of possible solutions is 5
        //  - Then do a union on the three cells, if the count is 3 then these three cells have to
        //    have those values os the memainder of the col can have them removed from their
        //    solution sets.
        let mut changed = false;

        for i in 0..81 {
            if i % 3 == 0 {
                let list = self.box_indexes(i);
                for j in 0..9 {
                    //examine sub-box rows
                    let row = self.sub_row_indexes(list[j]);
                    let mut empty = true;
                    for x in 0..3 {
                        if self.scratch.get(&row[x]) == None {
                            empty = false;
                            break;
                        }
                    }
                    if empty {
                        let mut vals: HashSet<usize> = HashSet::new();
                        for x in 0..3 {
                            let p = self.scratch.get(&row[x]).unwrap().clone();
                            vals = vals.union(&p).cloned().collect();
                        }
                        if vals.len() == 3 {
                            let v: Vec<_> = vals.iter().cloned().collect();
                            for x in 0..3 {
                                println!("sub-row changed for {} with value {}", row[x], v[x]);
                                changed |=
                                    self.update_scratch_row_with_exceptions(row[x], v[x], &row);
                                changed |=
                                    self.update_scratch_box_with_exceptions(row[x], v[x], &row);
                            }
                        }
                    }
                    //examine sub-box cols
                    if j < 3 {
                        let col = self.sub_col_indexes(list[j]);
                        let mut empty = true;
                        for x in 0..3 {
                            if self.scratch.get(&col[x]) == None {
                                empty = false;
                                break;
                            }
                        }
                        if empty {
                            let mut vals: HashSet<usize> = HashSet::new();
                            for x in 0..3 {
                                let p = self.scratch.get(&col[x]).unwrap().clone();
                                vals = vals.union(&p).cloned().collect();
                            }
                            if vals.len() == 3 {
                                let v: Vec<_> = vals.iter().cloned().collect();
                                for x in 0..3 {
                                    //println!("sub-col changed for {} with value {}", col[x], v[x]);
                                    changed |=
                                        self.update_scratch_col_with_exceptions(col[x], v[x], &col);
                                    changed |=
                                        self.update_scratch_box_with_exceptions(col[x], v[x], &col);
                                }
                            }
                        }
                    }
                }
            }
        }
        changed
    }

    fn solve_method_7(&mut self) -> bool {
        if self.prn_methods {
            println!("Calling Solve Method 7");
        }
        let mut changed = false;

        for i in 0..81 {
            if i % 3 == 0 {
                let list = self.box_indexes(i);
                for j in 0..list.len() {
                    if self.scratch.get(&list[j]) != None {
                        let vals: HashSet<usize> = self.scratch.get(&list[j]).unwrap().clone();
                        for val in vals {
                            let mut idxs: Vec<usize> = Vec::new();
                            for x in 0..list.len() {
                                // if x == j {
                                //     continue;
                                // }
                                if self.scratch.get(&list[x]) != None {
                                    let v: HashSet<usize> =
                                        self.scratch.get(&list[x]).unwrap().clone();
                                    if v.contains(&val) {
                                        idxs.push(list[x]);
                                    }
                                }
                            }
                            let mut row_set: HashSet<usize> = HashSet::new();
                            let mut col_set: HashSet<usize> = HashSet::new();
                            for r in 0..3 {
                                for c in 0..3 {
                                    let idx = c + (r * 3);
                                    if idxs.contains(&list[idx]) {
                                        row_set.insert(r);
                                        col_set.insert(c);
                                    }
                                }
                            }

                            if row_set.len() == 1 && col_set.len() > 1 {
                                //cleanup overall row for val
                                changed |=
                                    self.update_scratch_row_with_exceptions(list[j], val, &list);
                            }

                            if col_set.len() == 1 && row_set.len() > 1 {
                                //cleanup overall col for val
                                changed |=
                                    self.update_scratch_col_with_exceptions(list[j], val, &list);
                            }
                        }
                    }
                }
            }
        }
        changed
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
    fn scratch_posibilities() {
        let p = Puzzle::new(&data());

        let want = vec![3, 4];
        let pos = p.possibilities(22);
        let v: Vec<usize> = pos.into_iter().collect();
        for x in want {
            assert!(v.contains(&x));
        }
    }

    #[test]
    fn method_4() {
        let vals = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '.', '.', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '.', '.', '.', '7', '9'],
        ];

        //NOTE: the above data is not used in the test only to create Puzzle struct
        //Creating a small scratch for testing (not real)
        let mut p = Puzzle::new(&vals);
        p.scratch.insert(57, HashSet::from([4, 5, 8, 9]));
        p.scratch.insert(66, HashSet::from([2, 5, 9]));
        p.scratch.insert(67, HashSet::from([4, 5, 9]));
        p.scratch.insert(76, HashSet::from([4, 5, 8, 9]));

        let list = p.get_indexes_with_solutions();
        assert_eq!(4, list.len());
        assert_eq!(list, vec![57, 66, 67, 76]);

        p.solve_method_4();
        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&57)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![4, 5, 8, 9], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&67)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![4, 5, 9], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&76)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![4, 5, 8, 9], val);

        assert_eq!(2, p.board[66]);
    }

    #[test]
    fn method_5() {
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

        //NOTE: the above data is not used in the test only to create Puzzle struct
        //Creating a small scratch for testing (not real)
        let mut p = Puzzle::new(&vals);
        p.scratch.insert(0, HashSet::from([2, 3, 4, 5]));
        p.scratch.insert(9, HashSet::from([1, 2, 3, 5]));
        p.scratch.insert(36, HashSet::from([2, 3, 4]));
        p.scratch.insert(55, HashSet::from([5, 6]));
        p.scratch.insert(56, HashSet::from([1, 5, 6]));
        p.scratch.insert(63, HashSet::from([1, 3]));
        p.scratch.insert(64, HashSet::from([3, 6, 7]));
        p.scratch.insert(72, HashSet::from([1, 3]));

        let list = p.get_indexes_with_two_item_solutions();
        assert_eq!(3, list.len());
        assert_eq!(list, vec![55, 63, 72]);

        p.solve_method_5();
        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&0)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![2, 4, 5], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&9)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![2, 5], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&36)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![2, 4], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&56)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![5, 6], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&63)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![1, 3], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&64)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![6, 7], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&72)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![1, 3], val);
    }

    #[test]
    fn method_6() {
        let vals = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        //NOTE: the above data is not used in the test only to create Puzzle struct
        //Creating a small scratch for testing (not real)
        let mut p = Puzzle::new(&vals);
        p.scratch.insert(3, HashSet::from([1, 3, 5, 6, 7]));
        p.scratch.insert(21, HashSet::from([1, 3, 7, 8]));
        p.scratch.insert(30, HashSet::from([3, 6]));
        p.scratch.insert(32, HashSet::from([2, 3, 4, 6, 9]));
        p.scratch.insert(39, HashSet::from([3, 6, 8]));
        p.scratch.insert(41, HashSet::from([3, 4, 6, 8]));
        p.scratch.insert(48, HashSet::from([6, 8]));
        p.scratch.insert(50, HashSet::from([2, 4, 6, 8, 9]));
        p.scratch.insert(75, HashSet::from([3, 5, 6, 7]));

        p.solve_method_6();
        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&3)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![1, 5, 7], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&21)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![1, 7], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&30)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![3, 6], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&32)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![2, 4, 9], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&39)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![3, 6, 8], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&41)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![4], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&48)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![6, 8], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&50)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![2, 4, 9], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&75)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![5, 7], val);
    }

    #[test]
    fn method_7() {
        let vals = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        //NOTE: the above data is not used in the test only to create Puzzle struct
        //Creating a small scratch for testing (not real)
        let mut p = Puzzle::new(&vals);
        p.scratch.insert(4, HashSet::from([1, 3, 5, 6, 7]));
        p.scratch.insert(31, HashSet::from([1, 5]));
        p.scratch.insert(40, HashSet::from([2, 4, 8]));
        p.scratch.insert(49, HashSet::from([2, 4, 5]));
        p.scratch.insert(76, HashSet::from([3, 5, 6, 7]));

        p.solve_method_7();
        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&4)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![1, 3, 6, 7], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&31)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![1, 5], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&40)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![2, 4, 8], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&49)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![2, 4, 5], val);

        let mut val: Vec<usize> = p
            .scratch
            .get_mut(&76)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        val.sort();
        assert_eq!(vec![3, 6, 7], val);
    }
}
