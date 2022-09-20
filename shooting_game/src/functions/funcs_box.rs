use super::super::constants::consts::*;

pub struct BoxStruct{
    pub speed: (i32, i32),
    pub position: (i32, i32), //left corner of box i think
    pub length: i32,
}

pub fn make_box(speed: (i32, i32), position: (i32, i32), length: i32) -> BoxStruct {
    let mut Box = BoxStruct{
        speed: speed, 
        position: position,
        length: length,
    };
    Box
}

pub fn update_box(Box: &mut BoxStruct){
    //checking collison
    if Box.position.0 <= 0 || Box.position.0 >= SCREEN_WIDTH() - Box.length{
        Box.speed.0 = -1*Box.speed.0;
    }
    if Box.position.1 <= 0 || Box.position.1 >= SCREEN_HEIGHT() - Box.length{
        Box.speed.1 = -1* Box.speed.1;
    }
    //adding speed
    Box.position.0 = Box.position.0 + Box.speed.0;
    Box.position.1 = Box.position.1 + Box.speed.1;
}

//aim: create box that collides with edge