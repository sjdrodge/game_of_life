use std::time::{Duration, SystemTime};

use piston_window::{color, PistonWindow, WindowSettings};
use rand::prelude::*;

use game_of_life::GolBoard;

fn main() {
    // let board = GolBoard::new(2, 5);
    // println!("{:?}", board);
    //
    // let board = [[true, false], [false, true]];
    // let board = GolBoard::from_slice(&board);
    // println!("{:?}", board);
    //
    // let board = [[0, 1, 0], [0, 1, 0], [0, 1, 0]];
    // let mut board = GolBoard::from_slice(&board);
    // println!("{:?}", board);
    // for _ in 0..2 {
    //     board.process_step();
    //     println!("{:?}", board);
    // }

    let mut rng = thread_rng();
    let mut board = [[0; 200]; 200];
    for _ in 0..5000 {
        *board
            .choose_mut(&mut rng)
            .unwrap()
            .choose_mut(&mut rng)
            .unwrap() = 1;
    }
    let mut board = GolBoard::from_slice(&board);
    let (height, width) = board.dims();
    let background_color = color::hex("1e1e1e");
    let foreground_color = color::hex("d4d4d4");
    const CELL_SIZE: f64 = 5.0;
    let mut window: PistonWindow = WindowSettings::new(
        "Conway's Game of Life",
        [width as f64 * CELL_SIZE, height as f64 * CELL_SIZE],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    const STEP_DELAY: Duration = Duration::from_millis(100);
    let mut previous_update = SystemTime::now();
    while let Some(event) = window.next() {
        if previous_update.elapsed().unwrap() > STEP_DELAY {
            board.process_step();
            previous_update = SystemTime::now();
        }
        window.draw_2d(&event, |context, graphics, _device| {
            piston_window::clear(background_color, graphics);
            for (r, c) in board.alive_cells() {
                piston_window::rectangle(
                    foreground_color,
                    [
                        c as f64 * CELL_SIZE,
                        r as f64 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                    ],
                    context.transform,
                    graphics,
                );
            }
        });
    }
}
