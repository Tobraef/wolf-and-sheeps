use crate::game::{Board, engine, Species};

use super::{AI, get_ai, AITypes};

const GAMES_TO_LEARN: u32 = 100_000;

pub struct LearningProgress {
    pub current: u32,
    pub max: u32,
}

impl LearningProgress {
    pub fn new() -> Self {
        Self {
            current: 0,
            max: GAMES_TO_LEARN,
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.current == self.max {
            panic!("Logic bug");
        }
        self.current += 1;
        self.current == self.max
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }
}

pub fn learning_session(ai: &mut dyn AI, species: Species) {
    let mut random_opponent = get_ai(AITypes::Random);
    let mut board = Board::default();
    match species {
        Species::Wolf => loop {
            let first_move = &ai.next_move(&board).unwrap();
            if engine::handle_move(&mut board, first_move).is_some() {
                break;
            }
            let second_move = &random_opponent.next_move(&board).unwrap();
            if engine::handle_move(&mut board, second_move).is_some() {
                break;
            }
        },
        Species::Sheep => loop {
            let first_move = &random_opponent.next_move(&board).unwrap();
            if engine::handle_move(&mut board, first_move).is_some() {
                break;
            }
            let second_move = &ai.next_move(&board).unwrap();
            if engine::handle_move(&mut board, second_move).is_some() {
                break;
            }
        },
    }
}
