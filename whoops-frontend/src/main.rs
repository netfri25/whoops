use macroquad::prelude::*;

use ::rand::prelude::ThreadRng;

use crate::app::App;
use crate::game::Game;

pub mod app;
pub mod game;
pub mod grid_layout;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 640;

fn window_conf() -> macroquad::conf::Conf {
    let miniquad_conf = Conf {
        window_title: "whoops".into(),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: true,
        sample_count: 4,
        platform: miniquad::conf::Platform {
            swap_interval: Some(1),
            linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
            blocking_event_loop: true,
            ..Default::default()
        },
        ..Default::default()
    };

    macroquad::conf::Conf {
        miniquad_conf,
        update_on: Some(macroquad::conf::UpdateTrigger {
            key_down: true,
            mouse_down: true,
            mouse_up: true,
            mouse_motion: true,
            mouse_wheel: true,
            specific_key: None,
            touch: false,
        }),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let rng = ThreadRng::default();
    let game = Game::from_rng(rng, 6, 6);
    let mut app = App::new(Some(game));

    loop {
        clear_background(BLACK);
        let bounds = get_window_bounds();
        app.update(bounds);
        app.draw(bounds);
        next_frame().await
    }
}

fn get_window_bounds() -> Rect {
    let w = screen_width();
    let h = screen_height();
    Rect::new(0., 0., w, h)
}
