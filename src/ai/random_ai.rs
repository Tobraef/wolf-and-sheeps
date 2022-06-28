use crate::game::{Board, Species, movement::{all_available_wolf_moves, all_available_sheeps_moves}, Move};

use super::AI;
use rand::prelude::*;

pub struct RandomAI;

impl RandomAI {
    pub fn new() -> Self { Self }
}

fn random_from_arr<T>(a: &[T]) -> Option<&T> {
    if a.is_empty() {
        None
    } else {
        let index: usize = random::<usize>() % a.len();
        Some(&a[index])
    }
}

impl AI for RandomAI {
    fn next_move(&mut self, board: &Board) -> Option<Move> {
        match board.currently_moving {
            Species::Wolf => {
                let available_moves = all_available_wolf_moves(&board.wolf, &board.sheeps);
                random_from_arr(&available_moves).map(|random_move| random_move.clone())
            },
            Species::Sheep => {
                let available_sheep_moves = all_available_sheeps_moves(board);
                random_from_arr(&available_sheep_moves).map(|random_move| random_move.clone())
            },
        }
    }
}