use raylib::prelude::*;
use rust_math::trigonometry::*;

const SCREEN_HEIGHT: i32 = 1000;
const SCREEN_WIDTH: i32 = 1000;
const no_mf: usize = 3;
const no_mo: usize = 4;
const no_nums: usize = 7;
const math_funcs_str: [&str; no_mf] = ["sin", "tan", "cos"];
const math_ops_str: [&str; no_mo] = ["+", "/", "*", "-"];
const easy_nums: [f32; no_nums] = [1.0, 2.0, 3.0, 4.0, -1.0, -2.0, -3.0];

pub fn function_text(d: &mut RaylibDrawHandle, random_numbers: &(usize, usize, usize)) {
    d.draw_text(
        &format!(
            "{}(x) {} {}",
            math_funcs_str[random_numbers.0], math_ops_str[random_numbers.1], random_numbers.2
        ),
        30,
        SCREEN_HEIGHT - 80,
        25,
        Color::BLACK,
    );
}
