extern crate piston_window;
extern crate rand;

use piston_window::*;
use piston_window::types::Color;

mod draw;
mod snake;
mod game;

pub use crate::game::Game;
pub use crate::draw::to_coordinate_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new(
        "snake",
        [to_coordinate_u32(width), to_coordinate_u32(height)],
    ).exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event)= window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args(){
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }

    
}
