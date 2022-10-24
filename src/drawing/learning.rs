use iced::{Element, Column, Length, Text, ProgressBar, Alignment};

use crate::ai::learning::LearningProgress;

pub fn learning_progress<'a>(progress: &LearningProgress) -> Element<'a, ()> {
    Column::new()
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10)
        .push(ProgressBar::new(0.0..=(progress.max as f32), progress.current as f32))
        .into()
}