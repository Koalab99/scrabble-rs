/// Describe the orientation of a `Move`
#[derive(Copy, Clone)]
pub enum Direction {
    Horizontal,
    Vertical,
}

/// Describe a player action in the game
pub struct Move {
    /// Position of the first letter of the word on the absciss
    x : u8,
    /// Position of the first letter of the word on the ordinate
    y : u8,
    /// The word itself
    word : String,
    /// The direction the word is being placed
    direction : Direction,
}

impl Move {
    /// Create a Move
    pub fn new(x : u8, y : u8, word : String, direction : Direction) -> Move {
        Move {
            x, y, word, direction
        }
    }

    /// Get the word
    pub fn word(&self) -> &str {
        return &self.word;
    }

    /// Get the direction
    pub fn direction(&self) -> Direction {
        return self.direction;
    }

    /// Get the x
    pub fn x(&self) -> u8 {
        return self.x;
    }

    /// Get the y
    pub fn y(&self) -> u8 {
        return self.y;
    }
}
