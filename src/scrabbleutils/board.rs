use super::spot::Spot;
use super::bonuses::{WordBonus, LetterBonus};
use super::{Move, Direction, Tile};

/// The board we're playing on
///
/// It contains an array of `Spot` and provide shortcuts to interract with them.
/// All positions range start at 0.
/// The ordinate position 0 is considered at the top.
pub struct Board {
    spots: Vec<Spot>,
}

impl Board {
    /// Create a new board
    ///
    /// The board contains the most used position for its bonuses.
    /// It is the one found on [the scrabble wikipedia page](https://en.wikipedia.org/wiki/Scrabble).
    ///
    /// # Return Value
    /// A board of 15*15
    pub fn new() -> Board {
        // Create the default board with no bonuses
        let mut spots = vec![Spot::new(); 225];
        // Add Triple Word
        let wtriple_pos = [ 0, 7, 14, 105, 119, 210, 217, 224 ];
        for i in wtriple_pos.iter() {
            spots.get_mut(*i).unwrap().bonus_word = WordBonus::Triple;
        }

        // Add Double Word
        let wdouble_pos = [ 16, 28, 32, 42, 48, 56, 64, 70, 112, 154, 160,
                            168, 176, 182, 192, 196, 208];
        for i in wdouble_pos.iter() {
            spots.get_mut(*i).unwrap().bonus_word = WordBonus::Double;
        }

        // Add Triple Letter
        let ltriple_pos = [ 20, 24, 76, 80, 84, 88, 136, 140, 144, 148,
                            200, 204];
        for i in ltriple_pos.iter() {
            spots.get_mut(*i).unwrap().bonus_letter = LetterBonus::Triple;
        }

        // Add Double Letter
        let ldouble_pos = [ 3, 11, 36, 38, 45, 52, 59, 92, 96, 98, 102, 108,
                            116, 122, 126, 128, 132, 165, 172, 179, 186, 188,
                            213, 221 ];
        for i in ldouble_pos.iter() {
            spots.get_mut(*i).unwrap().bonus_letter = LetterBonus::Double;
        }

        // Create and return the board
        Board {
            spots,
        }
    }

    /// Get a reference to a spot
    ///
    /// # Arguments
    /// * `x` - the absciss position on the board (from left (0) to right (width - 1)).
    /// * `y` - the ordinate position on the board (from top (0) to bottom (height - 1)).
    ///
    /// # Panic
    /// If `x` or `y` are greater or equal to 15.
    fn get_spot(&self, x : u8, y : u8) -> &Spot{
        assert!(x < 15, "x is out of the board");
        assert!(y < 15, "y is out of the board");
        return self.spots.get((y * 15 + x) as usize).unwrap();
    }

    /// Get a mutable reference to a spot
    ///
    /// # Arguments
    /// * `x` - the absciss position on the board.
    /// * `y` - the ordinate position on the board.
    ///
    /// # Panic
    /// If `x` or `y` are greater or equal to 15.
    fn get_spot_mut(&mut self, x : u8, y : u8) -> &mut Spot{
        assert!(x < 15, "x is out of the board");
        assert!(y < 15, "y is out of the board");
        return self.spots.get_mut((y * 15 + x) as usize).unwrap();
    }

    /// Get a char at a position
    ///
    /// # Arguments
    /// * `x` - the absciss position on the board.
    /// * `y` - the ordinate position on the board.
    ///
    /// # Return Value
    /// An `Option<char>` where None is in the case there is no tile on this spot
    pub fn get_letter(&self, x : u8, y : u8) -> Option<char> {
        if x >= 15 || y >= 15 {
            return None;
        }
        let spot = self.get_spot(x, y);
        let tile = &spot.tile;
        match tile {
            None => None,
            Some(x) => Some(x.letter()),
        }
    }

    /// Get a clone of the optional tile at position (`x`, `y`)
    ///
    /// # Return Value
    /// An `Option<Tile>` where None is in the case there is no tile on
    /// this spot
    pub fn get_tile(&self, x : u8, y : u8) -> Option<Tile> {
        if x >= 15 || y >= 15 {
            return None;
        }
        let spot = self.get_spot(x, y);
        return spot.tile.clone();
    }

