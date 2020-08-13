use super::{Board, Move, Hand};

pub trait PlayerTrait {
    fn name(&self) -> &str;
    fn play(&self, board : &Board, hand : &Hand) -> Move;
    fn move_score(&self, score : u32);
    fn total_score(&self, score : u32);
}

pub struct Player {
    pub player: Box<dyn PlayerTrait>,
    pub hand: Hand,
    pub score: u32,
}

impl Player {
    pub fn new(player: Box<dyn PlayerTrait>) -> Player {
        Player {
            player,
            hand : Hand::new(),
            score : 0,
        }
    }
}
