pub fn play(is_first_player: bool, player_input: usize, table: &mut [[char; 3]; 3]) -> bool {
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

pub fn print_table(table: [[char; 3]; 3]) {
    println!("-------");
    for row in table {
        for cell in row {
            print!("|{cell}");
        }
        print!("|\n-------\n")
    }
}

pub fn player_won(table: &[[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if table[i][0] == table[i][1] && table[i][0] == table[i][2] {
            return true;
        }

        if table[0][i] == table[1][i] && table[0][i] == table[2][i] {
            return true;
        }
    }

    return (table[0][0] == table[1][1] && table[0][0] == table[2][2]) || (table[0][2] == table[1][1] && table[0][2] == table[2][0]);
}

pub fn is_there_a_tie(table: &[[char; 3]; 3]) -> bool {
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