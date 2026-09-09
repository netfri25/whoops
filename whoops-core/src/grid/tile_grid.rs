use std::fmt;

use super::Grid;

use crate::pos::Pos;
use crate::tile::Tile;

/// invariant: tiles.len() % width == 0
/// simple grid implementation that keeps tiles as a 1-dim array of Tile
#[derive(Clone, PartialEq, Eq)]
pub struct TileGrid {
    tiles: Box<[Tile]>,
    width: u32,
}

impl TileGrid {
    pub fn from_parts(tiles: impl IntoIterator<Item = Tile>, width: u32) -> Option<Self> {
        let tiles: Box<[Tile]> = tiles.into_iter().collect();

        if !tiles.len().is_multiple_of(width as usize) {
            return None;
        }

        Some(Self { tiles, width })
    }

    fn index_of(&self, pos: Pos) -> Option<usize> {
        if pos.x >= self.width() || pos.y >= self.height() {
            return None;
        }

        let index = pos.x + pos.y * self.width();
        Some(index as usize)
    }

    fn at(&self, index: usize) -> Option<Tile> {
        self.tiles.get(index).copied()
    }

    fn at_mut(&mut self, index: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(index)
    }
}

impl fmt::Debug for TileGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TileGrid(")?;

        for chunk in self.tiles.chunks(self.width as usize) {
            writeln!(f, "{:?}", chunk)?;
        }

        writeln!(f, ")")
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

impl<const W: usize, const H: usize> From<[[Tile; W]; H]> for TileGrid {
    fn from(value: [[Tile; W]; H]) -> Self {
        let tiles = value.as_flattened().into();
        let width = W as u32;
        Self { tiles, width }
    }
}

#[macro_export]
macro_rules! tile_grid {
    ( $( [ $( $elem:tt ),* ] ),* $(,)? ) => {
        $crate::grid::tile_grid::TileGrid::from([
            $([
                $( $crate::tile_grid!(@tile $elem) ),*
            ]),*
        ])
    };

    // Match special tokens
    (@tile x) => { $crate::tile::Tile::Wall };
    (@tile o) => { $crate::tile::Tile::Dot(0) };
    (@tile 0) => { $crate::tile::Tile::Dot(0) };
    (@tile _) => { $crate::tile::Tile::Unknown };

    // Default: assume numeric literals
    (@tile $n:literal) => { $crate::tile::Tile::Dot($n) };
}
