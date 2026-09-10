use crate::grid::GridIter;

pub trait GridExt: GridIter {
    /// tries to match a different grid as a pattern, and returns `true` if they match.
    /// uses the `Tile::matches` method to compare between tiles
    fn matches(&self, pattern: &impl GridExt) -> bool {
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

impl<G> GridExt for G where G: GridIter + ?Sized {}

