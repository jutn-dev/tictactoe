use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{f32::INFINITY, io::Write, sync::mpsc, thread};

pub fn game_loop() {
    let mut board: Vec<Vec<char>> = Vec::new();
    let size: (u8, u8);
    let mut lenght = 3;
    let mut turn = 'X';
    let mut depth;

    print!("set size of the board: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    size = {
        // let temp: Vec<&str> = input.split(" ").collect();
        (
            input.trim().parse::<u8>().unwrap(),
            input.trim().parse::<u8>().unwrap(),
        )
    };

    print!("set row lenght: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    lenght = input.trim().parse::<u8>().unwrap();

    print!("set AI depth: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    depth = input.trim().parse::<i32>().unwrap();

    // print!("is multi threading?(1,0): ");
    // std::io::stdout().flush().unwrap();
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // lenght = input.trim().parse::<u8>().unwrap();
    // print!("{:?}", sus);
    clear_bloard(&mut board, size);

    print_board(&board);
    // println!("board: {:?}", board);

    loop {
        if turn == 'X' {
            // print!("{} set piece (x y): ", turn);
            // std::io::stdout().flush().unwrap();
            // let mut input = String::new();
            // std::io::stdin().read_line(&mut input).unwrap();
            // let pos = {
            //     let temp: Vec<&str> = input.split(" ").collect();
            //     (
            //         temp[0].trim().parse::<u8>().unwrap(),
            //         temp[1].trim().parse::<u8>().unwrap(),
            //     )
            // };
            // if place_piece(&mut board, turn, pos) != "piece is already there" {
            //     turn = next_turn(turn);
            // } else {
            //     println!("invalid piece place");
            // }

            let time = std::time::Instant::now();
            AI_multi_thread(&mut board,'X', lenght, depth);
            turn = next_turn(turn);
            println!("AI time: {}", time.elapsed().as_secs_f64());

        } else {
            let time = std::time::Instant::now();
            // AI(&mut board, 'O', &lenght, depth);
            AI_multi_thread(&mut board,'O', lenght, depth);
            //0.23
            println!("AI time: {}", time.elapsed().as_secs_f64());
            turn = next_turn(turn);
        }
        print_board(&board);
        // println!("board: {:?}", board);
        let winner = check_winner(&mut board, &lenght);
        if winner != 'N' {
            println!("{}, won!", winner);
            break;
        }
        thread::sleep(Duration::from_secs_f64(1.5));
    }
}

fn AI(board: &mut Vec<Vec<char>>, ai: char, lenght: &u8, depth: i32) {



    let mut next_move: (usize, usize) = (0, 0);

    let mut max_eval = -INFINITY;
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == ' ' {
                board[y][x] = ai;
                let eval = minmax(
                    board.clone(),
                    ai,
                    next_turn(ai),
                    lenght.clone(),
                    depth - 1,
                    -99999.9,
                    99999.9,
                    false,
                );
                board[y][x] = ' ';
                if eval > max_eval {
                    max_eval = eval;
                    next_move = (y as usize, x as usize);
                }
            }
        }
    }

    board[next_move.0][next_move.1] = ai;
}

fn AI_multi_thread(board: &mut Vec<Vec<char>>, ai: char, lenght: u8, depth: i32) {
    // let mut next_move: (usize, usize) = (0, 0);
    // let mut moves: Vec<(usize, usize,f32)> = Vec::new();
    // let mut tx_ = Vec::new();
    // let mut rx_ = Vec::new();
    // for _ in 0..board.len() * board[0].len() {
    //     let (tx, rx) =  mpsc::channel();
    //     let mut val:(usize,usize,f32) = (0,0,0.0);
    //     tx.send(val).unwrap();
    //     tx_.push(tx);
    //     rx_.push(rx);
    // }


    let threads = Arc::new(AtomicUsize::new(0));
    let mut best_result: (usize, usize, f32) = (0, 0, -99999.9);
    let (tx, rx) = mpsc::channel();
    let mut i = 0;
    for y in 0..board.len() {
        for x in 0..board.len() {
            if board[y][x] == ' ' {
                let threads_ = Arc::clone(&threads);
                // let threads = Arc::clone(&threads);
                let tx_ = tx.clone();
                let mut board_t = board.clone();
                thread::spawn(move || {
                    threads_.fetch_add(1, Ordering::SeqCst);
                    // println!("working...");
                    let mut val: (usize, usize, f32) = (y, x, 0.0);
                    val.0 = y;
                    val.1 = x;
                    board_t[y][x] = ai;
                    val.2 = minmax(
                        board_t.clone(),
                        ai,
                        next_turn(ai),
                        lenght.clone(),
                        depth.clone() - 1,
                        -99999.9,
                        99999.9,
                        false,
                    );
                    board_t[y][x] = ' ';
                    tx_.send(val).unwrap();
                    threads_.fetch_sub(1, Ordering::SeqCst);
                    // println!("done");
                });
                thread::sleep(Duration::from_millis(1));
                // println!("{}", threads.load(Ordering::SeqCst));
                while threads.load(Ordering::SeqCst)
                    >= thread::available_parallelism().unwrap().get()
                {
                    // println!("{}", threads.load(Ordering::SeqCst));
                    // println!("wait");
                    thread::sleep(Duration::from_millis(1));
                }

                i += 1;

                // moves.push(rx_[i].recv().unwrap());
            }
        }
    }

    // let m = rx.recv().unwrap();
    // let pasi = rx.iter().count();

    for _ in 0..i {
        let b = rx.recv().unwrap();
        if b.2 > best_result.2 {
            best_result = b;
        }
    }

    // for b in rx.iter() {
    //     println!("{}",b.2);
    //     if b.2 > best_result.2 {
    //         best_result = b;
    //     }
    // }
    println!("wow");
    board[best_result.0][best_result.1] = ai;
}

