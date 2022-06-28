mod board;
mod setup;

use iced::{Element, Length, Canvas, Column};

use crate::game::{Board, Coord, Species, Controls};

#[derive(Debug)]
pub enum GraphicMsg {
    PinSelected(Coord),
    PinMoved(Coord),
    ControlChanged(Species),
}

pub fn view<'a>(board: &Board, controls: &Controls) -> Element<'a, GraphicMsg> {
    let board_graphics = Canvas::new(self::board::BoardGraphic::new(board.clone()))
        .height(Length::Fill)
        .width(Length::Fill);
    let control = setup::view(controls);
    Column::new()
        .push(control)
        .push(board_graphics)
        .into()
}
