mod board;
mod setup;
mod choosing;
mod learning;

use iced::{Canvas, Column, Element, Length};

use crate::{game::{Board, Controls, Coord, Species}, mode::GameMode, ai::learning::LearningProgress};

use self::choosing::choosing;

#[derive(Debug)]
pub enum GraphicMsg {
    PinSelected(Coord),
    PinMoved(Coord),
    ControlChanged(Species),
    ModeSelected(GameMode),
}

pub fn view<'a>(board: &Board, controls: &Controls, mode: &GameMode, progress: &LearningProgress) -> Element<'a, GraphicMsg> {
    match mode {
        GameMode::ChoosingMode => choosing_view(),
        GameMode::Playing => playing_view(board, controls),
        GameMode::Learning => learning_progress_view(progress),
    }
}

fn learning_progress_view<'a>(progress: &LearningProgress) -> Element<'a, GraphicMsg> {
    learning::learning_progress(progress).map(|_| unreachable!())
}

fn choosing_view<'a>() -> Element<'a, GraphicMsg> {
    choosing().map(|m| match m {
        choosing::ChoosingGraphicMsg::Play => GraphicMsg::ModeSelected(GameMode::Playing),
        choosing::ChoosingGraphicMsg::Learn => GraphicMsg::ModeSelected(GameMode::Learning),
    })
}

fn playing_view<'a>(board: &Board, controls: &Controls) -> Element<'a, GraphicMsg> {
    let board_graphics = Canvas::new(self::board::BoardGraphic::new(board.clone()))
        .height(Length::Fill)
        .width(Length::Fill);
    let control = setup::view(controls);
    Column::new().push(control).push(board_graphics).into()
}
