use iced::{
    alignment,
    widget::{container, text},
    Length,
};

use super::View;

pub fn loading_view() -> View<'static> {
    container(
        text("Loading...")
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(30),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .center_x()
    .into()
}
