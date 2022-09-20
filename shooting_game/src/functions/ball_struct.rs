pub struct BallStruct{
    pub speed: (i32, i32),
    pub position: (i32, i32),
    pub radius: i32
}

pub fn Ball(speed: (i32, i32), position: (i32, i32), radius: i32) -> BallStruct{
    BallStruct{
        speed: speed,        
        position: position,
        radius: radius,
    }
}