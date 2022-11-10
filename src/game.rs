use std::time::Duration;
use std::{io::Write, thread};

//use crate::ai::*;
use crate::board::*;

pub fn start_game() -> game {
    let mut game = game::new();

    print!("set size of the board: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    game.size = {
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
    game.lenght = input.trim().parse::<u8>().unwrap();

    print!("set AI depth: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    game.depth = input.trim().parse::<i32>().unwrap();

    // print!("is multi threading?(1,0): ");
    // std::io::stdout().flush().unwrap();
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // lenght = input.trim().parse::<u8>().unwrap();
    // print!("{:?}", sus);
    game.clear_bloard();

    game.print_board();
    // println!("board: {:?}", board);

    game
}

pub fn game_loop(mut game: game) {
    loop {
        if game.turn == 'X' {
            print!("{} set piece (x y): ", game.turn);
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let pos = {
                let temp: Vec<&str> = input.split(" ").collect();
                (
                    temp[0].trim().parse::<u8>().unwrap(),
                    temp[1].trim().parse::<u8>().unwrap(),
                )
            };
            if game.place_piece(pos) != "piece is already there" {
                game.turn = game.next_turn();
            } else {
                println!("invalid piece place");
            }

            // let time = std::time::Instant::now();
            //AI_multi_thread(&mut board, 'X', lenght, depth);

            //turn = next_turn(turn);
            //println!("AI time: {}", time.elapsed().as_secs_f64());
        } else {
            let time = std::time::Instant::now();
            AI(&mut board, 'O', &lenght, depth);
            //game.AI_multi_thread();
            //0.23
            println!("AI time: {}", time.elapsed().as_secs_f64());
            game.turn = game.next_turn();
        }
        game.print_board();
        // println!("board: {:?}", board);
        let winner = game.check_winner();
        if winner != 'N' {
            println!("{}, won!", winner);
            break;
        }
        thread::sleep(Duration::from_secs_f64(1.5));
    }
}
