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
            if let Some(letter) = board_letter {
                if letter != c {
                    return false;
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
        for c in chars {
            if self.get_letter(pos_x, pos_y) == None {
                let mut tile : Tile = tiles_it.next().unwrap();
                if tile.wildcard() {
                    // Define the new use value for the wildcard
                    tile.set_wildcard(c);
                }
                self.get_spot_mut(pos_x, pos_y).tile = Some(tile);
            }
            pos_x += x_offset;
            pos_y += y_offset;
        }
        score
    }

    // Return the score made by perpendicular words
    fn perp_score(&self, initial_x : u8, initial_y : u8, direction : Direction, added : &Tile) -> u32 {
        assert!(initial_x < 15);
        assert!(initial_y < 15);
        let mut score = 0;

        let mut pos_x = initial_x as i16;
        let mut pos_y = initial_y as i16;

        let offset_x : i16;
        let offset_y : i16;

        match direction {
            Direction::Horizontal => {
                offset_x = 1;
                offset_y = 0;
            }
            Direction::Vertical => {
                offset_x = 0;
                offset_y = 1;
            }
        }

        // Initial step, count the number of point of the added letter
        score += added.points() as u32;

        // Then count the previous perpendicular letters
        pos_x -= offset_y;
        pos_y -= offset_x;

        while pos_x >= 0 && pos_y >= 0 && pos_y < 15 &&
                self.get_letter(pos_x as u8, pos_y as u8) != None {
            score += self.get_tile(pos_x as u8, pos_y as u8)
                .unwrap().points() as u32;
            pos_x -= offset_y;
            pos_y -= offset_x;
        }

        // Then get the nexts perpendicular letters
        pos_x = initial_x as i16 + offset_y;
        pos_y = initial_y as i16 + offset_x;
        while pos_x < 15 && pos_y < 15 &&
                self.get_letter(pos_x as u8, pos_y as u8) != None {
            score += self.get_tile(pos_x as u8, pos_y as u8).unwrap().points() as u32;
            pos_x += offset_y;
            pos_y += offset_x;
        }

        score
    }

    pub fn score(&self, mv : &Move, removed : &Vec<Tile>) -> u32 {
        let mut score = 0;
        let mut word_bonus = 1;
        let mut tile_score : u32;

        let mut pos_x = mv.x();
        let mut pos_y = mv.y();
        let offset_x : u8;
        let offset_y : u8;

        match mv.direction() {
            Direction::Horizontal => {
                offset_x = 1;
                offset_y = 0;
            }
            Direction::Vertical => {
                offset_x = 0;
                offset_y = 1;
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
                    let (lb, wb) = current_spot.get_bonuses_value();
                    word_bonus *= wb;
                    if self.get_letter(pos_x + offset_y, pos_y + offset_x) == None ||
                       self.get_letter(pos_x - offset_y, pos_y - offset_x) == None {
                        tile_score = self.perp_score(pos_x, pos_y, mv.direction(), removed_tile);
                    }
                    else {
                        tile_score = lb * removed_tile.points() as u32;
                    }

                }
                Some(tile) => {
                    tile_score = tile.points() as u32;
                }
            }
            score += tile_score;
            pos_x += offset_x;
            pos_y += offset_y;
        }
        score * word_bonus
    }

    pub fn get_bonuses(&self, x : u8, y : u8) -> (LetterBonus, WordBonus) {
        assert!(x < 15);
        assert!(y < 15);

        return self.get_spot(x, y).get_bonuses();
    }
}
