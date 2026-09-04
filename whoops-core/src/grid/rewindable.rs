use crate::pos::Pos;
use crate::tile::Tile;

use super::Grid;

#[derive(Clone)]
pub struct Rewindable<G> {
    grid: G,
    undo_left: usize,
    history: Vec<Modification>,
}

impl<G> Rewindable<G>
where
    G: Grid,
{
    pub fn new(grid: G) -> Self {
        Self {
            grid,
            undo_left: 0,
            history: Default::default(),
        }
    }

    pub fn take_grid(self) -> G {
        self.grid
    }

    /// tries to undo the last modification.
    /// on success returns true.
    /// if no undo available, returns false.
    pub fn undo(&mut self) -> bool {
        if self.undo_left == 0 {
            return false;
        }

        self.undo_left -= 1;
        let Modification { pos, old_tile, .. } = self.history[self.undo_left];

        self.grid.set(pos, old_tile);
        true
    }

    /// tries to undo the last undo to the last modification.
    /// on success returns true.
    /// if no undo to undo available, returns false.
    pub fn redo(&mut self) -> bool {
        if self.undo_left == self.history.len() {
            return false;
        }

        let Modification { pos, new_tile, .. } = self.history[self.undo_left];
        self.undo_left += 1;

        self.grid.set(pos, new_tile);
        true
    }

    fn history_add(&mut self, modification: Modification) {
        self.history.truncate(self.undo_left);
        self.history.push(modification);
        self.undo_left += 1;
    }
}

impl<G> Grid for Rewindable<G>
where
    G: Grid,
{
    fn get(&self, pos: Pos) -> Option<Tile> {
        self.grid.get(pos)
    }

    fn set(&mut self, pos: Pos, new_tile: Tile) -> Option<Tile> {
        let old_tile = self.grid.set(pos, new_tile)?;
        self.history_add(Modification { pos, old_tile, new_tile });
        Some(old_tile)
    }

    fn width(&self) -> u32 {
        self.grid.width()
    }

    fn height(&self) -> u32 {
        self.grid.height()
    }
}

impl<G> From<G> for Rewindable<G>
where
    G: Grid,
{
    fn from(value: G) -> Self {
        Self::new(value)
    }
}

impl<G> AsRef<G> for Rewindable<G> {
    fn as_ref(&self) -> &G {
        &self.grid
    }
}

impl<G> AsMut<G> for Rewindable<G> {
    fn as_mut(&mut self) -> &mut G {
        &mut self.grid
    }
}

#[derive(Clone)]
struct Modification {
    pub pos: Pos,
    pub old_tile: Tile,
    pub new_tile: Tile,
}
