use super::spot::Spot;
use super::bonuses::{WordBonus, LetterBonus};
use super::{Move, Direction, Tile};

pub struct Board {
    spots: Vec<Spot>,
}

impl Board {
    pub fn new() -> Board {
        let mut spots = vec![Spot::new(); 225];
        // Triple Word
        let wtriple_pos = [ 0, 7, 14, 105, 119, 210, 217, 224 ];
        for i in wtriple_pos.iter() {
            spots.get_mut(*i).unwrap().bonus_word = WordBonus::Triple;
        }

        // Double Word
        let wdouble_pos = [ 16, 28, 32, 42, 48, 56, 64, 70, 112, 154, 160,
                            168, 176, 182, 192, 196, 208];
        for i in wdouble_pos.iter() {
            spots.get_mut(*i).unwrap().bonus_word = WordBonus::Double;
        }

        // Triple Letter
        let ltriple_pos = [ 20, 24, 76, 80, 84, 88, 136, 140, 144, 148,
                            200, 204];
        for i in ltriple_pos.iter() {
            spots.get_mut(*i).unwrap().bonus_letter = LetterBonus::Triple;
        }

        // Double Letter
        let ldouble_pos = [ 3, 11, 36, 38, 45, 52, 59, 92, 96, 98, 102, 108,
                            116, 122, 126, 128, 132, 165, 172, 179, 186, 188,
                            213, 221 ];
        for i in ldouble_pos.iter() {
            spots.get_mut(*i).unwrap().bonus_letter = LetterBonus::Double;
        }

        Board {
            spots,
        }
    }

    fn get_spot(&self, x : u8, y : u8) -> &Spot{
        assert!(x < 15, "x is out of the board");
        assert!(y < 15, "y is out of the board");
        return self.spots.get((y * 15 + x) as usize).unwrap();
    }

    fn get_spot_mut(&mut self, x : u8, y : u8) -> &mut Spot{
        assert!(x < 15, "x is out of the board");
        assert!(y < 15, "y is out of the board");
        return self.spots.get_mut((y * 15 + x) as usize).unwrap();
    }

    pub fn get_letter(&self, x : u8, y : u8) -> Option<char> {
        if x >= 15 || y > 15 {
            return None;
        }
        let spot = self.get_spot(x, y);
        let tile = &spot.tile;
        match tile {
            None => None,
            Some(x) => Some(x.letter()),
        }
    }

    pub fn get_tile(&self, x : u8, y : u8) -> Option<Tile> {
        if x >= 15 || y > 15 {
            return None;
        }
        let spot = self.get_spot(x, y);
        return spot.tile.clone();
    }

    pub fn can_place(&self, mv : &Move) -> bool {
        let mut pos_x = mv.x();
        let mut pos_y = mv.y();

        let offset_x : u8;
        let offset_y : u8;

        match mv.direction() {
            Direction::Horizontal => {
                if pos_x as usize + mv.word().chars().count() >= 15 {
                    return false;
                }
                offset_x = 1;
                offset_y = 0;
            }
            Direction::Vertical => {
                if pos_y as usize + mv.word().chars().count() >= 15 {
                    return false;
                }
                offset_x = 0;
                offset_y = 1;
            }
        }

        let word_it : Vec<char> = mv.word().chars().collect();
        for c in word_it {
            let board_letter = self.get_letter(pos_x, pos_y);
            match board_letter {
                None => { continue; },
                Some(letter) => {
                    if letter != c {
                        return false;
                    }
                }
            }
            pos_x += offset_x;
            pos_y += offset_y;
        }

        true
    }

    // Return a vector with the letters needed to make the move
    pub fn needed_letters(&self, mv : &Move) -> Vec<char> {
        // The returned value
        let mut letters : Vec<char> = Vec::with_capacity(
                mv.word().chars().count() - 1);
        // The offset we'll add in every loop
        let offset_x : u8;
        let offset_y : u8;
        // The counter to the actual position on the board
        let mut pos_x = mv.x();
        let mut pos_y = mv.y();
        // char iterator and storage for next() return value
        let mut word_it = mv.word().chars();
        let mut next_char : Option<char>;

        match mv.direction() {
            Direction::Horizontal => {
                offset_x = 1;
                offset_y = 0;
            },
            Direction::Vertical => {
                offset_x = 0;
                offset_y = 1;
            }
        }
        loop {
            next_char = word_it.next();
            if next_char == None {
                // Word end
                return letters;
            }
            let next_char = next_char.unwrap();
            if let None = self.get_letter(pos_x, pos_y) {
                // The current spot is free
                letters.push(next_char);
            }
            pos_x += offset_x;
            pos_y += offset_y;
        }
    }

    pub fn add_move(&mut self, mv : Move, tiles : Vec<Tile>) -> u32 {
        let mut pos_x = mv.x();
        let mut pos_y = mv.y();
        let x_offset : u8;
        let y_offset : u8;
        let mut score = 0;

        match mv.direction() {
            Direction::Horizontal => {
                x_offset = 1;
                y_offset = 0;
            }
            Direction::Vertical => {
                x_offset = 0;
                y_offset = 1;
            }
        }

        let chars = mv.word().chars();
        let mut tiles_it = tiles.into_iter();
        for _ in chars {
            if self.get_letter(pos_x, pos_y) == None {
                self.get_spot_mut(pos_x, pos_y).tile = tiles_it.next();
            }
            pos_x += x_offset;
            pos_y += y_offset;
        }
        score
    }

    pub fn score(&self, mv : &Move, removed : &Vec<Tile>) -> u32 {
        let mut score = 0;
        let mut word_bonus = 1;
        let mut tile_score : u32;

        let mut pos_x = mv.x();
        let mut pos_y = mv.y();
        let x_offset : u8;
        let y_offset : u8;

        match mv.direction() {
            Direction::Horizontal => {
                x_offset = 1;
                y_offset = 0;
            }
            Direction::Vertical => {
                x_offset = 0;
                y_offset = 1;
            }
        }

        let mut remove_it = removed.iter();
        let chars = mv.word().chars();
        for _ in chars {
            let current_spot = self.get_spot(pos_x, pos_y);
            match &current_spot.tile {
                None => {
                    // get the bonuses
                    let removed_tile = remove_it.next().unwrap();
                    if !removed_tile.wildcard() {
                        let (lb, wb) = current_spot.
                            get_bonuses_value();
                        word_bonus *= wb;
                        tile_score = lb * removed_tile.points() as u32;
                    }
                    else {
                        tile_score = 0;
                    }

                }
                Some(tile) => {
                    tile_score = tile.points() as u32;
                }
            }
            score += tile_score;
            pos_x += x_offset;
            pos_y += y_offset;
        }
        score * word_bonus
    }
}
