use super::super::constants::consts::*;

use super::ball_struct::BallStruct;

fn balls_out_of_screen(balls: &mut Vec<BallStruct>){
    let mut index: usize = 0;
    let mut index_to_deleted: Vec<usize> = Vec::new();
    for ball in balls.iter(){
        if ball.position.0 > SCREEN_WIDTH() || ball.position.1 > SCREEN_HEIGHT() {
            index_to_deleted.push(index)
        }
        index += 1;
    }
    for index in index_to_deleted{
        balls.remove(index);
    }
}

fn speed_up_balls(balls: &mut Vec<BallStruct>){
    for ball in balls{
        ball.position.0 = ball.position.0 + ball.speed.0;
        ball.position.1 = ball.position.1 + ball.speed.1;
    }
}

pub fn update_ball(balls: &mut Vec<BallStruct>) { 
    balls_out_of_screen(balls);
    speed_up_balls(balls);
}