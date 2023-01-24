use iced::Theme;

use crate::Message;

pub mod life;
pub mod loaded;
pub mod loading;
pub mod select_talents;

pub type View<'a> = iced::Element<'a, Message, iced::Renderer<Theme>>;
