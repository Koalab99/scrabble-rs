const HAND_CAPACITY : usize = 7;
use super::{Tile, TileBag};

/// Stores a vector of tiles
pub struct Hand {
    tiles : Vec<Tile>,
}

impl Hand {
    /// Create a new empty Hand
    pub fn new() -> Hand {
        Hand {
            tiles : Vec::with_capacity(HAND_CAPACITY),
        }
    }

    /// Take tiles from a bag
    ///
    /// Argument:
    /// * `bag` - A mutable reference to the Bag to draw from
    pub fn draw(&mut self, bag : &mut TileBag) {
        while self.tiles.len() < HAND_CAPACITY {
            let new_tile = bag.pick();
            if let None = new_tile {
                return;
            }
            let new_tile = new_tile.unwrap();
            self.tiles.push(new_tile);
        }
    }

    /// Remove one or more tiles and return them
    ///
    /// It takes a vector of character and return the associated tiles
    /// (including wildcard if it exists) or None if there is no letter
    /// matching the one asked.
    ///
    /// # Argument
    /// `remove` - The chars to remove
    pub fn remove(&mut self, remove : &Vec<char>) -> Option<Vec<Tile>> {
        let mut ret : Vec<Tile> = Vec::with_capacity(7);
        if !self.contains(remove) {
            return None;
        }
        for c in remove {
            match self.tiles.iter()
                    .position(|tile| tile.letter() == *c) {
                None => {
                    // We don't have this letter, but we have a wildcard
                    let index = self.tiles.iter()
                        .position(|tile| tile.wildcard() == true).unwrap();
                    ret.push(self.tiles.swap_remove(index));
                },
                Some(index) => {
                    ret.push(self.tiles.swap_remove(index));
                }
            }
        }
        if ret.len() > 0 {
            return Some(ret);
        }
        else {
            return None;
        }
    }

    /// Get a copy of the tiles in hand
    pub fn get(&self) -> Vec<Tile> {
        self.tiles.clone()
    }

    /// Whether some tile are present
    ///
    /// Tells if there is a different tile for each character in `elem`
    /// # Argument
    /// * `elem` - The vector of letter to verify they match a different tile
    pub fn contains(&self, elem : &Vec<char>) -> bool {
        let mut tmp_tiles = self.tiles.clone();
        for c in elem {
            match tmp_tiles.iter().position(|tile| tile.letter() == *c) {
                None => {
                    // We don't have this letter, but we have a wildcard
                    match tmp_tiles.iter().position(|tile| tile.wildcard() == true) {
                        None => {
                            return false;
                        },
                        Some(index) => {
                            tmp_tiles.remove(index);
                        }
                    }
                },
                Some(index) => {
                    tmp_tiles.remove(index);
                }
            }
        }
        true
    }
}
