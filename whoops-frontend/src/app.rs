use macroquad::prelude::*;

use crate::game::Game;

#[derive(Default)]
pub struct App {
    game: Option<Game>,
}

impl App {
    pub fn new(game: Option<Game>) -> Self {
        Self { game }
    }

    pub fn update(&mut self, bounds: Rect) {
        if let Some(ref mut game) = self.game {
            game.update(bounds)
        }
    }

    pub fn draw(&self, bounds: Rect) {
        if let Some(ref game) = self.game {
            game.draw(bounds)
        }
    }
}
