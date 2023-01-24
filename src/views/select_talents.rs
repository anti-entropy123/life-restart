use iced::widget::{button, column, container, Column};
use iced::{alignment, widget::text};
use iced::{theme, Background, Length, Theme};
use lazy_static::lazy_static;

use crate::load_data::Talent;
use crate::{Message, DEFAULT_WIN_WIDTH};

use super::View;

lazy_static! {
    static ref BUTTON_ACTIVED_COLOR: Background =
        Background::Color(iced::Color::from_rgb8(0x6b, 0x0a, 0xff));
}

struct TalentButtonStyle;

impl button::StyleSheet for TalentButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(*BUTTON_ACTIVED_COLOR),
            text_color: iced::Color::WHITE,
            ..button::Appearance::default()
        }
    }
}

pub fn select_talents_view(talents: Vec<Talent>, chosen: Vec<usize>) -> View<'static> {
    let title = text("选择你的天赋")
        .size(30)
        .horizontal_alignment(alignment::Horizontal::Center);

    let choosable = {
        let mut col: Column<Message> = column!();

        for (idx, talent) in talents.iter().enumerate() {
            let btn_txt =
                text(talent.display()).horizontal_alignment(alignment::Horizontal::Center);

            let mut talent_item = button(btn_txt)
                .width(Length::Fill)
                .on_press(Message::ClickTalentItem(idx));

            if chosen.contains(&idx) {
                let chosen_btn_style = theme::Button::Custom(Box::from(TalentButtonStyle));
                talent_item = talent_item.style(chosen_btn_style);
            };
            col = col.push(talent_item);
        }
        col.align_items(iced::Alignment::Center)
    };

    let submit_btn_txt = text("确认").horizontal_alignment(alignment::Horizontal::Center);
    let submit_btn = button(submit_btn_txt)
        .width(Length::Units((0.3 * DEFAULT_WIN_WIDTH as f32) as u16))
        .on_press(Message::SubmitTalent);

    let col: Column<Message> = column![title, choosable, submit_btn]
        .width(Length::Units((0.5 * DEFAULT_WIN_WIDTH as f32) as u16))
        .align_items(alignment::Alignment::Center)
        .spacing(70);

    container(col)
        .align_x(alignment::Horizontal::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .center_x()
        .into()
}
