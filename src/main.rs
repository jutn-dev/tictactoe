mod board;
mod game;

fn main() {
    game::game_loop(game::start_game());
}
