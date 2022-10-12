use crate::ai::AI;

use super::{
    board::{Control, Controls, Move, Species},
    movement::*,
    Board,
};

fn has_a_winner(board: &Board) -> Option<Species> {
    if board.wolf.y >= board.sheeps.iter().map(|x| x.y).max().unwrap() {
        Some(Species::Wolf)
    } else if wolf_cant_move(board) {
        Some(Species::Sheep)
    } else {
        None
    }
}

fn change_current_mover(board: &mut Board) {
    board.currently_moving = match board.currently_moving {
        Species::Wolf => Species::Sheep,
        Species::Sheep => Species::Wolf,
    }
}

fn deselect_pin(board: &mut Board) {
    board.selected = None;
}

#[must_use]
pub fn handle_move(board: &mut Board, mv: &Move) -> Option<Species> {
    if valid_move(board, mv) {
        move_pin(board, mv);
        change_current_mover(board);
        deselect_pin(board);
        has_a_winner(board)
    } else {
        None
    }
}

fn computer_moving(
    current_mover: &Species,
    wolf_control: &Control,
    sheep_control: &Control,
) -> bool {
    if matches!(current_mover, Species::Wolf) {
        matches!(wolf_control, Control::Computer)
    } else {
        matches!(sheep_control, Control::Computer)
    }
}

#[must_use]
pub fn handle_tick(
    board: &mut Board,
    controls: &Controls,
    ai: &mut Box<dyn AI>,
) -> Option<Species> {
    if computer_moving(
        &board.currently_moving,
        &controls.wolf_controlled_by,
        &controls.sheep_controlled_by,
    ) {
        if let Some(mv) = ai.next_move(board) {
            return handle_move(board, &mv);
        } else {
            change_current_mover(board);
        }
    }
    None
}

fn opposite(control: &Control) -> Control {
    match control {
        Control::Player => Control::Computer,
        Control::Computer => Control::Player,
    }
}

pub fn handle_control_change(controls: &mut Controls, species: Species) {
    match species {
        Species::Wolf => controls.wolf_controlled_by = opposite(&controls.wolf_controlled_by),
        Species::Sheep => controls.sheep_controlled_by = opposite(&controls.sheep_controlled_by),
    }
}

pub fn handle_win(winner: Species, board: &mut Board) {
    println!("{:?} wins!", winner);
    *board = Board::default();
}
