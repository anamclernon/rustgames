use raylib::prelude::*;

pub const SCREEN_HEIGHT: i32 = 1000;
pub const SCREEN_WIDTH: i32 = 1000;
pub const MAX_COLOURS_COUNT: usize = 8;
pub const REC_SIZE: i32 = 40;
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

pub fn draw(
    d: &mut RaylibDrawHandle,
    go_back: &bool,
    new_function: &bool,
    warn_inj: &bool,
    save_points: &mut Vec<(f32, f32, i32)>,
    colour_sel: &i32,
    diff: &f32,
) {
    // make consts in another file
    d.clear_background(Color::WHITE);
    d.draw_rectangle(0, 0, SCREEN_HEIGHT, 50, Color::RAYWHITE);

    // Drawing grid
    for n in 1..31 {
        let modi = (2 * ((n == 15) as i32)) as f32;
        d.draw_line_ex(
            Vector2::new(0.0, (SCREEN_HEIGHT / 30 * n) as f32),
            Vector2::new(SCREEN_WIDTH as f32, (SCREEN_HEIGHT / 30 * n) as f32),
            1.0 + modi,
            Color::BLACK,
        );
        d.draw_line_ex(
            Vector2::new((SCREEN_WIDTH / 30 * n) as f32, 0.0),
            Vector2::new((SCREEN_WIDTH / 30 * n) as f32, SCREEN_HEIGHT as f32),
            1.0 + modi,
            Color::BLACK,
        );
    }

    d.draw_text(
        "(0,0)",
        SCREEN_WIDTH / 2,
        SCREEN_HEIGHT / 2,
        25,
        Color::BLUE,
    );

    d.draw_text("(-15,0)", 10, SCREEN_HEIGHT / 2, 25, Color::BLUE);
    d.draw_text(
        "(15,0)",
        SCREEN_WIDTH - 60,
        SCREEN_HEIGHT / 2,
        25,
        Color::BLUE,
    );

    // Draw colour selection rectangles
    let mut j: u32 = 0;
    for colour in COLOURS {
        d.draw_rectangle(
            (10 + 45 * j).try_into().unwrap(),
            10,
            REC_SIZE,
            REC_SIZE,
            colour,
        );
        d.draw_rectangle_lines(10, 10, 30, 30, Color::LIGHTGRAY); //what does this do
        j += 1;
    }
    //put point on selected colour
    d.draw_circle(10 + 45 * *colour_sel + 20, 30, 5.0, Color::BLACK);

    //button for back and next function
    d.draw_text("Back", SCREEN_WIDTH - 70, 7, 25, Color::BLACK);
    d.draw_text(
        "Give me a new function",
        SCREEN_WIDTH - 290,
        SCREEN_WIDTH - 40,
        25,
        Color::BLACK,
    );

    let s: String = diff.to_string();
    d.draw_text(
        &format!("{} {}", "Closeness to ", s),
        20,
        SCREEN_HEIGHT - 40,
        25,
        Color::BLACK,
    );

    if *go_back {
        save_points.pop();
    }

    if *new_function {
        *save_points = Vec::new();
    }
    if *warn_inj {
        d.draw_text(
            "Functions must be injective!",
            SCREEN_WIDTH / 5,
            SCREEN_HEIGHT / 5,
            50,
            Color::RED,
        )
    }

    //redraw past
    for point in save_points {
        d.draw_circle(
            point.0 as i32,
            point.1 as i32,
            BRUSH_SIZE,
            COLOURS[point.2 as usize],
        );
    }
}
