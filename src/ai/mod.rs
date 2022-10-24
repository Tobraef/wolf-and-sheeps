mod linear_function_ai;
mod random_ai;
mod remembrance_ai;
pub mod learning;
mod smart_ai;

use crate::game::{Board, Move};

use std::fmt::Debug;

pub enum AITypes {
    Random,
    Remembrance,
    Smart,
}

pub trait AI: Debug {
    fn next_move(&mut self, board: &Board) -> Option<Move>;
    fn feedback(&mut self, won: bool);
}

pub fn get_ai(ai_type: AITypes) -> Box<dyn AI + Send> {
    match ai_type {
        AITypes::Random => Box::new(random_ai::RandomAI::new()),
        AITypes::Remembrance => Box::new(remembrance_ai::RemembranceAI::new()),
        AITypes::Smart => Box::new(smart_ai::SmartAI {}),
    }
}
