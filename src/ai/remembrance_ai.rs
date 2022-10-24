use super::AI;
use crate::game::{
    movement::{all_available_sheeps_moves, all_available_wolf_moves},
    Board, Coord, Move, Species,
};
use std::collections::{HashMap, HashSet};

type Sheeps = [Coord; 4];

#[derive(Debug)]
pub struct RemembranceAI {
    losing_states: HashMap<Coord, HashSet<Sheeps>>,
    previous_move: (Coord, [Coord; 4]),
}

impl RemembranceAI {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub fn setup(
        losing_states: HashMap<Coord, HashSet<Sheeps>>,
        previous_move: (Coord, [Coord; 4]),
    ) -> Self {
        Self {
            losing_states,
            previous_move,
        }
    }
}

impl Default for RemembranceAI {
    fn default() -> Self {
        Self {
            losing_states: Default::default(),
            previous_move: (
                Coord::new(0, 0),
                [
                    Coord::new(0, 0),
                    Coord::new(0, 0),
                    Coord::new(0, 0),
                    Coord::new(0, 0),
                ],
            ),
        }
    }
}

pub fn state_after_sheep_move(s_move: &Move, sheeps: &[Coord]) -> [Coord; 4] {
    let mut to_ret = [
        sheeps[0].clone(),
        sheeps[1].clone(),
        sheeps[2].clone(),
        sheeps[3].clone(),
    ];
    let to_swap = to_ret.iter_mut().find(|s| **s == s_move.from).unwrap();
    *to_swap = s_move.to.clone();
    to_ret
}

pub fn state_is_lost_for_sheep(sheeps: &[Coord], wolf: &Coord) -> bool {
    let last_sheep_y = sheeps.iter().map(|s| s.y).max().unwrap();
    if wolf.y >= last_sheep_y {
        return true;
    }
    let possible_wolf_moves = all_available_wolf_moves(wolf, sheeps);
    possible_wolf_moves.iter().any(|mv| mv.to.y >= last_sheep_y)
}

fn move_based_on_data(ai: &mut RemembranceAI, board: &Board, available_moves: &[Move]) -> Move {
    // find any move that isn't marked as failed
    // if there is none, mark previous as failed
    if let Some(losing_states) = ai.losing_states.get(&board.wolf) {
        let ok_possible_state = available_moves
            .iter()
            .map(|s_move| (s_move, state_after_sheep_move(s_move, &board.sheeps)))
            .find(|(_, state)| !losing_states.contains(state));
        if let Some((mv, state)) = ok_possible_state {
            ai.previous_move = (board.wolf.clone(), state);
            mv.clone()
        } else {
            mark_previous_move_as_fail(ai);
            available_moves[0].clone()
        }
    } else {
        ai.previous_move = (board.wolf.clone(), state_after_sheep_move(&available_moves[0], &board.sheeps));
        available_moves[0].clone()
    }
}

pub fn mark_previous_move_as_fail(ai: &mut RemembranceAI) {
    let previous_states = ai.losing_states.entry(ai.previous_move.0.clone()).or_default();
    previous_states.insert(ai.previous_move.1.clone());
}

impl AI for RemembranceAI {
    fn feedback(&mut self, won: bool) {
        if !won {
            mark_previous_move_as_fail(self);
        }
    }

    fn next_move(&mut self, board: &Board) -> Option<Move> {
        if matches!(board.currently_moving, Species::Wolf) {
            todo!("Remembrance AI not implemented for wolf");
        }
        let possible_moves = all_available_sheeps_moves(board);
        if possible_moves.is_empty() {
            None
        } else {
            Some(move_based_on_data(self, board, &possible_moves))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sheeps(a1: u8, b1: u8, a2: u8, b2: u8, a3: u8, b3: u8, a4: u8, b4: u8) -> [Coord; 4] {
        [xy(a1, b1), xy(a2, b2), xy(a3, b3), xy(a4, b4)]
    }

    fn xy(a: u8, b: u8) -> Coord {
        Coord::new(a, b)
    }

    fn mv_c(a: &Coord, b: &Coord) -> Move {
        Move::new(a.clone(), b.clone())
    }

    #[test]
    fn state_after_sheep_move_should_change_single_sheep() {
        let sheeps = sheeps(0, 0, 2, 2, 4, 6, 7, 7);
        for sheep in &sheeps {
            let sheeps = state_after_sheep_move(&mv_c(sheep, &xy(1, 3)), &sheeps);
            assert!(sheeps.contains(&xy(1, 3)) && !sheeps.contains(sheep));
        }
    }

    #[test]
    fn state_is_lost_for_sheep_should_return_true_if_wolf_is_move_from_win() {
        let sheepss = sheeps(3, 3, 4, 4, 5, 5, 6, 6);

        assert!(state_is_lost_for_sheep(&sheepss, &xy(3, 5)));
        assert!(!state_is_lost_for_sheep(&sheepss, &xy(2, 4)));

        let sheeps = sheeps(0, 2, 2, 2, 4, 2, 6, 2);
        for i in (1..8).step_by(2) {
            assert!(!state_is_lost_for_sheep(&sheeps, &xy(i, 1)));
        }
    }

    #[test]
    fn mark_previous_move_as_fail_should_find_previous_state_in_data_and_mark_it() {
        let mut ai = RemembranceAI::setup(
            [
                (
                    xy(3, 3),
                    HashSet::from_iter(
                        std::iter::once(sheeps(0, 0, 1, 1, 2, 2, 4, 4))
                    ),
                ),
            ]
            .into_iter()
            .collect(),
            (
                Coord::new(3, 3),
                sheeps(0, 0, 1, 1, 3, 3, 2, 2),
            ),
        );

        mark_previous_move_as_fail(&mut ai);

        let found_move_in_data = ai.losing_states[&xy(3, 3)]
            .iter()
            .find(|x| x[2] == xy(3, 3));
        assert!(found_move_in_data.is_some());
    }
}
