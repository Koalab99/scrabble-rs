/// Everything related to the player
///
/// Contains its interface's trait and a struct to store player's data in game
mod player;
/// Code of the board
///
/// It contains some important logic about the game
mod board;
/// All about the bonuses of the spot
pub mod bonuses;
/// The elements of the board
mod spot;
/// Where the dictionnary is stored
mod dico;
mod gaddag;
/// The bag we draw tiles from
mod tilebag;
/// The way we describe moves
mod smove;
/// The player's Hand, stores tiles and provide a few function.
mod hand;
/// All about the configurable stuff
///
/// It is currently not really configurable but its a starting points
mod config;
/// The small tiles we place in the game
mod tile;

/// Interface to make a player
pub use player::PlayerTrait;
pub use board::Board;
pub use dico::Dico;
pub use tilebag::TileBag;
/// The way we tell what we want to play
pub use smove::Move;
pub use tile::Tile;
pub use smove::Direction;
/// Part of Move
pub use hand::Hand;
/// All the player implementation shell
pub use player::Player;
/// Part of the configuration
///
/// Define all the tiles we want in our bag
pub use config::TileSet;
