use crate::pos::Pos;
use crate::tile::Tile;

mod ext;
pub use ext::*;

pub mod history;
pub mod konst;
pub mod lazy;
pub mod tile_grid;

pub use history::History;
pub use konst::Konst;
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

    /// tries to match a different grid as a pattern, and returns `true` if they match.
    /// uses the `Tile::matches` method to compare between tiles
    fn matches(&self, pattern: &dyn Grid) -> bool {
        let width = self.width();
        let height = self.height();
        if width != pattern.width() || height != pattern.height() {
            return false;
        }

        self.iter_tile()
            .zip(pattern.iter_tile())
            .all(|(value, expected)| value.matches(expected))
    }

    fn is_full(&self) -> bool {
        !self.iter_tile().any(|tile| tile.is_unknown())
    }
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
