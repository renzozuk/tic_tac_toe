use std::io::{stdin, BufRead};

use services::play;
use services::print_table;
use services::player_won;
use services::is_there_a_tie;

pub mod services;

fn main() {
    let mut table: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
    let mut is_first_player = false;
    
    while !player_won(&table) && !is_there_a_tie(&table) {
        print_table(table);

        is_first_player = !is_first_player;

        let mut buffer = String::new();

        loop {
            if is_first_player {
                println!("1st player: ");
            }else{
                println!("2nd player: ");
            }

            stdin().lock()
                .read_line(&mut buffer)
                .expect("Failed to read a number.");

            buffer = buffer.trim().to_string();

            if let Ok(_) = buffer.parse::<usize>() {
                break;
            } else {
                buffer.clear();
            }
        }

        let player_input = match buffer.trim().parse() {
            Ok(num) => num,
            Err(error) => panic!("Problem converting String to usize: {:?}", error)
        };

        if !play(is_first_player, player_input, &mut table) {
            is_first_player = !is_first_player;
        }
    }

    if player_won(&table) {
        if is_first_player {
            println!("The first player won the game!!!");
        }else{
            println!("The second player won the game!!!");
        }
    }

    if is_there_a_tie(&table) {
        println!("That's a tie!!!");
    }

    print_table(table);
}