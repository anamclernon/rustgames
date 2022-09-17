use raylib::prelude::*;
use raylib::KeyboardKey::*;
fn main() {
    
    const SCREEN_HEIGHT: i32 = 1000;
    const SCREEN_WIDTH: i32 = 1000;
    
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_HEIGHT as i32, SCREEN_WIDTH as i32)
        .title("Shooter")
        .build();

    while(!rl.window_should_close()){

    }
}
