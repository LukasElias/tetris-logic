use std::{
    thread::sleep,
    time::Duration,
};

use rand::rng;
use ratatui::{
    macros::{horizontal, vertical}, style::Color, symbols::Marker, widgets::{
        Block,
        canvas::{Canvas, Points},
    }
};
use tetris_logic::{Game, Tetromino};

fn main() {
    ratatui::run(|terminal| {
        let rng = rng();

        let mut game = Game::new(rng);

        loop {
            let state = game.tick(Duration::from_millis(50), None);

            {
                terminal
                    .draw(|frame| {
                        let canvas = Canvas::default()
                            .block(Block::bordered().title("Tetris"))
                            .marker(Marker::HalfBlock)
                            .x_bounds([0.0, 10.0])
                            .y_bounds([0.0, 20.0])
                            .paint(|ctx| {
                                let mut o_points = vec![];
                                let mut i_points = vec![];
                                let mut t_points = vec![];
                                let mut l_points = vec![];
                                let mut j_points = vec![];
                                let mut s_points = vec![];
                                let mut z_points = vec![];

                                let matrix = state.render_matrix();

                                for (y, row) in matrix.iter().enumerate() {
                                    for (x, mino) in row.iter().enumerate() {
                                        match mino {
                                            Some(Tetromino::O) => o_points.push((x as f64, y as f64)),
                                            Some(Tetromino::I) => i_points.push((x as f64, y as f64)),
                                            Some(Tetromino::T) => t_points.push((x as f64, y as f64)),
                                            Some(Tetromino::L) => l_points.push((x as f64, y as f64)),
                                            Some(Tetromino::J) => j_points.push((x as f64, y as f64)),
                                            Some(Tetromino::S) => s_points.push((x as f64, y as f64)),
                                            Some(Tetromino::Z) => z_points.push((x as f64, y as f64)),
                                            None => (),
                                        }
                                    }
                                }

                                ctx.draw(&Points::new(o_points.as_slice(), Color::Yellow));
                                ctx.draw(&Points::new(i_points.as_slice(), Color::Cyan));
                                ctx.draw(&Points::new(t_points.as_slice(), Color::Magenta));
                                ctx.draw(&Points::new(l_points.as_slice(), Color::Rgb(255, 151, 28)));
                                ctx.draw(&Points::new(j_points.as_slice(), Color::Blue));
                                ctx.draw(&Points::new(s_points.as_slice(), Color::Green));
                                ctx.draw(&Points::new(z_points.as_slice(), Color::Red));
                            });

                        let horizontal = horizontal![*=1, ==10, *=1];
                        let vertical = vertical![*=1, ==10, *=1];

                        let area = horizontal.areas::<3>(frame.area())[1];
                        let area = vertical.areas::<3>(area)[1];

                        frame.render_widget(
                            canvas,
                            area,
                        );
                    })
                    .expect("it's probably fine");
            }

            sleep(Duration::from_millis(50));
        }
    });
}
