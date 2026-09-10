use std::iter::FusedIterator;
use std::ops::Deref;

use crate::grid::Grid;
use crate::offset::Offset;
use crate::pos::Pos;
use crate::tile::Tile;

pub trait GridIter: Grid {
    #[inline(always)]
    fn iter_pos(&self) -> impl Iterator<Item = Pos> + 'static {
        GridPosIter::new(self.width(), self.height())
    }

    #[inline(always)]
    fn iter_tile(&self) -> impl Iterator<Item = Tile> {
        self.iter_pos().filter_map(|pos| self.get(pos))
    }

    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = (Pos, Tile)> {
        self.iter_pos()
            .filter_map(|pos| Some(pos).zip(self.get(pos)))
    }

    #[inline(always)]
    fn iter_pos_offset(&self, start: Pos, offset: Offset) -> impl Iterator<Item = Pos> + 'static {
        let w = self.width();
        let h = self.height();
        std::iter::successors(Some(start), move |pos| pos.add_offset(offset))
            .take_while(move |pos| pos.x < w && pos.y < h)
    }

    #[inline(always)]
    fn iter_tile_offset(&self, start: Pos, offset: Offset) -> impl Iterator<Item = Tile> {
        self.iter_pos_offset(start, offset)
            .filter_map(|pos| self.get(pos))
    }

    #[inline(always)]
    fn iter_offset(&self, start: Pos, offset: Offset) -> impl Iterator<Item = (Pos, Tile)> {
        self.iter_pos_offset(start, offset)
            .flat_map(|pos| Some(pos).zip(self.get(pos)))
    }
}

impl<D> GridIter for D
where
    D: Deref + Grid,
    <D as Deref>::Target: GridIter,
{
    fn iter_pos(&self) -> impl Iterator<Item = Pos> + 'static {
        self.deref().iter_pos()
    }

    fn iter_tile(&self) -> impl Iterator<Item = Tile> {
        self.deref().iter_tile()
    }

    fn iter(&self) -> impl Iterator<Item = (Pos, Tile)> {
        self.deref().iter()
    }

    fn iter_pos_offset(&self, start: Pos, offset: Offset) -> impl Iterator<Item = Pos> + 'static {
        self.deref().iter_pos_offset(start, offset)
    }

    fn iter_tile_offset(&self, start: Pos, offset: Offset) -> impl Iterator<Item = Tile> {
        self.deref().iter_tile_offset(start, offset)
    }

    fn iter_offset(&self, start: Pos, offset: Offset) -> impl Iterator<Item = (Pos, Tile)> {
        self.deref().iter_offset(start, offset)
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
impl FusedIterator for GridPosIter {}
