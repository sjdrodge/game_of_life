use rand::prelude::*;

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

    let mut rng = StdRng::seed_from_u64(0x04e043f39bb45bd8);
    let mut board = [[0; 1000]; 1000];
    for _ in 0..1000 {
        *board
            .choose_mut(&mut rng)
            .unwrap()
            .choose_mut(&mut rng)
            .unwrap() = 1;
    }
    let mut board = GolBoard::from_slice(&board);
    const ITERATIONS: usize = 100000;
    let (height, width) = board.dims();
    println!(
        "Iterating {} times on a {}x{} game...",
        ITERATIONS, height, width
    );
    for _ in 0..ITERATIONS {
        board.process_step();
    }
}
