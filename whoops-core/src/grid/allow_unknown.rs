use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

use super::Grid;
use crate::grid::GridIter;
use crate::pos::Pos;
use crate::tile::Tile;

/// allows modification to unknown tiles at creation
pub struct AllowUnknown<G> {
    grid: G,
    allow: HashSet<Pos>,
}

impl<G> AllowUnknown<G>
where
    G: GridIter,
{
    pub fn new(grid: G) -> Self {
        let allow = Default::default();
        let mut this = Self { grid, allow };
        this.allow_unknown_update();
        this
    }

    pub fn into_grid(self) -> G {
        self.grid
    }

    pub fn allow_unknown_update(&mut self) {
        self.allow = self
            .grid
            .iter()
            .filter_map(|(pos, tile)| tile.is_unknown().then_some(pos))
            .collect();
    }
}

impl<G> Grid for AllowUnknown<G>
where
    G: Grid,
{
    fn get(&self, pos: Pos) -> Option<Tile> {
        self.grid.get(pos)
    }

    fn set(&mut self, pos: Pos, tile: Tile) -> Option<Tile> {
        if !self.allow.contains(&pos) {
            return None;
        }

        self.grid.set(pos, tile)
    }

    fn width(&self) -> u32 {
        self.grid.width()
    }

    fn height(&self) -> u32 {
        self.grid.height()
    }
}

impl<G> From<G> for AllowUnknown<G>
where
    G: GridIter,
{
    fn from(value: G) -> Self {
        Self::new(value)
    }
}

impl<G> DerefMut for AllowUnknown<G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

impl<G> Deref for AllowUnknown<G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}
