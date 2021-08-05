use game_of_life::GolBoard;

fn main() {
    let board = GolBoard::new(2, 5);
    println!("{:?}", board);

    let board = GolBoard::new_square(2);
    println!("{:?}", board);

    let board = [[true, false], [false, true]];
    let board = GolBoard::from_slice(&board);
    println!("{:?}", board);

    let board = [[0, 1, 0], [0, 1, 0], [0, 1, 0]];
    let mut board = GolBoard::from_slice(&board);
    println!("{:?}", board);
    for _ in 0..2 {
        board.process_step();
        println!("{:?}", board);
    }
}
