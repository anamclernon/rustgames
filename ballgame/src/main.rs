//idea: two balls, one moves randomly and the other one has to try hit it and then gets points for it
// or maybe have to make ball hit one of the corners
// do i have to put this into different functions? draw update etc?
// new idea is shoot something from middle and has to hit flying object, that comes at random speeds
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const SCREEN_HEIGHT: f32 = 900.0;
const SCREEN_WIDTH: f32 = 900.0;

struct Ball {
    position: Vector2,
    color: Color,
    radius: f32,
    speed: Vector2,
}

struct Goal {
    size: i32,
    color: Color,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_HEIGHT as i32, SCREEN_WIDTH as i32)
        .title("Best Game Ever")
        .build();

    rl.set_target_fps(60);

    let mut pause = false;
    let mut frame_count = 0;
    
    let goal1 = Goal{
        size: 100,
        color: Color::BLUE,
    };

    let mut ball = Ball{
        color: Color::RED,
        radius: 10.0,
        position: Vector2::new(320.0, 240.0),
        speed: Vector2::new(-10.0, 10.0)
    }; 

    let mut last_speed: Vector2 = ball.speed;

    while !rl.window_should_close() {

        if rl.is_key_pressed(KEY_SPACE) {pause = !pause;}

        if rl.is_key_pressed(KEY_DOWN) && !pause {ball.position.y += 20.0;}

        if rl.is_key_pressed(KEY_UP) && !pause {ball.position.y -= 20.0;}

        if rl.is_key_pressed(KEY_RIGHT) && !pause {ball.position.x += 20.0;}

        if rl.is_key_pressed(KEY_LEFT) && !pause {ball.position.x -= 20.0;}

        if (ball.speed.x == 0.0) && (ball.speed.y == 0.0) && (pause) {
            ball.speed = last_speed;}

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        d.draw_rectangle(0, 0, goal1.size, goal1.size, goal1.color);

        d.draw_circle_v(ball.position, ball.radius, ball.color);

        d.draw_text("GOAL", 8, 2, 30, Color::BLACK);
        
        ball.position = ball.position + ball.speed;

        if (ball.position.x >= (SCREEN_WIDTH - ball.radius)) || (ball.position.x <= ball.radius) 
            {ball.speed.x = -1.0*ball.speed.x;
            last_speed = ball.speed;}

        if (ball.position.y >= (SCREEN_HEIGHT as f32 - ball.radius)) || (ball.position.y <= ball.radius)
            {ball.speed.y = -1.0*ball.speed.y;
            last_speed = ball.speed;}

        else {frame_count +=1;}

        if pause {
            ball.speed = Vector2::new(0.0, 0.0);
            d.draw_text("PAUSE", (SCREEN_WIDTH / 2.0 -45.0) as i32, (SCREEN_HEIGHT / 2.0-30.0) as i32, 30, Color::GREEN);}

        if ball.position.x <= goal1.size as f32 && ball.position.y <= goal1.size as f32 {
            d.clear_background(Color::WHITE);
            d.draw_text("WIN!",(SCREEN_WIDTH / 2.0 -45.0) as i32, (SCREEN_HEIGHT / 2.0-30.0) as i32, 50, Color::BLACK);
            ball.speed = Vector2::new(0.0, 0.0);
            pause = false;
        }
    }
}
