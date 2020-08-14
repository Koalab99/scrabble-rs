use super::{Board, Move, Hand};

/// Gives a simple player interface to work with
pub trait PlayerTrait {
    /// Must return the player's name
    fn name(&self) -> &str;
    /// Called when it's the player turn.
    ///
    /// # Arguments
    /// * `board` - The game board in its current state.
    /// * `hand` - The struct that stores your tiles.
    ///
    /// # Return Value
    /// A move structure.
    /// it before adding it to the game.
    fn play(&self, board : &Board, hand : &Hand) -> Move;
    /// Gives you the score for your move.
    ///
    /// # Arguments
    /// * `score` - The score your move made.
    fn move_score(&self, score : u32);
    /// Gives you the sum of all your moves' score.
    ///
    /// # Arguments
    /// * `score` - Your score.
    fn total_score(&self, score : u32);
}

/// The Player Struct
///
/// Used by the main function or something.
pub struct Player {
    /// A dynamic pointer to the player implementation
    pub player: Box<dyn PlayerTrait>,
    /// The player's hand
    pub hand: Hand,
    /// The player's score
    pub score: u32,
}

impl Player {
    /// Create a new Player structure
    ///
    /// # Argument
    /// * `player` - A pointer to an implementation of the PlayerTrait.
    ///
    /// # Return Value
    /// A Player structure with an empty hand and a score of 0.
    pub fn new(player: Box<dyn PlayerTrait>) -> Player {
        Player {
            player,
            hand : Hand::new(),
            score : 0,
        }
    }
}