fn minmax(
    mut board: Vec<Vec<char>>,
    ai: char,
    player: char,
    lenght: u8,
    depth: i32,
    mut alpha: f32,
    mut beta: f32,
    is_maximizing: bool,
) -> f32 {
    let winner = check_winner(&board, &lenght);
    if winner != 'N' {
        if winner == ai {
            return 10.0;
        }
        if winner == 'T' {
            return 0.0;
        }
        if winner != ai {
            return -10.0;
        }
    }
    if depth == 0 {
        return 0.0;
    }

    if is_maximizing {
        let mut max_eval = -99999.9;
        'y: for y in 0..board.len() {
            for x in 0..board[y].len() {
                if board[y][x] == ' ' {
                    board[y][x] = ai;
                    let eval = minmax(
                        board.clone(),
                        ai,
                        player,
                        lenght,
                        depth - 1,
                        alpha.clone(),
                        beta.clone(),
                        false,
                    );
                    board[y][x] = ' ';
                    if eval > max_eval {
                        max_eval = eval;
                    }
                    if eval > alpha {
                        alpha = eval;
                    }
                    if beta <= alpha {
                        break 'y;
                    }
                }
            }
        }
        return max_eval;
    } else if !is_maximizing {
        let mut max_eval = 99999.9;
        'y2: for y in 0..board.len() {
            for x in 0..board[y].len() {
                if board[y][x] == ' ' {
                    board[y][x] = player;
                    let eval = minmax(
                        board.clone(),
                        ai,
                        player,
                        lenght,
                        depth - 1,
                        alpha.clone(),
                        beta.clone(),
                        true,
                    );
                    board[y][x] = ' ';
                    if eval < max_eval {
                        max_eval = eval;
                    }
                    if eval < beta {
                        beta = eval;
                    }
                    if beta <= alpha {
                        break 'y2;
                    }
                }
            }
        }
        return max_eval;
    }

    return 0.0;
}

fn check_winner(board: &Vec<Vec<char>>, lenght: &u8) -> char {
    let mut turn: char = 'X';
    for _ in 0..2 {
        for y in 0..board.len() {
            let mut in_row_y: u8 = 0;
            let mut in_row_x: u8 = 0;
            for x in 0..board[y].len() {
                if board[y][x] == turn {
                    in_row_x += 1;
                } else {
                    in_row_x = 0;
                }
                if in_row_x == *lenght {
                    return turn;
                }

                if board[x][y] == turn {
                    in_row_y += 1;
                } else {
                    in_row_y = 0;
                }
                if in_row_y == *lenght {
                    return turn;
                }
            }
        }

        // y is for \ check and y2 is for / check
        let mut y = board.len();
        let mut y2 = 0;
        for i in -(board.len() as i32)..board.len() as i32 {
            let mut in_row = 0;
            let mut in_row2 = 0;
            for x in 0..board.len() - i.abs() as usize {
                // / check
                if board[y + x][x] == turn {
                    in_row += 1;
                } else {
                    in_row = 0;
                }
                if in_row == *lenght {
                    return turn;
                }

                // \ check
                if board[y2 - x][x] == turn {
                    in_row2 += 1;
                } else {
                    in_row2 = 0;
                }
                if in_row2 == *lenght {
                    return turn;
                }
            }

            if y != 0 {
                y -= 1;
            }
            if y2 != board.len() - 1 {
                y2 += 1;
            }
        }
        turn = next_turn(turn);
    }

    if is_board_full(&board) == true {
        return 'T';
    }

    return 'N';
}

fn is_board_full(board: &Vec<Vec<char>>) -> bool {
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == ' ' {
                return false;
            }
        }
    }
    return true;
}

fn place_piece(board: &mut Vec<Vec<char>>, turn: char, pos: (u8, u8)) -> &str {
    if board[pos.1 as usize][pos.0 as usize] == ' ' {
        board[pos.1 as usize][pos.0 as usize] = turn;
        return "ok";
    }
    return "piece is already there";
}

fn next_turn(turn: char) -> char{
    match turn {
        'X' => return 'O',
        'O' => return 'X',
        _ => return 'X',
    }
}

fn print_board(board: &Vec<Vec<char>>) {
    //prints top part
    print!("╔═══");
    for _ in 0..board.len() - 1 {
        print!("╦═══");
    }
    println!("╗");

    //prints grid
    for y in 0..board.len() {
        //prints lines with numbers
        for x in 0..board[y].len() {
            print!("║ {} ", board[y][x])
        }
        println!("║");

        // prints middle lines
        if y != board.len() - 1 {
            print!("╠═══");
            for _ in 0..board[y].len() - 1 {
                print!("╬═══");
            }
            println!("╣");
        }
    }

    //prints bottom part
    print!("╚═══");
    for _ in 0..board.len() - 1 {
        print!("╩═══");
    }
    println!("╝");
}

fn clear_bloard(board: &mut Vec<Vec<char>>, size: (u8, u8)) {
    for _ in 0..size.1 {
        let mut temp_vec: Vec<char> = Vec::new();
        for _ in 0..size.0 {
            temp_vec.push(' ');
        }
        board.push(temp_vec);
    }
}
