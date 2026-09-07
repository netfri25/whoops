use macroquad::prelude::*;
use whoops_core::grid::{AllowUnknown, Grid, History, Lazy, TileGrid};

mod grid;
use grid::GameGrid;
use whoops_core::pos::Pos;
use whoops_core::tile::Tile;
use whoops_solver::{
    AssertFull, AssertValid, FillDotsWithValues, Solver, SolverExt, default_solver_checked,
};

use crate::grid_layout::GridLayout;

// TODO:
//  Rust warns me that this is a complex type, so I put it as a type alias.
//  should I break this up to traits? like the Monad Transformers typeclasses in Haskell

pub struct Game {
    grid: AllowUnknown<Lazy<History<GameGrid<TileGrid>>>>,
}

impl Game {
    pub fn new(grid: TileGrid) -> Self {
        let grid = GameGrid::new(grid);
        let grid = History::new(grid);
        let grid = Lazy::new(grid, false, [].into());
        let grid = AllowUnknown::new(grid);
        Self { grid }
    }

    pub fn update(&mut self, bounds: Rect) {
        self.grid.update();
        self.handle_mouse_input(bounds);
        self.handle_keyboard_input();
        self.fill_with_values_if_finished();
    }

    pub fn draw(&self, bounds: Rect) {
        self.grid.draw(bounds);
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
        self.update_tile(pos, update_fn);
        self.fill_with_values_if_finished();
    }

    fn update_tile(&mut self, pos: Pos, update_fn: impl FnOnce(Tile) -> Tile) {
        let src_tile = self.grid.get(pos).unwrap();
        let dst_tile = update_fn(src_tile);
        self.grid.set(pos, dst_tile);
    }

    fn fill_with_values_if_finished(&mut self) {
        let mut solver = AssertFull
            .and_then(AssertValid)
            .and_then(FillDotsWithValues);
        let grid: &mut TileGrid = &mut self.grid;

        let ok = solver.solve(grid).is_ok();
        drop(solver);

        if ok {
            self.grid.allow_unknown_update();
            self.grid.clear_animations();
        }
    }

    fn handle_keyboard_input(&mut self) {
        if is_key_pressed(KeyCode::U) {
            if is_key_down(KeyCode::LeftShift) {
                self.grid.redo();
            } else {
                self.grid.undo();
            }
        }

        if is_key_pressed(KeyCode::S) {
            self.grid.lazy_step();
            self.fill_with_values_if_finished();
        }

        if is_key_pressed(KeyCode::T) {
            self.grid.toggle_is_lazy();
        }

        if is_key_pressed(KeyCode::C) {
            self.solve();
        }
    }

    fn solve(&mut self) {
        let grid: TileGrid = self.grid.clone();

        let mut solver = default_solver_checked().and_then(FillDotsWithValues);
        let start = std::time::Instant::now();
        let solution = solver.solve(History::new(grid));
        let elapsed = start.elapsed();
        eprintln!("solving took {:?}", elapsed);

        let mut solved_grid = match solution {
            Ok(grid) | Err(grid) => grid,
        };

        let history = solved_grid.take_history();
        self.grid.lazy_extend(history.into_iter().map(Into::into));
        self.grid.lazy_flush();
        self.fill_with_values_if_finished();
    }
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
