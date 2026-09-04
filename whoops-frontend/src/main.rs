use std::sync::LazyLock;

use macroquad::prelude::*;

use whoops_core::grid::Grid;
use whoops_core::grid::rewindable::Rewindable;
use whoops_core::grid::tile_grid::TileGrid;
use whoops_core::pos::Pos;
use whoops_core::tile::Tile;
use whoops_core::tile_grid;

use crate::grid_layout::GridLayout;

mod grid_layout;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 640;

const UNKNOWN_COLOR: Color = Color::from_rgba(0x22, 0x24, 0x26, 0xff);
const WALL_COLOR: Color = Color::from_rgba(0xaa, 0x00, 0x10, 0xff);
const DOT_COLOR: Color = Color::from_rgba(0x16, 0x9a, 0xb3, 0xff);
const BACKGROUND: Color = Color::from_rgba(0x18, 0x1a, 0x1b, 0xff);

static FONT: LazyLock<Font> = LazyLock::new(|| {
    load_ttf_font_from_bytes(include_bytes!("../../assets/JosefinSans-Bold.ttf")).unwrap()
});

fn window_conf() -> Conf {
    Conf {
        window_title: "whoops".into(),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: true,
        sample_count: 4,
        platform: miniquad::conf::Platform {
            swap_interval: Some(1),
            linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    #[rustfmt::skip]
    let grid = tile_grid![
        [3, 2, _, _],
        [_, _, x, _],
        [_, 3, _, _],
        [_, 4, _, 4],
    ];

    let app = App::default();

    loop {
        clear_background(BLACK);
        let bounds = get_window_bounds();
        app.draw(bounds);
        draw_fps();
        next_frame().await
    }
}

#[derive(Default)]
pub struct App {
    game: Option<Game>,
}

impl App {
    pub fn draw(&self, bounds: Rect) {
        if let Some(ref game) = self.game {
            game.draw(bounds)
        }
    }
}

pub struct Game {
    grid: Rewindable<TileGrid>,
}

impl Game {
    pub fn new(grid: impl Into<Rewindable<TileGrid>>) -> Self {
        let grid = grid.into();
        Self { grid }
    }

    pub fn draw(&self, bounds: Rect) {
        draw_rectangle(bounds.x, bounds.y, bounds.w, bounds.h, BACKGROUND);

        let w = self.grid.width();
        let h = self.grid.height();
        let layout = GridLayout::new(bounds, w, h);

        for y in 0..h {
            for x in 0..w {
                let pos = Pos::new(x, y);
                self.draw_tile(&layout, pos);
            }
        }
    }

    fn draw_tile(&self, layout: &GridLayout, pos: Pos) {
        let Some(tile) = self.grid.get(pos) else {
            return;
        };

        let center = layout.center_of(pos);
        let radius = layout.radius();
        self.draw_tile_circle(center, radius, tile);

        let font_size = layout.font_size();
        self.draw_tile_text(center, font_size, tile);
    }

    fn draw_tile_circle(&self, center: Vec2, radius: f32, tile: Tile) {
        let color = match tile {
            Tile::Unknown => UNKNOWN_COLOR,
            Tile::Wall => WALL_COLOR,
            Tile::Dot(_) => DOT_COLOR,
        };

        let sides = 255;
        let rotation = 0.;
        draw_poly(center.x, center.y, sides, radius, rotation, color);
    }

    fn draw_tile_text(&self, center: Vec2, font_size: f32, tile: Tile) {
        let Some(value) = tile.as_value() else {
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
}

fn get_window_bounds() -> Rect {
    let w = screen_width();
    let h = screen_height();
    Rect::new(0., 0., w, h)
}
