use board::*;

fn main() {
    #![allow(unused_variables)]
    // Dificult
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

    //Expert
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

    //Master
    let vals3 = vec![
        vec!['.', '9', '.', '7', '.', '1', '.', '.', '.'],
        vec!['.', '.', '.', '4', '.', '.', '.', '.', '.'],
        vec!['7', '.', '.', '.', '.', '6', '.', '.', '.'],
        vec!['.', '1', '.', '.', '.', '.', '.', '.', '4'],
        vec!['.', '.', '.', '.', '9', '5', '.', '.', '7'],
        vec!['6', '.', '8', '.', '4', '.', '.', '9', '.'],
        vec!['8', '.', '.', '3', '.', '.', '7', '.', '.'],
        vec!['.', '.', '4', '.', '5', '.', '.', '.', '2'],
        vec!['.', '2', '9', '.', '.', '.', '.', '5', '8'],
    ];

    //Extreme
    let vals4 = vec![
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['9', '.', '.', '.', '7', '.', '.', '.', '3'],
        vec!['.', '2', '.', '.', '6', '.', '.', '5', '.'],
        vec!['.', '1', '.', '.', '.', '.', '.', '.', '8'],
        vec!['.', '7', '.', '.', '2', '.', '.', '.', '9'],
        vec!['.', '5', '.', '7', '1', '.', '3', '.', '.'],
        vec!['2', '.', '.', '.', '.', '6', '8', '.', '.'],
        vec!['4', '.', '.', '.', '3', '.', '.', '.', '6'],
        vec!['.', '.', '.', '8', '.', '1', '.', '.', '.'],
    ];

    let mut puzzle = Puzzle::new(&vals1);
    puzzle.solve();

    println!("Solving difficult puzzle");
    puzzle.prn_in();
    println!("");
    puzzle.prn_board();

    puzzle = Puzzle::new(&vals2);
    puzzle.solve();

    println!("");
    println!("Solving expert puzzle");
    puzzle.prn_in();
    println!("");
    puzzle.prn_board();

    puzzle = Puzzle::new(&vals3);
    let solved = puzzle.solve();

    println!("");
    println!("Solving master puzzle");
    puzzle.prn_in();
    println!("");
    puzzle.prn_board();

    puzzle = Puzzle::new(&vals4);
    let solved = puzzle.solve();

    println!("");
    println!("Solving evil puzzle");
    puzzle.prn_in();
    println!("");
    puzzle.prn_board();

    if !solved {
        puzzle.prn_scratch();
        // let list = puzzle.get_indexes_with_two_item_solutions();
        let list = puzzle.get_indexes_with_solutions();
        // println!("{:?}", list);
        let list = puzzle.get_indexes_with_solutions();
        // println!("{:?}", list);

        let mut sub = puzzle.box_indexes(66);
        sub.sort();
        println!("{:?}", sub);
    }
}
