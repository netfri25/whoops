use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

use super::{Grid, GridExt};
use crate::pos::Pos;
use crate::tile::Tile;

/// disallows modification to known tiles at creation
pub struct Konst<G> {
    grid: G,
    preserve: HashSet<Pos>,
}

impl<G> Konst<G>
where
    G: Grid,
{
    pub fn new(grid: G) -> Self {
        let preserve = grid
            .iter()
            .filter_map(|(pos, tile)| (!tile.is_unknown()).then_some(pos))
            .collect();

        Self { grid, preserve }
    }
}

impl<G> Grid for Konst<G>
where
    G: Grid,
{
    fn get(&self, pos: Pos) -> Option<Tile> {
        self.grid.get(pos)
    }

    fn set(&mut self, pos: Pos, tile: Tile) -> Option<Tile> {
        if self.preserve.contains(&pos) {
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

impl<G> From<G> for Konst<G>
where
    G: Grid,
{
    fn from(value: G) -> Self {
        Self::new(value)
    }
}

impl<G> DerefMut for Konst<G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

impl<G> Deref for Konst<G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}
