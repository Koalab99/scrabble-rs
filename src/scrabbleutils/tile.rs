#[derive(Clone, PartialEq, Debug)]
pub struct Tile {
    letter : char,
    wildcard : bool,
    points : u8,
}

impl Tile {
    pub fn new(letter : char, points : u8, wildcard : bool) -> Tile {
        Tile { letter, wildcard, points}
    }
    pub fn letter(&self) -> char {
        self.letter
    }
    pub fn wildcard(&self) -> bool {
        self.wildcard
    }
    pub fn points(&self) -> u8 {
        self.points
    }

    pub fn set_wildcard(&mut self, c : char) {
        if self.wildcard == true {
            self.letter = c;
        }
    }
}

