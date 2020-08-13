use crate::scrabbleutils::{Board, PlayerTrait, Move, Direction::*, Hand};
use crate::scrabbleutils::bonuses::{WordBonus, LetterBonus};
use std::io::stdin;

// I don't like designing ui, please feel free to improve it.

pub fn print_board(board : &Board) {
    print!("     ");
    for _ in 0..15 {
        print!("-----");
    }
    print!("-\n     |");
    for i in 1..16 {
        print!(" {:>2} |", i);
    }
    print!("\n     |");
    for _ in 1..16 {
        print!("    |");
    }
    println!("");
    for _ in 0..16 {
        print!("-----");
    }
    println!("-");

    // For each board row
    for y in 0..15 {
        print!("|");
        for x in 0..16 {
            if x == 0 {
                // Print the row number
                print!(" {:>2} |", y + 1);
            }
            else {
                let letter = board.get_letter(x - 1, y);
                match letter {
                    Some(x) => {
                        print!(" {}  |", x);
                    }
                    None => {
                        // Check letter and word bonus
                        let (lb, wb) = board.get_bonuses(x - 1, y);
                        match wb {
                            WordBonus::Triple => {
                                print!(" W3 |");
                                continue;
                            }
                            WordBonus::Double => {
                                print!(" W2 |");
                                continue;
                            }
                            _ => {}
                        }
                        match lb {
                            LetterBonus::Triple => {
                                print!(" L3 |");
                            }
                            LetterBonus::Double => {
                                print!(" L2 |");
                            }
                            _ => {
                                print!("    |");
                            }
                        }
                    }
                }
            }
        }
        print!("\n|");
        for x in 0..16 {
            if x == 0{
                print!("    |");
            }
            else {
                let tile = board.get_tile(x - 1, y);
                if tile == None {
                    print!("    |");
                }
                else {
                    print!("  {:>2}|", tile.unwrap().points());
                }
            }
        }
        println!("");
        for _ in 0..16 {
            print!("-----");
        }
        println!("-");
    }
    println!("");
}

pub fn print_hand(hand : &Hand) {
    // Top line
    for _ in 0..hand.get().len() {
        print!("-----");
    }
    // End of first line and start of second
    print!("-\n|");
    // Second line (letters + separators)
    for i in hand.get() {
        print!(" {}  |", i.letter());
    }
    // End of second line + start separator of third line
    print!("\n|");
    // Third line
    for i in hand.get() {
        print!("  {:>2}|", i.points());
    }
    println!("");
    // Bottom line
    for _ in 0..hand.get().len() {
        print!("-----");
    }
    print!("-\n");
}

pub struct SimplePlayer {
    name : String,
}

impl SimplePlayer {
    pub fn new(name : String) -> SimplePlayer {
        SimplePlayer {
            name,
        }
    }
}

impl PlayerTrait for SimplePlayer {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn play(&self, board : &Board, hand : &Hand) -> Move {
        let mv : Move;
        let mut error_msg : Option<&str> = None;
        loop {
            print_board(board);
            print_hand(hand);
            if let Some(msg) = error_msg {
                eprintln!("{}", msg);
            }
            let word;
            println!("What do you want to play ?");
            let mut line = String::new();
            stdin().read_line(&mut line).expect("This is not a correct string");
            let words : Vec<&str> = line.split_whitespace().collect();
            if words.len() != 1 {
                error_msg = Some("You should give exactly one word");
                continue;
            }
            else {
                word = words[0].to_owned();
            }
            println!("At what position do you want to play it ?\n\tex : 1 15");
            let mut line = String::new();
            stdin().read_line(&mut line).expect("This is not a correct string");
            let positions : Vec<&str> = line.split_whitespace().collect();
            if positions.len() != 2 {
                error_msg = Some("You should give two positions");
                continue;
            }
            let positions : Vec<Result<u8, std::num::ParseIntError>> = positions.iter().map(|e| e.parse::<u8>()).collect();
            if positions.iter().any(|e| e.is_err()) {
                error_msg = Some("Positions should be numbers, the first is on the x coordinate from 1 to 15, the second in the descending y coordinate from 1 to 15.");
                continue;
            }
            let positions : Vec<u8> = positions.into_iter().map(|e| e.unwrap().clone().to_owned()).collect();
            if positions.iter().any(|e| e < &1 || e > &15) {
                error_msg = Some("Positions should be between 1 and 15");
                continue;
            }
            println!("Choose your direction (H/V) :");
            let mut line = String::new();
            stdin().read_line(&mut line).expect("This is not a correct string");
            let line : Vec<&str> = line.split_whitespace().collect();
            if line.len() == 0 {
                error_msg = Some("Please provide at least the first character of a direction");
                continue;
            }
            let direction_char = line.get(0).unwrap().chars().next().unwrap();
            let direction = match direction_char {
                'H' | 'h' => Horizontal,
                'V' | 'v' => Vertical,
                _ => {
                    error_msg = Some("Could not get the direction properly, write `V` for vertical and `H` for horizontal");
                    continue;
                }
            };
            mv = Move::new(positions[0] - 1, positions[1] - 1, word, direction);
            break;
        }
        return mv;
    }

    fn move_score(&self, score : u32) {
        println!("Your move made {} points!", score);
    }

    fn total_score(&self, score : u32) {
        println!("You have a total of {} points!", score);
    }
}

pub fn handle_error(error : &str) {
    eprintln!("Error : {}", error);
}
