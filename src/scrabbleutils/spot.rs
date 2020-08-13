use super::bonuses::{LetterBonus, WordBonus};
use super::Tile;

#[derive(Debug, Clone)]
pub struct Spot {
    pub tile : Option<Tile>,
    pub bonus_letter : LetterBonus,
    pub bonus_word : WordBonus,
}

impl Spot {
    pub fn new() -> Spot {
        Spot {
            tile: None,
            bonus_letter : LetterBonus::None,
            bonus_word : WordBonus::None,
        }
    }

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

    pub fn get_bonuses(&self) -> (LetterBonus, WordBonus) {
        return (self.bonus_letter, self.bonus_word);
    }
}
