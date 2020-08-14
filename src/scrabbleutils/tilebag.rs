use rand::Rng;
use super::{Tile, TileSet};


/// An interface to a vector of `Tile`
///
/// It's content is randomised
pub struct TileBag {
    tiles : Vec<Tile>,
}

impl TileBag {
    /// Create a new TileBag
    ///
    /// Its content is randomised here.
    ///
    /// # Argument
    /// `ts` - The `TileSet` to use
    pub fn new(ts : &TileSet) -> TileBag {
        let mut rng = rand::thread_rng();
        let mut tiles : Vec<Tile> = Vec::with_capacity(103);
        // Avoid calling gen_rand(0, 0)
        let dummy_tile = Tile::new('_', 0, true);
        tiles.push(dummy_tile.clone());

        for line in ts.infos() {
            let count = line.occurences();
            for _ in 0..count {
                tiles.insert(rng.gen_range(0, tiles.len()),
                    line.tile());
            }
        }
        tiles.swap_remove(tiles.iter().position(|e| e == &dummy_tile).unwrap());
        TileBag { tiles }
    }

    /// Whether the bag is empty
    pub fn is_empty(&self) -> bool {
        return self.tiles.len() == 0;
    }

    /// Take a `Tile` from the bag
    /// Returns None if the bag is empty
    pub fn pick(&mut self) -> Option<Tile> {
        return self.tiles.pop();
    }
}
