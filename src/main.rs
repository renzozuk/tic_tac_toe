use std::{
    io::{stdin, BufRead}
};

fn main() {
    let mut table: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
    let mut is_first_player = false;
    
    while !player_won(&table) && !is_there_a_tie(&table) {
        print_table(table);

        is_first_player = !is_first_player;

        let mut buffer = String::new();

        loop {
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

        while !play(is_first_player, player_input, &mut table) {}
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

fn play(is_first_player: bool, player_input: usize, table: &mut [[char; 3]; 3]) -> bool {
    if table[(player_input - 1) / 3][(player_input - 1) % 3].is_digit(10) {
        if is_first_player {
            table[(player_input - 1) / 3][(player_input - 1) % 3] = 'X';
        }else{
            table[(player_input - 1) / 3][(player_input - 1) % 3] = 'O';
        }

        return true;
    }else{
        println!("This place is already played.");

        return false;
    }
}

fn print_table(table: [[char; 3]; 3]) {
    println!("-------");
    for row in table {
        for cell in row {
            print!("|{cell}");
        }
        print!("|\n-------\n")
    }
}

fn player_won(table: &[[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if table[i][0] == table[i][1] && table[i][0] == table[i][2] {
            return true;
        }

        if table[0][i] == table[1][i] && table[0][i] == table[2][i] {
            return true;
        }
    }

    if table[0][0] == table[1][1] && table[0][0] == table[2][2] {
        return true;
    }
    
    if table[0][2] == table[1][1] && table[0][2] == table[2][0] {
        return true;
    }

    return false;
}

fn is_there_a_tie(table: &[[char; 3]; 3]) -> bool {
    return !table[0][0].is_digit(10) 
    && !table[0][1].is_digit(10) 
    && !table[0][2].is_digit(10) 
    && !table[1][0].is_digit(10) 
    && !table[1][1].is_digit(10) 
    && !table[1][2].is_digit(10) 
    && !table[2][0].is_digit(10) 
    && !table[2][1].is_digit(10) 
    && !table[2][2].is_digit(10);
}