use std::time::{Duration, Instant};

use color_eyre::eyre::Result;
use crossterm::event::{self, KeyCode};
use rand::rng;
use ratatui::{
    DefaultTerminal, Frame,
    macros::{horizontal, vertical},
    style::Color,
    symbols::Marker,
    widgets::{
        Block,
        canvas::{Canvas, Points},
    },
};
use tetris_logic::{Game, GameState, Input, InputAction, Tetromino};

fn main() -> Result<()> {
    color_eyre::install()?;

    ratatui::run(run)
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let rng = rng();

    let mut game = Game::new(rng);

    let mut last_update = Instant::now();

    loop {
        // Input handling

        let mut inputs: Vec<Input> = Vec::new();
        if event::poll(Duration::ZERO)? {
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left => {
                        inputs.push(Input::new(InputAction::Left, last_update.elapsed()))
                    }
                    KeyCode::Right => {
                        inputs.push(Input::new(InputAction::Right, last_update.elapsed()))
                    }
                    KeyCode::Up => inputs.push(Input::new(
                        InputAction::RotateClockwise,
                        last_update.elapsed(),
                    )),
                    KeyCode::Char('z') => inputs.push(Input::new(
                        InputAction::RotateCounterclockwise,
                        last_update.elapsed(),
                    )),
                    KeyCode::Char('c') => {
                        inputs.push(Input::new(InputAction::Hold, last_update.elapsed()))
                    }
                    _ => (),
                }
            }
        }

        let state = game.render_tick(last_update.elapsed(), inputs);
        last_update = Instant::now();

        // Rendering

        terminal
            .draw(|frame| render(frame, state))
            .expect("it's probably fine");
    }
}

fn render(frame: &mut Frame, state: &GameState) {
    let canvas = Canvas::default()
        .block(Block::bordered().title("Tetris"))
        .marker(Marker::HalfBlock)
        .x_bounds([0.0, 9.0])
        .y_bounds([0.0, 19.0])
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

    let horizontal = horizontal![*=1, ==12, *=1];
    let vertical = vertical![*=1, ==12, *=1];

    let area = horizontal.areas::<3>(frame.area())[1];
    let area = vertical.areas::<3>(area)[1];

    frame.render_widget(canvas, area);
}
