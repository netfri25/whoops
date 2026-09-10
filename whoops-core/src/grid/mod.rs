use crate::pos::Pos;
use crate::tile::Tile;

mod ext;
mod iter;
pub use ext::*;
pub use iter::*;

pub mod allow_unknown;
pub mod history;
pub mod lazy;
pub mod tile_grid;

pub use allow_unknown::AllowUnknown;
pub use history::History;
pub use lazy::Lazy;
pub use tile_grid::TileGrid;

pub trait Grid {
    /// returns a tile at a given position.
    /// if out of bounds, returns None.
    fn get(&self, pos: Pos) -> Option<Tile>;

    /// sets a tile in a given position.
    /// returns the old tile, if in bounds.
    fn set(&mut self, pos: Pos, tile: Tile) -> Option<Tile>;

    /// returns the width of the grid
    fn width(&self) -> u32;

    /// returns the height of the grid
    fn height(&self) -> u32;
}

impl<G> Grid for Box<G>
where
    G: Grid + ?Sized,
{
    fn get(&self, pos: Pos) -> Option<Tile> {
        self.as_ref().get(pos)
    }

    fn set(&mut self, pos: Pos, tile: Tile) -> Option<Tile> {
        self.as_mut().set(pos, tile)
    }

    fn width(&self) -> u32 {
        self.as_ref().width()
    }

    fn height(&self) -> u32 {
        self.as_ref().height()
    }
}

impl<G> Grid for &mut G
where
    G: Grid + ?Sized,
{
    fn get(&self, pos: Pos) -> Option<Tile> {
        (**self).get(pos)
    }

    fn set(&mut self, pos: Pos, tile: Tile) -> Option<Tile> {
        (**self).set(pos, tile)
    }

    fn width(&self) -> u32 {
        (**self).width()
    }

    fn height(&self) -> u32 {
        (**self).height()
    }
}
