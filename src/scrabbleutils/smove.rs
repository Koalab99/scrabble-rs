#[derive(Copy, Clone)]
pub enum Direction {
    Horizontal,
    Vertical,
}

pub struct Move {
    x : u8,
    y : u8,
    word : String,
    direction : Direction,
}

impl Move {
    pub fn new(x : u8, y : u8, word : String, direction : Direction) -> Move {
        Move {
            x, y, word, direction
        }
    }

    pub fn word(&self) -> &str {
        return &self.word;
    }
    pub fn direction(&self) -> Direction {
        return self.direction;
    }

    pub fn x(&self) -> u8 {
        return self.x;
    }
    pub fn y(&self) -> u8 {
        return self.y;
    }
}
