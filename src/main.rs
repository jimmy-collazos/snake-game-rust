extern crate piston_window;
extern crate rand;

mod draw;
mod snake;
mod game;
mod constants;

use piston_window::*;
use constants::*;

use crate::game::Game;
use crate::draw::to_coord_u32;




fn main() {
    let (width, height) = CANVAS_SIZE;

    let mut window: PistonWindow =
        WindowSettings::new(WINDOW_TITLE, [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(WINDOW_BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}