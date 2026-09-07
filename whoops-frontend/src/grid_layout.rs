use macroquad::prelude::*;
use whoops_core::pos::Pos;

pub struct GridLayout {
    origin: Vec2,
    size: Vec2,
    grid_count: Vec2,
}

impl GridLayout {
    pub fn new(bounds: Rect, width: u32, height: u32) -> Self {
        let origin = bounds.point();
        let size = bounds.size();
        let grid_count = Vec2::new(width as f32, height as f32);
        Self {
            origin,
            size,
            grid_count,
        }
    }

    pub fn radius(&self) -> f32 {
        self.tile_size() * 0.45
    }

    pub fn font_size(&self) -> f32 {
        self.tile_size() / 2.
    }

    pub fn position_of(&self, pos: Pos) -> Vec2 {
        let tl = self.top_left();
        let offset = Vec2::new(pos.x as f32, pos.y as f32) * self.tile_size();
        tl + offset
    }

    pub fn center_of(&self, pos: Pos) -> Vec2 {
        self.bounds_of(pos).center()
    }

    pub fn bounds_of(&self, pos: Pos) -> Rect {
        let Vec2 { x, y } = self.position_of(pos);
        let size = self.tile_size();
        Rect::new(x, y, size, size)
    }

    pub fn tile_at(&self, point: Vec2) -> Option<Pos> {
        let norm = point - self.top_left();
        let br = self.bottom_right();

        if norm.x < 0. || norm.y < 0. || point.x >= br.x || point.y >= br.y {
            return None;
        }

        let target = norm / self.tile_size();
        let pos = Pos::new(target.x as u32, target.y as u32);

        // TODO: check inside circle?

        Some(pos)
    }

    fn size_vec(&self) -> Vec2 {
        self.size / self.grid_count
    }

    pub fn tile_size(&self) -> f32 {
        self.size_vec().min_element()
    }

    pub fn top_left(&self) -> Vec2 {
        self.origin + (self.size_vec() - self.tile_size()) * self.grid_count / 2.
    }

    pub fn bottom_right(&self) -> Vec2 {
        self.top_left() + self.tile_size() * self.grid_count
    }
}
