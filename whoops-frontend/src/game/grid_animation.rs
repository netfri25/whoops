use std::collections::VecDeque;
use std::time::{Duration, Instant};

use macroquad::prelude::miniquad::window::schedule_update;
use macroquad::prelude::*;
use whoops_core::pos::Pos;

use crate::game::draw_tile;
use crate::grid_layout::GridLayout;

const TILE_ANIMATION_DURATION: Duration = Duration::from_millis(200);

#[derive(Default)]
pub struct GridAnimation {
    tile_animations: VecDeque<TileAnimation>,
}

impl GridAnimation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn animate(&mut self, pos: Pos, src_color: Color, dst_color: Color) {
        let duration = TILE_ANIMATION_DURATION;
        self.tile_animations
            .push_back(TileAnimation::new(pos, duration, src_color, dst_color));
    }

    pub fn tick(&mut self) {
        while self
            .tile_animations
            .pop_front_if(|anim| anim.is_done())
            .is_some()
        {}
    }

    pub fn draw(&self, layout: &GridLayout) {
        for anim in self.tile_animations.iter() {
            let pos = anim.pos();
            let color = anim.color();
            draw_tile(layout, pos, color, None);
        }

        if !self.tile_animations.is_empty() {
            // tell macroquad to render another frame
            schedule_update();
        }
    }
}

struct TileAnimation {
    pos: Pos,
    start: Instant,
    duration: Duration,
    src_color: Color,
    dst_color: Color,
}

impl TileAnimation {
    pub fn new(pos: Pos, duration: Duration, src_color: Color, dst_color: Color) -> Self {
        let start = Instant::now();
        Self {
            pos,
            start,
            duration,
            src_color,
            dst_color,
        }
    }

    pub fn pos(&self) -> Pos {
        self.pos
    }

    pub fn is_done(&self) -> bool {
        self.start.elapsed() >= self.duration
    }

    pub fn color(&self) -> Color {
        let elapsed = self.start.elapsed();
        let f = elapsed.div_duration_f32(self.duration);

        Color::new(
            self.src_color.r.lerp(self.dst_color.r, f),
            self.src_color.g.lerp(self.dst_color.g, f),
            self.src_color.b.lerp(self.dst_color.b, f),
            self.src_color.a.lerp(self.dst_color.a, f),
        )
    }
}
