use crate::pos::Pos;
use crate::tile::Tile;

pub trait Grid {
    /// returns a tile at a given position.
    /// if out of bounds, returns None.
    fn get(&self, pos: Pos) -> Option<Tile>;

    /// sets a tile in a given position.
    /// returns the old tile, if in bounds.
    fn set(&mut self, pos: Pos, tile: Tile) -> Option<Tile>;

    /// returns the size of the grid
    fn size(&self) -> Size;

    /// tries to match a different grid as a pattern, and returns `true` if they match.
    /// uses the `Tile::matches` method to compare between tiles
    fn matches(&self, pattern: &dyn Grid) -> bool {
        let size = self.size();
        if size != pattern.size() {
            return false;
        }

        (0..size.height)
            .flat_map(|y| (0..size.width).map(move |x| Pos { x, y }))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum GridSetError {
    #[error("unable to set cell outside grid bounds")]
    OutOfBounds,

    #[error("unable to change a constant cell")]
    Constant,
}
