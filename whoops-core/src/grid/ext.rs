use crate::grid::Grid;
use crate::offset::Offset;
use crate::pos::Pos;
use crate::tile::Tile;

pub trait GridExt {
    fn iter_pos(&self) -> impl Iterator<Item = Pos> + 'static;
    fn iter_tile(&self) -> impl Iterator<Item = Tile>;
    fn iter(&self) -> impl Iterator<Item = (Pos, Tile)>;

    fn iter_pos_offset(&self, start: Pos, offset: Offset) -> impl Iterator<Item = Pos> + 'static;
    fn iter_tile_offset(&self, start: Pos, offset: Offset) -> impl Iterator<Item = Tile>;
    fn iter_offset(&self, start: Pos, offset: Offset) -> impl Iterator<Item = (Pos, Tile)>;

    /// tries to match a different grid as a pattern, and returns `true` if they match.
    /// uses the `Tile::matches` method to compare between tiles
    fn matches(&self, pattern: &impl Grid) -> bool;
}

impl<G> GridExt for G
where
    G: Grid + ?Sized,
{
    fn iter_pos(&self) -> impl Iterator<Item = Pos> + 'static {
        GridPosIter::new(self.width(), self.height())
    }

    fn iter_tile(&self) -> impl Iterator<Item = Tile> {
        GridIter::new(self).map(|(_, tile)| tile)
    }

    fn iter(&self) -> impl Iterator<Item = (Pos, Tile)> {
        GridIter::new(self)
    }

    fn iter_pos_offset(&self, start: Pos, offset: Offset) -> impl Iterator<Item = Pos> + 'static {
        let w = self.width();
        let h = self.height();
        std::iter::successors(Some(start), move |pos| pos.add_offset(offset))
            .take_while(move |pos| pos.x < w && pos.y < h)
    }

    fn iter_tile_offset(&self, start: Pos, offset: Offset) -> impl Iterator<Item = Tile> {
        self.iter_pos_offset(start, offset)
            .flat_map(|pos| self.get(pos))
    }

    fn iter_offset(&self, start: Pos, offset: Offset) -> impl Iterator<Item = (Pos, Tile)> {
        self.iter_pos_offset(start, offset)
            .flat_map(|pos| self.get(pos).map(|tile| (pos, tile)))
    }

    fn matches(&self, pattern: &impl Grid) -> bool {
        let width = self.width();
        let height = self.height();
        if width != pattern.width() || height != pattern.height() {
            return false;
        }

        self.iter_tile()
            .zip(pattern.iter_tile())
            .all(|(value, expected)| value.matches(expected))
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

        Self {
            index,
            width,
            length,
        }
    }
}

impl Iterator for GridPosIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.length {
            return None;
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
pub struct GridIter<'a, G: ?Sized> {
    grid: &'a G,
    index: usize,
}

impl<'a, G> GridIter<'a, G>
where
    G: Grid + ?Sized,
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
    G: Grid + ?Sized,
{
    type Item = (Pos, Tile);

    fn next(&mut self) -> Option<Self::Item> {
        if self.len_remainder() == 0 {
            return None;
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

impl<'a, G> ExactSizeIterator for GridIter<'a, G> where G: Grid + ?Sized {}
