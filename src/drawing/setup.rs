use iced::{Alignment, Checkbox, Column, Element, Length, Row};

use super::GraphicMsg;
use crate::game::{Control, Controls, Species};

fn species_column<'a>(control: &Control, species: Species) -> Column<'a, GraphicMsg> {
    let is_ai_controlled = matches!(control, Control::Computer);
    let label = format!("{:?} AI controlled", species);
    Column::new()
        .width(Length::FillPortion(1))
        .align_items(Alignment::Center)
        .push(Checkbox::new(is_ai_controlled, label, move |_| {
            GraphicMsg::ControlChanged(species.clone())
        }))
}

pub fn view<'a>(controls: &Controls) -> Element<'a, GraphicMsg> {
    Row::new()
        .width(Length::Fill)
        .height(Length::Units(100))
        .align_items(Alignment::Center)
        .push(species_column(&controls.wolf_controlled_by, Species::Wolf))
        .push(species_column(
            &controls.sheep_controlled_by,
            Species::Sheep,
        ))
        .into()
}
