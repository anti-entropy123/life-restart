use super::View;
use crate::Message;
use iced::{
    alignment,
    widget::{button, column, container, text, Column},
    Length,
};

pub fn loaded_view() -> View<'static> {
    let txt = text("垃圾人生一秒也不想多待").horizontal_alignment(alignment::Horizontal::Center);

    let btn_txt = text("马上重开！").horizontal_alignment(alignment::Horizontal::Center);
    let btn = button(btn_txt).on_press(Message::Start);

    let col: Column<Message> = column![txt, btn].align_items(alignment::Alignment::Center);

    container(col)
        .align_x(alignment::Horizontal::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
}
