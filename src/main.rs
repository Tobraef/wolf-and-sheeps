mod ai;
mod drawing;
mod game;
mod mode;

use std::{time::Duration, ops::DerefMut};

use ai::{get_ai, AITypes, learning::{LearningProgress, learning_session}};
use game::{engine, Board, Controls, Coord, Move, Species};
use iced::{Application, Command, Settings};
use mode::GameMode;

struct App {
    board: Board,
    controls: Controls,
    mode: GameMode,
    learning_progress: LearningProgress,
    ai: Box<dyn ai::AI>,
}

#[derive(Debug)]
enum Msg {
    PinSelected(Coord),
    PinMoved(Coord),
    Tick,
    ControlChanged(Species),
    NewMode(GameMode),
    AILearned(Box<dyn ai::AI + Send>),
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Message = Msg;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                mode: GameMode::new(),
                board: Default::default(),
                ai: get_ai(AITypes::Remembrance),
                controls: Default::default(),
                learning_progress: LearningProgress::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Wolf and sheeps".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Msg::PinSelected(selected) => {
                self.board.selected = Some(selected);
            }
            Msg::PinMoved(moved_to) => {
                self.board.selected.clone().map(|selected| {
                    engine::handle_move(&mut self.board, &Move::new(selected, moved_to))
                        .map(|winner| {
                            self.ai.feedback(false);
                            engine::handle_win(winner, &mut self.board)
                    })
                });
            }
            Msg::Tick => {
                engine::handle_tick(&mut self.board, &self.controls, &mut self.ai)
                    .map(|winner| engine::handle_win(winner, &mut self.board));
            }
            Msg::ControlChanged(species) => {
                engine::handle_control_change(&mut self.controls, species);
            }
            Msg::NewMode(mode) => {
                self.mode = mode;
                if let GameMode::Learning = &self.mode {
                    self.learning_progress = LearningProgress::new();
                    return Command::perform(std::future::ready(
                        ai::get_ai(AITypes::Remembrance)), Msg::AILearned)
                }
            },
            Msg::AILearned(mut ai) => {
                println!("Learnt");
                if self.learning_progress.tick() {
                    self.ai = ai;
                    self.mode = GameMode::Playing;
                } else {
                    return Command::perform(async move {
                        learning_session(ai.deref_mut(), Species::Sheep);
                        ai
                    }, Msg::AILearned)
                }
            },
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        drawing::view(&self.board, &self.controls, &self.mode, &self.learning_progress).map(|m| match m {
            drawing::GraphicMsg::PinSelected(selected) => Msg::PinSelected(selected),
            drawing::GraphicMsg::PinMoved(to) => Msg::PinMoved(to),
            drawing::GraphicMsg::ControlChanged(species) => Msg::ControlChanged(species),
            drawing::GraphicMsg::ModeSelected(mode) => Msg::NewMode(mode),
        })
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(Duration::from_millis(100)).map(|_| Msg::Tick)
    }
}

#[tokio::main]
async fn main() {
    App::run(Settings {
        window: iced::window::Settings {
            resizable: false,
            size: (500, 600),
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap()
}
