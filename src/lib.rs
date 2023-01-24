mod config;
mod load_data;
mod message;
mod views;

use crate::{load_data::ALL_TALENTS, message::Message};
use config::*;
use iced::{window, Application, Command, Settings, Theme};
use load_data::{Talent, ALL_EVENTS};
use rand::seq::SliceRandom;
use std::{thread::sleep, time::Duration};
use views::{
    life::life_simulate_view, loaded::loaded_view, loading::loading_view,
    select_talents::select_talents_view, View,
};

pub fn game_start() -> Result<(), iced::Error> {
    let setting = Settings {
        window: window::Settings {
            size: (DEFAULT_WIN_WIDTH, DEFAULT_WIN_HEIGHT),
            ..window::Settings::default()
        },
        default_font: Some(DEFAULT_FONT.as_slice()),
        ..Settings::default()
    };
    Game::run(setting)
}

#[derive(Debug)]
pub struct LifeEvent {
    event: String,
    age: u32,
}
type EventList = Vec<LifeEvent>;
impl LifeEvent {
    fn new(age: u32, event: String) -> Self {
        LifeEvent { event, age }
    }

    fn display(&self) -> String {
        format!("{}岁: {}", self.age, self.event)
    }
}

#[derive(Debug)]
pub enum Game {
    Loading,
    Loaded,
    Select(Vec<Talent>, Vec<usize>),
    Life(Vec<Talent>, EventList, bool),
}

async fn load() {
    println!("talents' number is {}", ALL_TALENTS.len());
    sleep(Duration::from_millis(500));
}

impl Application for Game {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Game::Loading, Command::perform(load(), |_| Message::Loaded))
    }

    fn title(&self) -> String {
        TITLE.to_owned()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Loaded => {
                *self = Game::Loaded;
                Command::none()
            }
            Message::Start => {
                if let Game::Loaded = self {
                    let mut rng = rand::thread_rng();
                    let talents_list = ALL_TALENTS.choose_multiple(&mut rng, 10);
                    *self = Game::Select(talents_list.cloned().collect(), vec![])
                } else {
                    panic!("wrong game state")
                }
                Command::none()
            }
            Message::ClickTalentItem(target) => {
                if !(0..10).contains(&target) {
                    panic!("wrong select target")
                }

                let (talents, mut cur_chosen) = if let Game::Select(tals, chosen) = self {
                    (tals.clone(), chosen.clone())
                } else {
                    panic!("wrong game state")
                };

                let index = cur_chosen.iter().position(|&x| x == target);
                if let Some(idx) = index {
                    cur_chosen.remove(idx);
                } else if cur_chosen.len() == 3 {
                    println!("max chosen talent")
                } else {
                    cur_chosen.push(target)
                }

                *self = Game::Select(talents, cur_chosen);
                Command::none()
            }
            Message::SubmitTalent => {
                let chosen: Vec<Talent> = if let Game::Select(talents, chosen_idx) = self {
                    if chosen_idx.len() != 3 {
                        return Command::none();
                    }
                    chosen_idx
                        .iter()
                        .map(|&idx| talents.get(idx).unwrap().clone())
                        .collect()
                } else {
                    panic!("wrong game state")
                };

                println!("{:?}", chosen);
                *self = Game::Life(
                    chosen,
                    vec![LifeEvent {
                        event: "你出生了，是个男孩。".to_owned(),
                        age: 0,
                    }],
                    false,
                );
                Command::none()
            }
            Message::PressEventBox => {
                let (events, died) = if let Game::Life(_talents, events, died) = self {
                    (events, died)
                } else {
                    panic!("wrong game state")
                };

                if *died {
                    return Command::none();
                }

                if events.len() == 30 {
                    events.push(LifeEvent::new(events.len() as u32, "... ...".to_owned()));
                    events.push(LifeEvent::new(100, "你死了。".to_owned()));
                    *died = true;
                    return Command::none();
                }

                let rng = &mut rand::thread_rng();
                events.push(LifeEvent::new(
                    events.len() as u32,
                    ALL_EVENTS.choose(rng).unwrap().to_owned(),
                ));
                Command::none()
            }
            Message::GameOver => {
                *self = Game::Loaded;
                Command::none()
            }
        }
    }

    fn view(&self) -> View {
        match self {
            Game::Loading => loading_view(),
            Game::Loaded => loaded_view(),
            Game::Select(talents, chosen) => select_talents_view(talents.clone(), chosen.clone()),
            Game::Life(_, events, died) => life_simulate_view(events, *died),
        }
    }
}
