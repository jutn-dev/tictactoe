use crate::board::game;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{f32::INFINITY, sync::mpsc, thread};

impl game {
    pub fn AI(&mut self) {
        let mut next_move: (usize, usize) = (0, 0);

        let mut max_eval = -INFINITY;
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                if self.board[y][x] == ' ' {
                    self.board[y][x] = self.ai;
                    self.peaces_placed += 1;
                    let eval = self.clone().minmax(self.depth, -99999.9, 99999.9, false);
                    self.board[y][x] = ' ';
                    self.peaces_placed -= 1;
                    if eval > max_eval {
                        max_eval = eval;
                        next_move = (y as usize, x as usize);
                    }
                }
            }
        }

        self.board[next_move.0][next_move.1] = self.ai;
    }

    pub fn AI_multi_thread(&mut self) {
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
        for y in 0..self.board.len() {
            for x in 0..self.board.len() {
                if self.board[y][x] == ' ' {
                    let threads_ = Arc::clone(&threads);
                    // let threads = Arc::clone(&threads);
                    let tx_ = tx.clone();
                    let mut game_ = self.clone();
                    thread::spawn(move || {
                        threads_.fetch_add(1, Ordering::SeqCst);
                        // println!("working...");
                        let mut val: (usize, usize, f32) = (y, x, 0.0);
                        val.0 = y;
                        val.1 = x;
                        game_.peaces_placed += 1;
                        game_.board[y][x] = game_.ai;
                        val.2 = game_.clone().minmax(game_.depth, -99999.9, 99999.9, false);
                        game_.board[y][x] = ' ';
                        game_.peaces_placed += 1;
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
        self.board[best_result.0][best_result.1] = self.ai;
    }

    fn minmax(&mut self, depth: i32, mut alpha: f32, mut beta: f32, is_maximizing: bool) -> f32 {
        let winner = self.clone().check_winner();
        if winner != 'N' {
            if winner == self.ai {
                return 10.0;
            }
            if winner == 'T' {
                return 0.0;
            }
            if winner != self.ai {
                return -10.0;
            }
        }
        if depth == 0 {
            return 0.0;
        }

        if is_maximizing {
            let mut max_eval = -99999.9;
            'y: for y in 0..self.board.len() {
                for x in 0..self.board[y].len() {
                    if self.board[y][x] == ' ' {
                        self.board[y][x] = self.ai;
                        self.peaces_placed += 1;
                        let eval = self.minmax(depth - 1, alpha.clone(), beta.clone(), false);
                        self.board[y][x] = ' ';
                        self.peaces_placed -= 1;
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
            'y2: for y in 0..self.board.len() {
                for x in 0..self.board[y].len() {
                    if self.board[y][x] == ' ' {
                        self.board[y][x] = self.players[0].player;
                        self.peaces_placed += 1;
                        let eval = self.minmax(depth - 1, alpha.clone(), beta.clone(), true);
                        self.board[y][x] = ' ';
                        self.peaces_placed -= 1;
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
}
