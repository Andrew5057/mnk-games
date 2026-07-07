use crate::MnkGame;

/// A restricted [`MnkGame`] where stones must be placed at the bottom of a column.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GravityGame<const R: usize, const C: usize, const K: usize>(MnkGame<R, C, K>);
