use std::ops::{Deref, DerefMut};
use std::sync::LazyLock;

use macroquad::prelude::*;

use whoops_core::grid::{Grid, GridIter};
use whoops_core::pos::Pos;
use whoops_core::tile::Tile;

use crate::grid_layout::GridLayout;

mod animation;

use animation::GridAnimation;

static FONT: LazyLock<Font> = LazyLock::new(|| {
    load_ttf_font_from_bytes(include_bytes!("../../../assets/JosefinSans-Bold.ttf")).unwrap()
});

const FONT_SCALING: u16 = 250;

const UNKNOWN_COLOR: Color = Color::from_rgba(0x22, 0x24, 0x26, 0xff);
const WALL_COLOR: Color = Color::from_rgba(0xaa, 0x00, 0x10, 0xff);
const DOT_COLOR: Color = Color::from_rgba(0x16, 0x9a, 0xb3, 0xff);
const BACKGROUND: Color = Color::from_rgba(0x18, 0x1a, 0x1b, 0xff);

/// grid that draws to the screen using macroquad
pub struct GameGrid<G> {
    grid: G,
    animation: GridAnimation,
}

impl<G> GameGrid<G>
where
    G: GridIter,
{
    pub fn new(grid: G) -> Self {
        let animation = GridAnimation::new();
        Self { grid, animation }
    }

    pub fn update(&mut self) {
        self.animation.tick();
    }

    pub fn draw(&self, bounds: Rect) {
        draw_grid(&self.grid, bounds);
        let layout = GridLayout::new(bounds, self.grid.width(), self.grid.height());
        self.animation.draw(&layout);
    }

    pub fn clear_animations(&mut self) {
        self.animation.clear();
    }
}

impl<G> Grid for GameGrid<G>
where
    G: Grid,
{
    fn get(&self, pos: Pos) -> Option<Tile> {
        self.grid.get(pos)
    }

    fn set(&mut self, pos: Pos, dst_tile: Tile) -> Option<Tile> {
        let src_tile = self.grid.set(pos, dst_tile)?;

        let src_color = tile_color(src_tile);
        let dst_color = tile_color(dst_tile);
        self.animation.animate(pos, src_color, dst_color);
        Some(src_tile)
    }

    fn width(&self) -> u32 {
        self.grid.width()
    }

    fn height(&self) -> u32 {
        self.grid.height()
    }
}

impl<G> DerefMut for GameGrid<G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

impl<G> Deref for GameGrid<G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

pub fn draw_grid(grid: &impl GridIter, bounds: Rect) {
    draw_rectangle(bounds.x, bounds.y, bounds.w, bounds.h, BACKGROUND);

    let w = grid.width();
    let h = grid.height();
    let layout = GridLayout::new(bounds, w, h);

    for (pos, tile) in grid.iter() {
        let color = tile_color(tile);
        let value = tile.as_value();
        draw_tile(&layout, pos, color, value);
    }
}

pub fn draw_tile(layout: &GridLayout, pos: Pos, color: Color, value: Option<u8>) {
    let center = layout.center_of(pos);
    let radius = layout.radius();
    draw_tile_circle(center, radius, color);

    let font_size = layout.font_size();
    draw_tile_value(center, font_size, value);
}

fn draw_tile_circle(center: Vec2, radius: f32, color: Color) {
    let sides = 64;
    let rotation = 0.;
    draw_poly(center.x, center.y, sides, radius, rotation, color);
}

fn draw_tile_value(center: Vec2, font_size: f32, value: Option<u8>) {
    let Some(value) = value else {
        return;
    };

    let text = format!("{value}");
    let scaling = FONT_SCALING;

    let font_scale = font_size / scaling as f32;
    let font_size = scaling;

    let dims = measure_text(&text, Some(&FONT), font_size, font_scale);
    let x = center.x - dims.width / 2.;
    let y = center.y - dims.height / 2. + dims.offset_y;
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: Some(&FONT),
            font_size,
            font_scale,
            color: WHITE,
            ..Default::default()
        },
    );
}

fn tile_color(tile: Tile) -> Color {
    match tile {
        Tile::Unknown => UNKNOWN_COLOR,
        Tile::Wall => WALL_COLOR,
        Tile::Dot(_) => DOT_COLOR,
    }
}
