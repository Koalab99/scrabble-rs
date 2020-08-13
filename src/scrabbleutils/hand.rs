const HAND_CAPACITY : usize = 7;
use super::{Tile, TileBag};

#[derive(Debug)]
pub struct Hand {
    tiles : Vec<Tile>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            tiles : Vec::with_capacity(HAND_CAPACITY),
        }
    }

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

    // remove one or more tiles on the board
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

    pub fn get(&self) -> Vec<Tile> {
        self.tiles.clone()
    }

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
