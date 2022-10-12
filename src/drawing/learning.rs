use iced::{Element, Column, Length, Text, ProgressBar, Alignment};

use crate::ai::learning::LearningProgress;

fn progress_dots(progress: u32) -> &'static str {
    match progress % 3 {
        0 => ".",
        1 => "..",
        2 => "...",
        _ => unreachable!()
    }
}

pub fn learning_progress<'a>(progress: &LearningProgress) -> Element<'a, ()> {
    Column::new()
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10)
        .push(Text::new(format!("Progress{}", progress_dots(progress.current))))
        .push(ProgressBar::new(0.0..=(progress.max as f32), progress.current as f32))
        .into()
}