pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {

    let x_win = diagonals('X', table) || horizontal('X', table) || vertical('X', table);

    let o_win = diagonals('O', table) || horizontal('O', table) || vertical('O', table);

    if x_win && !o_win {

        "player X won".to_string()

    } else if o_win && !x_win {

        "player O won".to_string()

    } else {

        "tie".to_string()

    }

}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {

    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    }

    if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        return true;
    }

    return false;
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {

    let mut i = 0;

    while i < 3 {

        if table[i][0] == player && table[i][1] == player && table[i][2] == player {
            return true;
        }

        i += 1;

    }

    return false;
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {

    let mut j = 0;

    while j < 3 {

        if table[0][j] == player && table[1][j] == player && table[2][j] == player {
            return true;
        }

        j += 1;

    }
    return false;
}

