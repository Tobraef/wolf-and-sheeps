mod drawing;
mod game;
mod ai;

use std::time::Duration;

use ai::{get_ai, AITypes};
use game::{Board, Coord, engine, Species, Controls, Move};
use iced::{Application, Command, Settings};

struct App {
    board: Board,
    controls: Controls,
    ai: Box<dyn ai::AI>,
}

#[derive(Debug)]
enum Msg {
    PinSelected(Coord),
    PinMoved(Coord),
    Tick,
    ControlChanged(Species),
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Message = Msg;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self { board: Default::default(), ai: get_ai(AITypes::Random), controls: Default::default() }, Command::none())
    }

    fn title(&self) -> String {
        "Wolf and sheeps".to_owned()
    }

    fn update(
        &mut self,
        message: Self::Message
    ) -> iced::Command<Self::Message> {
        match message {
            Msg::PinSelected(selected) => {
                self.board.selected = Some(selected);
            },
            Msg::PinMoved(moved_to) => {
                self.board.selected.clone()
                    .map(|selected| engine::handle_move(&mut self.board, &Move::new(selected, moved_to))
                    .map(|winner| engine::handle_win(winner, &mut self.board)));
            },
            Msg::Tick => {
                engine::handle_tick(&mut self.board, &self.controls, &mut self.ai)
                    .map(|winner| engine::handle_win(winner, &mut self.board));
            },
            Msg::ControlChanged(species) => {
                engine::handle_control_change(&mut self.controls, species);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        drawing::view(&self.board, &self.controls).map(|m| match m {
            drawing::GraphicMsg::PinSelected(selected) => Msg::PinSelected(selected),
            drawing::GraphicMsg::PinMoved(to) => Msg::PinMoved(to),
            drawing::GraphicMsg::ControlChanged(species) => Msg::ControlChanged(species),
        })
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(Duration::from_millis(100)).map(|_| Msg::Tick)
    }
}

fn main() {
    App::run(Settings {
        window: iced::window::Settings {
            resizable: false,
            size: (500, 600),
            ..Default::default()
        },
        ..Default::default()
    }).unwrap()
}
