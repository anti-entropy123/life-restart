use iced::{
    theme,
    widget::{
        button, column, container,
        scrollable::{self},
        text, Column,
    },
    Alignment, Length, Padding, Theme,
};

use super::View;
use crate::{config::DEFAULT_WIN_HEIGHT, message::Message, EventList, DEFAULT_WIN_WIDTH};

struct LifeEventBoxStyle;

impl button::StyleSheet for LifeEventBoxStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: None,
            text_color: iced::Color::BLACK,
            border_color: iced::Color::BLACK,
            border_width: 1.,
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}

pub fn life_simulate_view(events: &EventList, died: bool) -> View<'static> {
    let title_txt = text("你能活到多少岁？".to_owned()).size(30);

    let event_text_list: Column<Message> = {
        let mut col = column![].spacing(5).padding(Padding {
            top: 15,
            right: 20,
            bottom: 15,
            left: 20,
        });

        for event in events {
            let event_txt = text(event.display()).size(25).width(Length::Fill);

            col = col.push(event_txt);
        }
        col
    };

    let scroll_box = scrollable::Scrollable::new(event_text_list)
        // .vertical_scroll(Properties::new())
        .height(Length::Units((DEFAULT_WIN_HEIGHT as f32 * 0.8) as u16));

    let life_box = button(scroll_box)
        .width(Length::Fill)
        .height(Length::Units((DEFAULT_WIN_HEIGHT as f32 * 0.8) as u16))
        .style(theme::Button::Custom(Box::from(LifeEventBoxStyle)))
        .on_press(Message::PressEventBox);

    let mut col: Column<Message> = column![title_txt, life_box]
        .align_items(Alignment::Center)
        .spacing(10)
        .height(Length::Fill)
        .width(Length::Units((DEFAULT_WIN_WIDTH as f32 * 0.8) as u16));

    if died {
        let game_over_btn = button("游戏结束").on_press(Message::GameOver);
        col = col.push(game_over_btn);
    }

    container(col)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .align_x(iced::alignment::Horizontal::Center)
        .into()
}
