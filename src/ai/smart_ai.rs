use rand::random;

use crate::game::{Species, Board, Move, movement::all_available_wolf_moves};

//non-learning just move down as wolf as much as possible
#[derive(Debug)]
pub struct SmartAI;

impl super::AI for SmartAI {
    fn next_move(&mut self, board: &Board) -> Option<Move> {
        match board.currently_moving {
            Species::Wolf => self.move_as_wolf(board),
            Species::Sheep => todo!("Not implemented yet"),
        }
    }

    fn feedback(&mut self, _won: bool) {
        
    }
}

impl SmartAI {
    fn move_as_wolf(&self, board: &Board) -> Option<Move> {
        let possible_moves = all_available_wolf_moves(&board.wolf, &board.sheeps);
        let moves_down: Vec<_> = possible_moves
            .iter()
            .cloned()
            .filter(|m| m.to.y > board.wolf.y)
            .collect();
        let move_to_choose_from = if !moves_down.is_empty() { &moves_down } else { &possible_moves };
        if random::<bool>() {
            move_to_choose_from.first().map(|x| x.clone())
        } else {
            move_to_choose_from.last().map(|x| x.clone())
        }
    }
}