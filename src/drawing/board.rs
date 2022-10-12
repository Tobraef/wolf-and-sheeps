use super::GraphicMsg;
use crate::game::{physics::*, Board, Coord, Species};
use iced::{
    canvas::{event::Status, Frame, Path, Program},
    Color, Point, Rectangle, Size,
};

/// radius to block's width ratio
const RADIUS_RATIO: f32 = 0.35;
const SELECTED_RADIUS_RATIO: f32 = 0.4;

pub struct BoardGraphic {
    board: Board,
}

fn current_mover_matches_selected(board: &Board, selected: &Coord) -> bool {
    if let Species::Wolf = board.currently_moving {
        if *selected == board.wolf {
            return true;
        }
    } else if *selected != board.wolf {
        return true;
    }
    false
}

impl BoardGraphic {
    pub fn new(board: Board) -> Self {
        Self { board }
    }

    fn handle_mouse(&self, bounds: &Rectangle, mouse: Point) -> (Status, Option<GraphicMsg>) {
        let block = bounds.width / 8f32;
        let radius = block * RADIUS_RATIO;
        let colliding = self.board.iter().find(|c| {
            let mid_point = coord_to_midpoint(c, block);
            distance_between(&mid_point, &mouse) <= radius
        });
        if let Some(selected) = colliding {
            if current_mover_matches_selected(&self.board, selected) {
                return (
                    Status::Captured,
                    Some(GraphicMsg::PinSelected(selected.clone())),
                );
            }
        }
        if self.board.selected.is_some() {
            let coord = point_to_coord(&mouse, bounds.width / 8f32);
            return (Status::Captured, Some(GraphicMsg::PinMoved(coord)));
        }
        (Status::Ignored, None)
    }
}

fn board(frame: &mut Frame, block: f32) {
    let mut white = true;
    for x in 0i16..8 {
        for y in 0..8 {
            let color = if white { Color::WHITE } else { Color::BLACK };
            white = !white;
            let top_left = top_left_point(x, y, block);
            frame.fill_rectangle(top_left, Size::new(block, block), color);
        }
        white = !white;
    }
}

fn put_circle_in(frame: &mut Frame, coord: &Coord, ratio: f32, block: f32, color: &Color) {
    let center = coord_to_midpoint(coord, block);
    let circle = Path::circle(center, block * ratio);
    frame.fill(&circle, *color)
}

fn pawns(frame: &mut Frame, block: f32, board: &Board) {
    let wolf_color = Color::from_rgb(0.8, 0.5, 0.2);
    let sheep_color = Color::from_rgb(1., 1., 0.);
    if let Some(selected) = &board.selected {
        let selected_color = Color::from_rgb(1., 0., 0.);
        put_circle_in(
            frame,
            selected,
            SELECTED_RADIUS_RATIO,
            block,
            &selected_color,
        );
    }
    put_circle_in(frame, &board.wolf, RADIUS_RATIO, block, &wolf_color);
    for sheep in &board.sheeps {
        put_circle_in(frame, sheep, RADIUS_RATIO, block, &sheep_color);
    }
}

impl Program<GraphicMsg> for BoardGraphic {
    fn draw(
        &self,
        bounds: iced::Rectangle,
        _cursor: iced::canvas::Cursor,
    ) -> Vec<iced::canvas::Geometry> {
        let mut frame = Frame::new(bounds.size());
        let block = bounds.width / 8f32;
        board(&mut frame, block);
        pawns(&mut frame, block, &self.board);
        vec![frame.into_geometry()]
    }

    fn update(
        &mut self,
        event: iced::canvas::Event,
        bounds: iced::Rectangle,
        cursor: iced::canvas::Cursor,
    ) -> (iced::canvas::event::Status, Option<GraphicMsg>) {
        if let iced::canvas::Event::Mouse(iced::mouse::Event::ButtonPressed(_)) = event {
            let mut position = cursor.position().unwrap();
            position.y -= 100.;
            return self.handle_mouse(&bounds, position);
        }
        (Status::Ignored, None)
    }
}
