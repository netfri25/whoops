use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};

use super::Grid;
use crate::pos::Pos;
use crate::tile::Tile;

/// allows to delay modifications to the grid, and apply them later using the `.step()` method.
/// delay happens only when calling the `lazy_set` method. the rest of the grid methods are simply
/// delegated to the inner grid
pub struct Lazy<G> {
    grid: G,
    steps: VecDeque<Step>,
}

impl<G> Lazy<G>
where
    G: Grid,
{
    pub fn new(grid: G) -> Self {
        Self::from_steps(grid, [].into())
    }

    pub fn from_steps(grid: G, steps: VecDeque<Step>) -> Self {
        Self { grid, steps }
    }

    pub fn into_grid(self) -> G {
        self.grid
    }

    pub fn lazy_is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn lazy_step(&mut self) {
        if let Some(step) = self.steps.pop_front() {
            self.apply_step(step)
        }
    }

    pub fn lazy_flush(&mut self) {
        while let Some(step) = self.steps.pop_front() {
            self.apply_step(step)
        }
    }

    pub fn lazy_clear(&mut self) {
        self.steps.clear();
    }

    pub fn lazy_set(&mut self, pos: Pos, tile: Tile) {
        self.steps.push_back(Step { pos, tile })
    }

    pub fn lazy_extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Step>,
    {
        self.steps.extend(iter)
    }

    fn apply_step(&mut self, Step { pos, tile }: Step) {
        self.grid.set(pos, tile);
    }
}

impl<G> Grid for Lazy<G>
where
    G: Grid,
{
    fn get(&self, pos: Pos) -> Option<Tile> {
        self.grid.get(pos)
    }

    fn set(&mut self, pos: Pos, tile: Tile) -> Option<Tile> {
        self.grid.set(pos, tile)
    }

    fn width(&self) -> u32 {
        self.grid.width()
    }

    fn height(&self) -> u32 {
        self.grid.height()
    }
}

impl<G> From<G> for Lazy<G>
where
    G: Grid,
{
    fn from(value: G) -> Self {
        Self::new(value)
    }
}

impl<G> DerefMut for Lazy<G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

impl<G> Deref for Lazy<G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

pub struct Step {
    pub pos: Pos,
    pub tile: Tile,
}

impl From<super::history::Modification> for Step {
    fn from(value: super::history::Modification) -> Self {
        Self {
            pos: value.pos,
            tile: value.new_tile,
        }
    }
}

impl<G> From<super::history::History<G>> for Lazy<G>
where
    G: Grid,
{
    fn from(mut value: super::history::History<G>) -> Self {
        let history = value.take_history();
        let steps = history.into_iter().map(Into::into).collect();
        let grid = value.into_grid();
        Self::from_steps(grid, steps)
    }
}
