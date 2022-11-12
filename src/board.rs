pub mod ai;

#[derive(Clone)]
pub struct Player {
    is_ai: bool,
    player: char,
}

#[derive(Clone)]
pub struct game {
    pub board: Vec<Vec<char>>,
    pub size: (u8, u8),
    pub lenght: u8,
    pub turn: char,
    pub depth: i32,
    pub ai: char,
    pub players: Vec<Player>,
    pub peaces_placed: i32,
}

impl game {
    pub fn new() -> game {
        game {
            board: (vec![vec![' ']]),
            size: (0, 0),
            lenght: (0),
            turn: ('X'),
            depth: (0),
            ai: ('O'),
            players: (vec![
                Player {
                    is_ai: false,
                    player: 'X',
                },
                Player {
                    is_ai: false,
                    player: 'X',
                },
            ]),
            peaces_placed: (0),
        }
    }

    pub fn check_winner(mut self) -> char {
        if self.peaces_placed
            < self.lenght as i32 * self.players.len() as i32 - self.players.len() as i32 - 1
        {
            return 'N';
        }

        for _ in 0..2 {
            for y in 0..self.board.len() {
                let mut in_row_y: u8 = 0;
                let mut in_row_x: u8 = 0;
                for x in 0..self.board[y].len() {
                    if self.board[y][x] == self.turn {
                        in_row_x += 1;
                    } else {
                        in_row_x = 0;
                    }
                    if in_row_x == self.lenght {
                        return self.turn;
                    }

                    if self.board[x][y] == self.turn {
                        in_row_y += 1;
                    } else {
                        in_row_y = 0;
                    }
                    if in_row_y == self.lenght {
                        return self.turn;
                    }
                }
            }

            // y is for \ check and y2 is for / check
            let mut y = self.board.len();
            let mut y2 = 0;
            for i in -(self.board.len() as i32)..self.board.len() as i32 {
                let mut in_row = 0;
                let mut in_row2 = 0;
                for x in 0..self.board.len() - i.abs() as usize {
                    // / check
                    if self.board[y + x][x] == self.turn {
                        in_row += 1;
                    } else {
                        in_row = 0;
                    }
                    if in_row == self.lenght {
                        return self.turn;
                    }

                    // \ check
                    if self.board[y2 - x][x] == self.turn {
                        in_row2 += 1;
                    } else {
                        in_row2 = 0;
                    }
                    if in_row2 == self.lenght {
                        return self.turn;
                    }
                }

                if y != 0 {
                    y -= 1;
                }
                if y2 != self.board.len() - 1 {
                    y2 += 1;
                }
            }

            self.turn = self.next_turn();
        }

        if self.is_board_full() == true {
            return 'T';
        }

        return 'N';
    }

    fn is_board_full(&self) -> bool {
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                if self.board[y][x] == ' ' {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn place_piece(&mut self, pos: (u8, u8)) -> &str {
        if self.board[pos.1 as usize][pos.0 as usize] == ' ' {
            self.board[pos.1 as usize][pos.0 as usize] = self.turn;
            self.peaces_placed += 1;
            return "ok";
        }
        return "piece is already there";
    }

    pub fn next_turn(&self) -> char {
        match self.turn {
            'X' => return 'O',
            'O' => return 'X',
            _ => return 'X',
        }
    }

    pub fn print_board(&self) {
        //prints top part
        print!("╔═══");
        for _ in 0..self.board.len() - 1 {
            print!("╦═══");
        }
        println!("╗");

        //prints grid
        for y in 0..self.board.len() {
            //prints lines with numbers
            for x in 0..self.board[y].len() {
                print!("║ {} ", self.board[y][x])
            }
            println!("║");

            // prints middle lines
            if y != self.board.len() - 1 {
                print!("╠═══");
                for _ in 0..self.board[y].len() - 1 {
                    print!("╬═══");
                }
                println!("╣");
            }
        }

        //prints bottom part
        print!("╚═══");
        for _ in 0..self.board.len() - 1 {
            print!("╩═══");
        }
        println!("╝");

        println!("{:?}", self.board)
    }

    pub fn clear_bloard(&mut self) {
        self.board.clear();
        for _ in 0..self.size.1 {
            let mut temp_vec: Vec<char> = Vec::new();
            for _ in 0..self.size.0 {
                temp_vec.push(' ');
            }
            self.board.push(temp_vec);
        }
    }
}
