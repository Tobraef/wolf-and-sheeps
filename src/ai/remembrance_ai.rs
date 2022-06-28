use std::collections::HashMap;
use crate::game::{Coord, Species, movement::{all_available_sheeps_moves, all_available_wolf_moves}, Board, Move};
use super::AI;

#[derive(Debug)]
pub struct GameState {
    is_ok: bool, 
    sheeps: [Coord; 4],
}

pub struct RemembranceAI {
    data: HashMap<Coord, Vec<GameState>>,
    previous_move: (Move, Coord, [Coord; 4]),
}

impl RemembranceAI {
    pub fn new() -> Self { Self { ..Default::default() } }

    #[cfg(test)]
    pub fn setup(data: HashMap<Coord, Vec<GameState>>, previous_move: (Move, Coord, [Coord; 4])) -> Self {
        Self { data, previous_move }
    }
}

impl GameState {
    pub fn new(is_ok: bool, sheeps: [Coord; 4]) -> Self {
        Self { is_ok, sheeps }
    }
}

impl Default for RemembranceAI {
    fn default() -> Self {
        Self { data: Default::default(), previous_move: (Move::new(Coord::new(0,0),Coord::new(0,0)), Coord::new(0,0), [
            Coord::new(0,0),
            Coord::new(0,0),
            Coord::new(0,0),
            Coord::new(0,0),
        ]) }
    }
}

pub fn state_after_sheep_move(s_move: &Move, sheeps: &[Coord]) -> [Coord; 4] {
    let mut to_ret = [sheeps[0].clone(), sheeps[1].clone(), sheeps[2].clone(), sheeps[3].clone()];
    let to_swap = to_ret.iter_mut().find(|s| **s == s_move.from).unwrap();
    *to_swap = s_move.to.clone();
    to_ret
}

pub fn state_is_lost_for_sheep(sheeps: &[Coord], wolf: &Coord) -> bool {
    let last_sheep_y = sheeps
        .iter()
        .map(|s| s.y)
        .max()
        .unwrap();
    if wolf.y >= last_sheep_y {
        return true
    }
    let possible_wolf_moves = all_available_wolf_moves(wolf, sheeps);
    possible_wolf_moves
        .iter()
        .any(|mv| mv.to.y >= last_sheep_y)
}

fn move_based_on_data(ai: &mut RemembranceAI, board: &Board, available_moves: &[Move]) -> Move {
    // list all possible states that are checked and list unchecked
    // add all unchecked states to checked by checking, whether wolf wins after that move
    // if so, the state is false
    // if after adding, all checked states lead to wolf win, sheep cannot allow current state to happen, thus setting 
    let all_possible_states = available_moves
        .iter()
        .map(|s_move| state_after_sheep_move(s_move, &board.sheeps));
    let checked_states = ai.data
        .entry(board.wolf.clone())
        .or_default();
    let unchecked_possible_states: Vec<_> = all_possible_states
        .filter(|state|
            checked_states.iter().any(|s| s.sheeps != *state))
        .collect();
    check_and_update_states(unchecked_possible_states, checked_states, &board.wolf);
    if let Some(state) = checked_states.iter().find(|s| s.is_ok) {
        let chosen_move = available_moves
            .iter()
            .find(|&s_move| state.sheeps == state_after_sheep_move(s_move, &board.sheeps))
            .unwrap()
            .clone();
        ai.previous_move = (chosen_move.clone(), board.wolf.clone(), board.sheeps.clone());
        chosen_move
    } else {
        drop(checked_states);
        mark_previous_move_as_fail(ai);
        available_moves[0].clone()
    }
}

pub fn mark_previous_move_as_fail(ai: &mut RemembranceAI) {
    let previous_states = ai.data
        .entry(ai.previous_move.1.clone())
        .or_default();
    let previous_state = previous_states
        .iter_mut()
        .find(|s| s.sheeps == ai.previous_move.2)
        .unwrap();
    previous_state.is_ok = false;
}

pub fn check_and_update_states(unchecked_possible_states: Vec<[Coord; 4]>, checked_states: &mut Vec<GameState>, wolf: &Coord) {
    if !unchecked_possible_states.is_empty() {
        let check_states = unchecked_possible_states
            .into_iter()
            .map(|state| GameState::new(!state_is_lost_for_sheep(&state, wolf), state));
        checked_states.extend(check_states);
    }
}

impl AI for RemembranceAI {
    fn next_move(&mut self, board: &Board) -> Option<Move> {
        if matches!(board.currently_moving, Species::Wolf) {
            todo!("Remembrance AI not implemented for wolf");
        }
        let possible_moves = all_available_sheeps_moves(board);
        Some(move_based_on_data(self, board, &possible_moves))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sheeps(a1: u8, b1: u8, a2: u8, b2: u8, a3: u8, b3: u8, a4: u8, b4: u8) -> [Coord; 4] {
        [
            xy(a1, b1),
            xy(a2, b2),
            xy(a3, b3),
            xy(a4, b4),]
    }

    fn xy(a: u8, b: u8) -> Coord { Coord::new(a,b) }

    fn mv_c(a: &Coord, b: &Coord) -> Move { Move::new(a.clone(), b.clone()) }

    #[test]
    fn state_after_sheep_move_should_change_single_sheep() {
        let sheeps = sheeps(
            0,0,
            2,2,
            4,6,
            7,7);
        for sheep in &sheeps {
            let sheeps = state_after_sheep_move(&mv_c(sheep, &xy(1,3)), &sheeps);
            assert!(sheeps.contains(&xy(1,3)) && !sheeps.contains(sheep));
        }
    }

    #[test]
    fn state_is_lost_for_sheep_should_return_true_if_wolf_is_move_from_win() {
        let sheepss = sheeps(
            3, 3,
            4, 4,
            5, 5,
            6, 6
        );
        
        assert!(state_is_lost_for_sheep(&sheepss, &xy(3, 5)));
        assert!(!state_is_lost_for_sheep(&sheepss, &xy(2, 4)));
        
        let sheeps = sheeps(
            0, 2,
            2, 2,
            4, 2, 
            6, 2,
        );
        for i in (1..8).step_by(2) {
            assert!(!state_is_lost_for_sheep(&sheeps, &xy(i, 1)));
        }
    }

    #[test]
    fn mark_previous_move_as_fail_should_find_previous_state_in_data_and_mark_it() {
        let mut ai = RemembranceAI::setup(
            [
                (xy(3,3), vec![
                    GameState::new(true, sheeps(0,0, 1, 1, 2, 2, 3, 3)),
                    GameState::new(true, sheeps(0,0, 1, 1, 3, 3, 2, 2)),
                    GameState::new(false, sheeps(0,0, 1, 1, 2, 2, 4, 4)),
                ]),
                (xy(1,2), vec![GameState::new(true, sheeps(0,0, 1, 1, 2, 2, 3, 3))]),
            ].into_iter().collect(),
            (Move::new(xy(0,2), xy(1,1)), Coord::new(3,3), sheeps(0,0, 1,1, 3,3, 2,2))
        );

        mark_previous_move_as_fail(&mut ai);

        let found_move_in_data = ai.data[&xy(3,3)].iter().find(|x| x.sheeps[2] == xy(3, 3)).unwrap();
        assert!(!found_move_in_data.is_ok);
    }
}