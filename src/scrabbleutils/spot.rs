use super::bonuses::{LetterBonus, WordBonus};
use super::Tile;

/// Describe a place on the `Board`
///
/// Its structure is public. That means you don't want players to get
/// access to it.
#[derive(Debug, Clone)]
pub struct Spot {
    /// It might have a tile on it
    pub tile : Option<Tile>,
    /// Some bonus applicable to a letter
    pub bonus_letter : LetterBonus,
    /// Some bonus applicable to a word
    pub bonus_word : WordBonus,
}

impl Spot {
    /// Create a new empty spot
    pub fn new() -> Spot {
        Spot {
            tile: None,
            bonus_letter : LetterBonus::None,
            bonus_word : WordBonus::None,
        }
    }

    /// Get integer factors for bonuses
    ///
    /// # Return Value
    /// A tuple with the (letter, word) bonus factor.
    pub fn get_bonuses_value(&self) -> (u32, u32) {
        let letter = match self.bonus_letter {
            LetterBonus::None => 1,
            LetterBonus::Double => 2,
            LetterBonus::Triple => 3,
        };
        let word = match self.bonus_word {
            WordBonus::None => 1,
            WordBonus::Double => 2,
            WordBonus::Triple => 3,
        };

        return (letter, word);
    }

    /// Get the bonus in a tuple
    ///
    /// # Return value
    /// A tuple with the (letter, word) bonus.
    pub fn get_bonuses(&self) -> (LetterBonus, WordBonus) {
        return (self.bonus_letter, self.bonus_word);
    }
}
