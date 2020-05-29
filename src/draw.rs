use piston_window::{rectangle,Context,G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0; //block will size up 25 pixeles

pub fn to_coordinate (game_cord:i32)->f64{
    (game_cord as f64) * BLOCK_SIZE //takes in game cordinates and multiply with block size
}

pub fn to_coordinate_u32 (game_cord:i32)->u32{
    game_cord as u32  
}

pub fn draw_block (x: i32, y: i32, color: Color, cont: &Context, g: &mut G2d) {
    let width = to_coordinate(x);
    let hight = to_coordinate(y);
    //froms a rectangle 
    rectangle(
        color,
        [width,hight,BLOCK_SIZE,BLOCK_SIZE],
        cont.transform,
        g,
    );
}

pub fn draw_rectangle (
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    hight: i32,
    cont: &Context,
    g: &mut G2d,
) {
    let x = to_coordinate(x);
    let y = to_coordinate(y);

    rectangle (
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (hight as f64),
        ],
        cont.transform,
        g,

    );
}