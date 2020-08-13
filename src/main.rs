mod scrabbleutils;
mod scrab_tui;

use scrabbleutils::{Board, Dico, TileBag, Player, TileSet};

fn main() {
    let tileset = TileSet::from_file("tileset.txt");
    let dico = Dico::new("dico.txt");
    let mut board = Board::new();
    let mut bag = TileBag::new(&tileset);
    let mut turn = 1;
    let mut players : Vec<Player> = Vec::new();
    players.push(Player::new(Box::new(
                scrab_tui::SimplePlayer::new("Yvan".to_string()))));
    players.push(Player::new(Box::new(
                scrab_tui::SimplePlayer::new("Ugo".to_string()))));

    // Init
    loop {
        for player_data in &mut players {
            player_data.hand.draw(&mut bag);
            //let mut mv : scrabbleutils::Move;
            loop {
                let mv = player_data.player.play(&board, &player_data.hand);
                let is_valid = dico.exists(mv.word());
                if is_valid {
                    scrab_tui::handle_error(
                        format!("{} is not in the dictionnary.", mv.word())
                        .as_str());
                    continue;
                }
                if !board.can_place(&mv) {
                    scrab_tui::handle_error(
                        format!("{} can't be placed", mv.word())
                        .as_str());
                    continue;
                }
                let letters = board.needed_letters(&mv);

                if !player_data.hand.contains(&letters) {
                    scrab_tui::handle_error(
                        format!("{} can't be made with your letters", mv.word())
                        .as_str());
                    continue;
                }
                let removed = player_data.hand.remove(&letters);
                if let None = removed {
                    scrab_tui::handle_error(
                        format!("{} already exists, you didn't change anything", mv.word())
                        .as_str());
                    continue;
                }
                let removed = removed.unwrap();
                let score = board.add_move(mv, removed);
                player_data.score += score;
                break;
            }
        }
        if bag.is_empty() {
            break;
        }
        turn += 1;
    }

    println!("party ended in {} turns", turn);
}
