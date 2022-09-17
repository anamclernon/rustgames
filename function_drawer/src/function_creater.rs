use rand::Rng;
use raylib::prelude::*;
use rust_math::trigonometry::*;

const SCREEN_HEIGHT: i32 = 1000;
const SCREEN_WIDTH: i32 = 1000;

pub const MAX_COLOURS_COUNT: usize = 8;
pub const BRUSH_SIZE: f32 = 10.0;
pub const COLOURS: [Color; MAX_COLOURS_COUNT] = [
    Color::ORANGE,
    Color::RED,
    Color::MAROON,
    Color::GREEN,
    Color::BLUE,
    Color::DARKBLUE,
    Color::PURPLE,
    Color::BLACK,
];

const no_mf: usize = 3;
const no_mo: usize = 4;
const no_nums: usize = 7;
const math_funcs: [fn(x: f32) -> f32; no_mf] = [sin, tan, cos];

const easy_nums: [f32; no_nums] = [1.0, 2.0, 3.0, 4.0, -1.0, -2.0, -3.0];

fn add(a: f32, b: f32) -> f32 {
    a + b
}

fn sub(a: f32, b: f32) -> f32 {
    a - b
}

fn mul(a: f32, b: f32) -> f32 {
    a * b
}

fn div(a: f32, b: f32) -> f32 {
    a / b
}

const math_ops: [fn(x: f32, y: f32) -> f32; no_mo] = [add, div, mul, sub];

pub fn function_creater() -> (usize, usize, usize) {
    let mut rng = rand::thread_rng();

    let ran1: usize = rng.gen_range(0..no_mf);
    let ran2: usize = rng.gen_range(0..no_mo);
    let ran3: usize = rng.gen_range(0..no_nums);

    (ran1, ran2, ran3)
}

pub fn my_function(x: f32, random_numbers: &(usize, usize, usize)) -> f32 {
    let m = math_ops[random_numbers.1];
    let f = math_funcs[random_numbers.0];
    m(f(x), easy_nums[random_numbers.2])
}

fn encode(x: f32) -> f32 {
    //range -15 to 15 to SCREEN_WIDTH
    (SCREEN_WIDTH / 30 * (x as i32 + 15)) as f32
}

pub fn diff_measure(
    save_points: &Vec<(f32, f32, i32)>,
    random_numbers: &(usize, usize, usize),
) -> f32 {
    let mut diff: f32 = 0.0;
    for n in 0..SCREEN_WIDTH {
        for point in &*save_points {
            if point.0 == n as f32 {
                diff += point.1 - encode(my_function(n as f32, random_numbers)) as f32;
            }
        }
    }
    diff
}

pub fn function_drawing( //maybe cant draw entire function in time frame?? no wouldnt make sense
    d: &mut RaylibDrawHandle,
    random_numbers: &(usize, usize, usize),
    colour_sel: &i32,
) {
    for x in -15..15 { // better off doing each pixel then converting for more points
        let mut y = encode(my_function(x as f32, random_numbers)).floor() as i32;
        if encode(my_function(x as f32, random_numbers)).floor() as i32 > SCREEN_HEIGHT{
            y = SCREEN_HEIGHT;
        }
        d.draw_circle(
            encode(x as f32).floor() as i32,
            y,
            BRUSH_SIZE,
            COLOURS[*colour_sel as usize],
        );
    }
}
