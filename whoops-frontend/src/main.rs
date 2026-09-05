use iced::widget::{container, text};
use iced::{Element, Length, Theme};
use whoops_core::tile_grid;

use crate::game::Game;

mod game;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(Theme::Oxocarbon)
        .resizable(true)
        .run()
}

#[derive(Default)]
struct App {
    game: Option<Game>,
}

enum Message {
    Game(game::Message),
}

impl App {
    fn new() -> Self {
        let game = Game::new(tile_grid![
            [3, 2, _, _],
            [_, _, x, _],
            [_, 3, _, _],
            [_, 4, _, 4],
        ]);

        Self { game: Some(game) }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Game(msg) => {
                if let Some(ref mut game) = self.game {
                    game.update(msg)
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        if let Some(ref game) = self.game {
            game.view().map(Message::Game)
        } else {
            container(text("waiting...")).center(Length::Fill).into()
        }
    }
}
