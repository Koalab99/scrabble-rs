use std::fs::File;
use std::io::{prelude::*, BufReader};
use super::Tile;

/// This is also known as the joker
pub const WILDCARD : char = '*';

/// Simple struct that stores information about how many times we add this tile
/// in the tileset
pub struct TileInfo {
    tile : Tile,
    occurences : u32,
}

/// Stores all the TileInfo
pub struct TileSet {
    infos : Vec<TileInfo>,
}

impl TileInfo {
    /// Create a TileInfo
    ///
    /// # Arguments
    /// * `c` - The character on the tile.
    /// * `occurences` - How many of this letter must be present in the bag.
    /// * `score` - The score it gives.
    pub fn new(c : char, occurences : u32, score : u8) -> TileInfo {
        // If this is a wildcard, set the flag
        let wildcard = match c {
            WILDCARD => true,
            _ => false,
        };
        let tile = Tile::new(c, score, wildcard);
        TileInfo {
            tile,
            occurences,
        }
    }

    /// Get the letter of this TileInfo's Tile
    pub fn c(&self) -> char {
        return self.tile.letter();
    }

    /// Get the occurences
    pub fn occurences(&self) -> u32 {
        return self.occurences;
    }

    /// Get the score of the Tile
    pub fn score(&self) -> u8 {
        return self.tile.points();
    }

    /// Get a copy of the tile
    pub fn tile(&self) -> Tile {
        self.tile.clone()
    }
}

impl TileSet {
    /// Create a TileSet from a vector of `TileInfo`
    ///
    /// Maybe someone need it, otherwise there is `from_file()`
    pub fn from_vec(vec : Vec<TileInfo>) -> TileSet {
        TileSet {
            infos : vec,
        }
    }

    /// Create a TileSet from a file
    ///
    /// The file need to have a special format to be understood.
    /// It is read line by line, and each one describe a TileInfo.
    /// A TileInfo is described like so :
    /// <letter> <occurences> <score>
    ///
    /// Have a look at the TileInfo constructor for more informations about these parameters.
    pub fn from_file(filename : &str) -> TileSet {
        let mut ts_vec : Vec<TileInfo> = Vec::new();
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let data : Vec<&str> = line.split_whitespace().collect();
            assert!(data.len() == 3, "not 3 elements on one line in tileset file");
            let c : char = data[0].parse().expect("The first word of a line should be a char");
            let occurences : u32 = data[1].parse().expect("The second word should be a number");
            let score : u8 = data[2].parse().expect("The third word should be a number");
            let ti = TileInfo::new(c, occurences, score);
            ts_vec.push(ti);
        }
        TileSet {
            infos : ts_vec
        }
    }

    /// Get the score of a letter
    ///
    /// It will look for the first tile with this score and get its score.
    /// TODO : It's complexity is O(n)
    pub fn get_points(&self, letter : char) -> u8 {
        let mut it = self.infos.iter();
        let pos = it.position(|e| e.tile.letter() == letter);
        match pos {
            Some(p) => {

                return self.infos.get(p).unwrap().tile.points();
            }
            None => {
                // If we don't know this letter we return 0
                return 0;
            }
        }
    }


    pub fn infos(&self) -> &Vec<TileInfo> {
        return &self.infos;
    }
}
