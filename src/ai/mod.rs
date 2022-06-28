mod random_ai;
mod linear_function_ai;
mod remembrance_ai;

use crate::game::{Board, Move};


pub enum AITypes {
    Random,
    Remembrance,
}

pub trait AI {
    fn next_move(&mut self, board: &Board) -> Option<Move>;
}

pub fn get_ai(ai_type: AITypes) -> Box<dyn AI> {
    match ai_type {
        AITypes::Random => Box::new(random_ai::RandomAI::new()),
        AITypes::Remembrance => Box::new(remembrance_ai::RemembranceAI::new()),
    }
}

