// ideas:
// show function new button
// why minus diff
// button that says show solution
//create random functions
mod consts;
use crate::consts::SCREEN_HEIGHT;
use crate::consts::SCREEN_WIDTH;

mod draw;
use draw::draw;

mod function_creater;
use function_creater::diff_measure;
use function_creater::function_creater;
use function_creater::function_drawing;

mod function_text;
use function_text::function_text;

use raylib::consts::KeyboardKey::*;
use raylib::consts::*;
use raylib::prelude::*;

fn main() {
    let mut colour_sel: i32 = 7; //default black
    let mut colour_prev: i32 = colour_sel;
    let mut colour_mouse: i32 = 7;
    let mut save_points: Vec<(f32, f32, i32)> = Vec::new();
    let mut warn_inj: bool = false;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_HEIGHT as i32, SCREEN_WIDTH as i32)
        .title("Function drawer")
        .build();

    let mut random_numbers: (usize, usize, usize) = function_creater();

    while !rl.window_should_close() {
        //UPDATE
        let mut go_back = false;

        let mut new_function = false;

        let mut show_function = false;

        if rl.is_key_pressed(KEY_ENTER) {
            break;
        }

        let mouse_pos: Vector2 = rl.get_mouse_position();

        if mouse_pos.x < ((10 + 45 * 8) as f32)
            && mouse_pos.y < 100.0
            && (rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
                || rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON))
        {
            colour_mouse = ((mouse_pos.x - 10.0) / 45.0).floor() as i32;
            colour_sel = colour_mouse;
            colour_prev = colour_sel;
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            if mouse_pos.x as i32 > SCREEN_WIDTH - 70 && mouse_pos.y < 70.0 {
                go_back = true;
            } else if mouse_pos.x as i32 > SCREEN_WIDTH - 300
                && mouse_pos.y as i32 > SCREEN_HEIGHT - 60
            {
                new_function = true;
                random_numbers = function_creater();
            } else if  mouse_pos.x as i32 > SCREEN_WIDTH - 300 && SCREEN_HEIGHT - 60 > mouse_pos.y as i32 && mouse_pos.y as i32 > SCREEN_HEIGHT - 100{
                show_function = true;
                println!("true");
            }
            else if mouse_pos.y > 50.0 {
                warn_inj = false;
                for point in &save_points {
                    if point.0 == mouse_pos.x && point.1 != mouse_pos.y {
                        warn_inj = true;
                    }
                }
                //until new input that is okay it should stay true. okay means: no point is equal to it
                if !warn_inj {
                    save_points.push((mouse_pos.x, mouse_pos.y, colour_sel));
                }
            }
        }

        let mut diff: f32 = diff_measure(&save_points, &random_numbers);
        // DRAW

        let mut d = rl.begin_drawing(&thread);
        draw(
            &mut d,
            &go_back,
            &new_function,
            &warn_inj,
            &mut save_points,
            &colour_sel,
            &diff,
        );

        // write function to draw
        if show_function{
            function_drawing(&mut d, &random_numbers, &colour_sel);
        }

        function_text(&mut d, &random_numbers);
    }
}

// now have to give a random function and then measure somehow

//referencing is not working at all
