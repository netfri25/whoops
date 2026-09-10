use macroquad::prelude::*;
use ::rand::prelude::Rng;

use crate::game::Game;

#[derive(Default)]
pub struct App<R> {
    game: Option<Game<R>>,
}

impl<R> App<R>
where
    R: Rng
{
    pub fn new(game: Option<Game<R>>) -> Self {
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
