use ::rand::prelude::Rng;
use macroquad::prelude::*;
use whoops_core::grid::{AllowUnknown, Grid, History, Lazy, TileGrid};

mod grid;
use grid::GameGrid;
use whoops_core::pos::Pos;
use whoops_core::tile::Tile;
use whoops_generator::random::{self, RandomGenerator};
use whoops_generator::{Generator, GeneratorOutput};
use whoops_solver::{
    AssertFull, AssertValid, Solver, SolverExt, UpdateAllValues, default_solver_checked,
};

use crate::grid_layout::GridLayout;

// TODO:
//  Rust warns me that this is a complex type, so I put it as a type alias.
//  should I break this up to traits? like the Monad Transformers typeclasses in Haskell

type OhnO = AllowUnknown<Lazy<History<GameGrid<TileGrid>>>>;

pub struct Game<R> {
    grid: OhnO,
    rng: R,
}

impl<R> Game<R>
where
    R: Rng,
{
    pub fn new(rng: R, grid: TileGrid) -> Self {
        let grid = construct_an_abomination_of_a_grid(grid);
        Self { grid, rng }
    }

    pub fn from_rng(mut rng: R, width: u32, height: u32) -> Self {
        let output = generate(&mut rng, width, height);
        let grid = construct_an_abomination_of_a_grid(output.grid);
        Self { grid, rng }
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
        let mut solver = AssertFull.and_then(AssertValid).and_then(UpdateAllValues);
        let grid: &mut TileGrid = &mut self.grid;

        let ok = solver.solve(grid).is_ok();
        drop(solver);

        if ok {
            self.grid.allow_unknown_update();
            self.grid.clear_animations();
            self.grid.take_history(); // remove history
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
            self.lazy_step();
            self.fill_with_values_if_finished();
        }

        if is_key_pressed(KeyCode::T) {
            self.grid.toggle_is_lazy();
        }

        if is_key_pressed(KeyCode::C) {
            self.complete_grid();
            self.fill_with_values_if_finished();
        }

        let keys = [
            (KeyCode::Key3, 3, 3),
            (KeyCode::Key4, 4, 4),
            (KeyCode::Key5, 5, 5),
            (KeyCode::Key6, 6, 6),
            (KeyCode::Key7, 7, 7),
            (KeyCode::Key8, 8, 8),
            (KeyCode::Key9, 9, 9),
            (KeyCode::Key0, 10, 10),
        ];

        for (key, w, h) in keys {
            if is_key_pressed(key) {
                self.regenerate(w, h)
            }
        }
    }

    fn regenerate(&mut self, width: u32, height: u32) {
        let output = generate(&mut self.rng, width, height);
        self.grid = construct_an_abomination_of_a_grid(output.grid);
    }

    fn lazy_step(&mut self) {
        if self.grid.is_lazy_empty() {
            self.solve();
        }

        self.grid.lazy_step();
    }

    fn complete_grid(&mut self) {
        self.solve();
        self.grid.lazy_flush();
    }

    /// solve the grid and add the steps to the solution into the lazy grid
    fn solve(&mut self) {
        let grid: TileGrid = self.grid.clone();

        let mut solver = default_solver_checked();
        let start = std::time::Instant::now();
        let solution = solver.solve(History::new(grid));
        let elapsed = start.elapsed();
        eprintln!("solving took {:?}", elapsed);

        let mut solved_grid = match solution {
            Ok(grid) | Err(grid) => grid,
        };

        let history = solved_grid.take_history();
        self.grid.lazy_extend(history.into_iter().map(Into::into));
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

fn generate(rng: impl Rng, width: u32, height: u32) -> GeneratorOutput<TileGrid> {
    let params = random::Params::new(width, height);
    let generator = RandomGenerator::new(rng, params);

    let start = std::time::Instant::now();
    let output = generator.generate();
    let elapsed = start.elapsed();
    eprintln!("generation took {:?}", elapsed);

    output
}

#[allow(clippy::let_and_return)]
fn construct_an_abomination_of_a_grid(grid: TileGrid) -> OhnO {
    let grid = GameGrid::new(grid);
    let grid = History::new(grid);
    let grid = Lazy::new(grid, false, [].into());
    let grid = AllowUnknown::new(grid);
    grid
}