    /// Whether it is possible to play this move
    ///
    /// Warning : The move should not be added to the board before calling this
    /// function. If you do so, you'll get an undefined behavior
    ///
    /// # Argument
    /// * `mv` - The move we're trying to add
    pub fn can_place(&self, mv : &Move) -> bool {
        // Make sure mv is ok
        assert!(mv.x() < 15);
        assert!(mv.y() < 15);

        // Mutable positions
        let mut pos_x = mv.x();
        let mut pos_y = mv.y();

        // Offset helps not having the almost same fonction for each directions
        let offset_x : u8;
        let offset_y : u8;

        match mv.direction() {
            Direction::Horizontal => {
                // Test for an out of array move
                if pos_x as usize + mv.word().chars().count() >= 15 {
                    return false;
                }
                offset_x = 1;
                offset_y = 0;
            }
            Direction::Vertical => {
                // Test for an out of array move
                if pos_y as usize + mv.word().chars().count() >= 15 {
                    return false;
                }
                offset_x = 0;
                offset_y = 1;
            }
        }

        // Iterator over the move's word
        let word_it : Vec<char> = mv.word().chars().collect();
        for c in word_it {
            let board_letter = self.get_letter(pos_x, pos_y);
            // If there is a letter on the board at this spot
            if let Some(letter) = board_letter {
                // Make sure it matches with the current word
                if letter != c {
                    return false;
                }
            }
            pos_x += offset_x;
            pos_y += offset_y;
        }
        // No problem so far, we're good
        true
    }

    /// Get the needed letters to make the word
    ///
    /// Returns the letters not present on the board to place `mv`
    ///
    /// # Arguments
    /// * `mv` - The move the player wants to make
    ///
    /// # Return Value
    /// A vector of chars
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
            // If we reach the word end
            if next_char == None {
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

    /// Add a move to the board
    ///
    /// Warning : No check will be made in this function, use with care,
    /// you should consider calling `can_place()` and `needed_letters`
    /// before calling this function.
    ///
    /// # Arguments
    /// * `mv` - The valid move you want to place
    /// * `tiles` - The needed tiles. You can get it by using `needed_letters()`
    /// and `Hand::remove()`
    pub fn add_move(&mut self, mv : Move, tiles : Vec<Tile>) {
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

        let chars = mv.word().chars();
        let mut tiles_it = tiles.into_iter();
        // For each char in the word
        for c in chars {
            // If we need to add a tile to the current spot
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
    }

    /// Get the score of a word made perpendicularly
    ///
    /// It should be called whenever a tile is being set with at least one
    /// perpendicular neighbor around.
    ///
    /// # Arguments
    /// * `initial_x` - Absciss position somewhere in the perpendicular word
    /// * `initial_y` - Ordinate position somewhere in the perpendicular word
    /// * `direction` - The direction the original word was
    /// * `added` - The added tile at this position
    ///
    /// # Return Value
    /// The score the perpendicular word made including the added letter
    fn perp_score(&self, initial_x : u8, initial_y : u8, direction : Direction,
            added : &Tile) -> u32 {
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

    /// Get the score of a move
    ///
    /// This function need sources for the way of calculating the points.
    /// It seems almost standard but it's good to have a rule somewhere.
    ///
    /// # Arguments
    /// * `mv` - The move the player wants to make
    /// * `removed` - The tiles the player removed from its hand to play.
    /// You can get it by using `needed_letters()` and `Hand::remove()`
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

    /// Get the bonuses for a given tile
    ///
    /// # Argument
    /// * The positions
    ///
    /// # Return Value
    /// A tuple made by:
    /// * an enum LetterBonus
    /// * an enum WordBonus
    pub fn get_bonuses(&self, x : u8, y : u8) -> (LetterBonus, WordBonus) {
        assert!(x < 15);
        assert!(y < 15);

        return self.get_spot(x, y).get_bonuses();
    }
}
