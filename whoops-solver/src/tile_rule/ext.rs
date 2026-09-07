use crate::tile_rule::{FuncTileRule, Response, TileRule};

pub trait TileRuleExt<G: ?Sized>: TileRule<G> {
    fn chain<B>(self, other: B) -> impl TileRule<G>
    where
        B: TileRule<G>;
}

impl<R, G> TileRuleExt<G> for R
where
    R: TileRule<G>
{
    fn chain<B>(mut self, mut other: B) -> impl TileRule<G>
    where
        B: TileRule<G>
    {
        FuncTileRule(move |pos, grid: &mut G| {
            let response = self.solve_at(pos, grid)?;
            match response {
                Response::Keep => other.solve_at(pos, grid),
                _ => Some(response)
            }
        })
    }
}
