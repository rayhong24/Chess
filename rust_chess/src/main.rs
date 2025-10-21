use rust_chess::engine::minimax::Minimax;
use rust_chess::game_classes::game::Game;


fn main() {
    let mut game = Game::new();
    let mut eng = Minimax::new(2,4, false, false);
    

    let to_move = game.get_game_state().get_turn();
    eng.find_best_move(&mut game, to_move);
}