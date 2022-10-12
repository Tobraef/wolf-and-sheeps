use iced::{Element, Column, Length, Button, button::State, Text, Alignment};

#[derive(Clone)]
pub enum ChoosingGraphicMsg {
    Play,
    Learn,
}

static mut PLAY_BUTTON_STATE: Option<State> = None;
static mut LEARN_BUTTON_STATE: Option<State> = None;

pub fn choosing<'a>() -> Element<'a, ChoosingGraphicMsg> {
    let play_state = unsafe { PLAY_BUTTON_STATE.get_or_insert_with(State::new) };
    let learn_state = unsafe { LEARN_BUTTON_STATE.get_or_insert_with(State::new) };
    Column::new()
        .width(Length::Fill)
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10)
        .push(Button::new(play_state, Text::new("Play"))
            .on_press(ChoosingGraphicMsg::Play))
        .push(Button::new(learn_state, Text::new("Learn"))
            .on_press(ChoosingGraphicMsg::Learn))
        .into()
}