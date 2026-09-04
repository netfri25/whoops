use crate::pos::Pos;
use crate::tile::Tile;

pub mod tile_grid;
pub mod rewindable;

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

        (0..height)
            .flat_map(|y| (0..width).map(move |x| Pos { x, y }))
            .all(|pos| {
                let Some(value) = self.get(pos) else {
                    return false;
                };

                let Some(expected) = pattern.get(pos) else {
                    return false;
                };

                value.matches(expected)
            })
    }
}
