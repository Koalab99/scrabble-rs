use std::fs::File;
use std::io::{prelude::*, BufReader};
use super::Tile;

pub const WILDCARD : char = '*';

pub struct TileInfo {
    tile : Tile,
    occurences : u32,
}

pub struct TileSet {
    infos : Vec<TileInfo>,
}

impl TileInfo {
    pub fn new(c : char, occurences : u32, score : u8) -> TileInfo {
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

    pub fn c(&self) -> char {
        return self.tile.letter();
    }

    pub fn occurences(&self) -> u32 {
        return self.occurences;
    }

    pub fn score(&self) -> u8 {
        return self.tile.points();
    }
    pub fn tile(&self) -> Tile {
        self.tile.clone()
    }
}

impl TileSet {
    pub fn from_vec(vec : Vec<TileInfo>) -> TileSet {
        TileSet {
            infos : vec,
        }
    }

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

    pub fn get_points(&self, letter : char) -> u8 {
        let mut it = self.infos.iter();
        let pos = it.position(|e| e.tile.letter() == letter).unwrap();
        self.infos.get(pos).unwrap().tile.points()
    }

    pub fn infos(&self) -> &Vec<TileInfo> {
        return &self.infos;
    }
}
