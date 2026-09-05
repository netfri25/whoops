use std::sync::LazyLock;

use macroquad::prelude::*;

use whoops_core::grid::{Grid, Konst, Rewindable, TileGrid, iter_grid};
use whoops_core::pos::Pos;
use whoops_core::tile::Tile;

use crate::game::grid_animation::GridAnimation;
use crate::grid_layout::GridLayout;

mod grid_animation;

static FONT: LazyLock<Font> = LazyLock::new(|| {
    load_ttf_font_from_bytes(include_bytes!("../../../assets/JosefinSans-Bold.ttf")).unwrap()
});

const UNKNOWN_COLOR: Color = Color::from_rgba(0x22, 0x24, 0x26, 0xff);
const WALL_COLOR: Color = Color::from_rgba(0xaa, 0x00, 0x10, 0xff);
const DOT_COLOR: Color = Color::from_rgba(0x16, 0x9a, 0xb3, 0xff);
const BACKGROUND: Color = Color::from_rgba(0x18, 0x1a, 0x1b, 0xff);

pub struct Game {
    grid: Konst<Rewindable<TileGrid>>,
    animation: GridAnimation,
}

impl Game {
    pub fn new(grid: TileGrid) -> Self {
        let grid = Konst::new(Rewindable::new(grid));
        let animation = GridAnimation::new();
        Self { grid, animation }
    }

    pub fn update(&mut self, bounds: Rect) {
        self.animation.tick();
        self.handle_mouse_input(bounds);
    }

    pub fn draw(&self, bounds: Rect) {
        draw_grid(&self.grid, bounds);
        let layout = GridLayout::new(bounds, self.grid.width(), self.grid.height());
        self.animation.draw(&layout);
    }

    pub fn set_tile(&mut self, pos: Pos, dst_tile: Tile) {
        let Some(src_tile) = self.grid.set(pos, dst_tile) else {
            return;
        };

        let src_color = tile_color(src_tile);
        let dst_color = tile_color(dst_tile);
        self.animation.animate(pos, src_color, dst_color);
    }

    pub fn update_tile(&mut self, pos: Pos, update_fn: impl FnOnce(Tile) -> Tile) {
        let src_tile = self.grid.get(pos).unwrap();
        let dst_tile = update_fn(src_tile);
        self.set_tile(pos, dst_tile)
    }

    fn handle_mouse_input(&mut self, bounds: Rect) {
        let update_fn = match () {
            _ if is_mouse_button_pressed(MouseButton::Left) => next_tile,
            _ if is_mouse_button_pressed(MouseButton::Right) => prev_tile,
            _ => return,
        };

        let layout = GridLayout::new(bounds, self.grid.width(), self.grid.height());
        let point = mouse_position().into();
        let Some(pos) = layout.tile_at(point) else {
            return;
        };

        // NOTE: if we want to allow only pressing on the circle itself, we can add here a condition
        // that checks the mouse position (`point`) against `layout.center_of(pos)`
        self.update_tile(pos, update_fn)
    }
}

pub fn draw_grid(grid: &dyn Grid, bounds: Rect) {
    draw_rectangle(bounds.x, bounds.y, bounds.w, bounds.h, BACKGROUND);

    let w = grid.width();
    let h = grid.height();
    let layout = GridLayout::new(bounds, w, h);

    for (pos, tile) in iter_grid(grid) {
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
    let scaling = 100;

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

fn next_tile(tile: Tile) -> Tile {
    match tile {
        Tile::Unknown => Tile::Dot(0),
        Tile::Wall => Tile::Unknown,
        Tile::Dot(_) => Tile::Wall,
    }
}

fn prev_tile(tile: Tile) -> Tile {
    match tile {
        Tile::Unknown => Tile::Wall,
        Tile::Wall => Tile::Dot(0),
        Tile::Dot(_) => Tile::Unknown,
    }
}

fn tile_color(tile: Tile) -> Color {
    match tile {
        Tile::Unknown => UNKNOWN_COLOR,
        Tile::Wall => WALL_COLOR,
        Tile::Dot(_) => DOT_COLOR,
    }
}
