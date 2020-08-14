/// Describe a game piece
#[derive(Clone, PartialEq, Debug)]
pub struct Tile {
    letter : char,
    wildcard : bool,
    points : u8,
}

impl Tile {
    /// Create a tile
    ///
    /// # Arguments
    /// * `letter` - The letter on the tile
    /// * `points` - The amount of points this letter gives
    /// * `wildcard` - Whether this tile is a joker
    pub fn new(letter : char, points : u8, wildcard : bool) -> Tile {
        Tile { letter, wildcard, points}
    }
    /// Get the letter
    pub fn letter(&self) -> char {
        self.letter
    }
    /// Get the wildcard field
    pub fn wildcard(&self) -> bool {
        self.wildcard
    }
    /// Get the amount of point it gives
    pub fn points(&self) -> u8 {
        self.points
    }

    /// Replace the char
    ///
    /// Works only when the tile has the wildcard flag
    ///
    /// # Argument
    /// * `c` - The letter to set
    pub fn set_wildcard(&mut self, c : char) {
        if self.wildcard == true {
            self.letter = c;
        }
    }
}

