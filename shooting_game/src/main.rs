//KAERESTE everything is too fast :(
//maybe restart button so all balls go away

//currently on collision checks
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

mod constants;
use constants::consts::*;

mod functions;

use functions::ball_struct::BallStruct;
use functions::ball_struct::Ball;
use functions::funcs_box::*;
use functions::update_balls::update_ball;

mod draw;
use draw::draw;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_HEIGHT() as i32, SCREEN_WIDTH() as i32)
        .title("Shooter")
        .build();

    let mut balls: Vec<BallStruct> = Vec::new(); //KAERESTE would u put this in a seperate file?
    
    let mut Box: BoxStruct = make_box(BOX_SPEED(), BOX_POSITION(), BOX_LENGTH());

    while !rl.window_should_close() {

        if rl.is_key_pressed(KEY_ENTER) {
            break;
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            let mousePos: Vector2 = rl.get_mouse_position();
            let NewBall: BallStruct = Ball(BALL_SPEED(), (mousePos.x as i32, mousePos.y as i32), BALL_RADIUS());
            balls.push(NewBall);
        } //make this separate function

        update_box(&mut Box);

        update_ball(&mut balls);

        let mut d = rl.begin_drawing(&thread);

        draw(&mut d, &Box, &balls);
    }
}
