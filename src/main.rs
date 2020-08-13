mod scrabbleutils;
mod scrab_tui;

use scrabbleutils::{Board, Dico, TileBag, Player, TileSet, Move, Tile};

fn player_turn(player_data : &mut Player, board : &Board, dico : &Dico) -> (Move, Vec<Tile>) {
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
        return (mv, removed);
    }

}

fn main() {
    let tileset = TileSet::from_file("english_tileset.txt");
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
            let (mv, removed) = player_turn(player_data, &board, &dico);
            let score = board.score(&mv, &removed);
            board.add_move(mv, removed);
            player_data.score += score;
            player_data.player.move_score(score);
            player_data.player.total_score(player_data.score);
        }
        if bag.is_empty() {
            break;
        }
        turn += 1;
    }

    println!("party ended in {} turns", turn);
}
