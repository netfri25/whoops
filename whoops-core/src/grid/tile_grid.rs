use super::Grid;

use crate::pos::Pos;
use crate::tile::Tile;

// invariant: tiles.len() % width == 0
#[derive(Clone)]
pub struct TileGrid {
    tiles: Vec<Tile>,
    width: u32,
}

impl TileGrid {
    pub fn from_parts(tiles: impl IntoIterator<Item = Tile>, width: u32) -> Option<Self> {
        let tiles: Vec<_> = tiles.into_iter().collect();
        if tiles.len() % width as usize != 0 {
            return None;
        }

        Some(Self { tiles, width })
    }

    fn index_of(&self, pos: Pos) -> Option<usize> {
        if pos.x >= self.width() || pos.y >= self.height() {
            return None
        }

        let index = pos.x + pos.y / self.width();
        Some(index as usize)
    }

    fn at(&self, index: usize) -> Option<Tile> {
        self.tiles.get(index).copied()
    }

    fn at_mut(&mut self, index: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(index)
    }
}

impl Grid for TileGrid {
    fn get(&self, pos: Pos) -> Option<Tile> {
        let index = self.index_of(pos)?;
        self.at(index)
    }

    fn set(&mut self, pos: Pos, tile: Tile) -> Option<Tile> {
        let index = self.index_of(pos)?;
        let target = self.at_mut(index)?;
        let old = std::mem::replace(target, tile);
        Some(old)
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.tiles.len() as u32 / self.width()
    }
}
