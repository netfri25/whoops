use crate::pos::Pos;
use crate::tile::Tile;

pub mod tile_grid;
pub mod rewindable;
pub mod konst;

pub use tile_grid::TileGrid;
pub use rewindable::Rewindable;
pub use konst::Konst;

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

    fn iter_pos(&self) -> GridPosIter {
        GridPosIter::new(self.width(), self.height())
    }

    fn iter(&self) -> GridIter<'_, Self>
    where
        Self: Sized,
    {
        GridIter::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct GridPosIter {
    index: u32,
    width: u32,
    length: u32,
}

impl GridPosIter {
    pub fn new(width: u32, height: u32) -> Self {
        let index = 0;
        let length = width * height;

        Self { index, width, length }
    }
}

impl Iterator for GridPosIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.length {
            return None
        }

        let index = self.index;
        self.index += 1;

        let w = self.width;
        let x = index % w;
        let y = index / w;
        let pos = Pos::new(x, y);
        Some(pos)
    }

    fn count(self) -> usize {
        self.length as usize
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.length as usize;
        (size, Some(size))
    }
}

impl ExactSizeIterator for GridPosIter {}

#[derive(Debug, Clone)]
pub struct GridIter<'a, G> {
    grid: &'a G,
    index: usize,
}

impl<'a, G> GridIter<'a, G>
where
    G: Grid
{
    pub fn new(grid: &'a G) -> Self {
        let index = 0;
        Self { grid, index }
    }

    fn len_remainder(&self) -> usize {
        let w = self.grid.width();
        let h = self.grid.height();
        let size = w as usize * h as usize;
        size.saturating_sub(self.index)
    }
}

impl<'a, G> Iterator for GridIter<'a, G>
where
    G: Grid
{
    type Item = (Pos, Tile);

    fn next(&mut self) -> Option<Self::Item> {
        if self.len_remainder() == 0 {
            return None
        }

        let index = self.index as u32;
        self.index += 1;

        let w = self.grid.width();
        let x = index % w;
        let y = index / w;
        let pos = Pos::new(x, y);
        let tile = self.grid.get(pos)?;
        Some((pos, tile))
    }

    fn count(self) -> usize {
        self.len_remainder()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len_remainder();
        (len, Some(len))
    }
}

impl<'a, G> ExactSizeIterator for GridIter<'a, G>
where
    G: Grid
{}
