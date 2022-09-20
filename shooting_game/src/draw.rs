use super::constants::consts::*;

use super::functions::funcs_box::BoxStruct;
use super::functions::ball_struct::*;

use raylib::prelude::*;

pub fn draw(d: &mut RaylibDrawHandle, Box: &BoxStruct, balls: &Vec<BallStruct>){
    d.clear_background(Color::WHITE);

    d.draw_rectangle(Box.position.0, Box.position.1, Box.length, Box.length, Color::BLUE);

    for ball in balls{
        d.draw_circle(ball.position.0, ball.position.1, ball.radius as f32, Color::GREEN)
    }
}