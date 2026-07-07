use crate::{GameStatus, MnkBoard, MnkGame};
use std::fmt;

/// A restricted [`MnkGame`] where stones must be placed at the bottom of a column.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GravityGame<const R: usize, const C: usize, const K: usize>(MnkGame<R, C, K>);

impl<const R: usize, const C: usize, const K: usize> GravityGame<R, C, K> {
    /// Returns a [`GameStatus::Ongoing`] `GravityGame<R, C, K>` with an empty board and current
    /// player [`Player::X`].
    #[must_use]
    pub const fn new() -> Self {
        Self(MnkGame::new())
    }

    /// The current state of the game's [`MnkBoard`].
    #[must_use]
    pub const fn board(&self) -> &MnkBoard<R, C, K> {
        self.0.board()
    }

    /// The current [`GameStatus`] of the game.
    #[must_use]
    pub const fn status(&self) -> GameStatus {
        self.0.status()
    }
}

impl<const R: usize, const C: usize, const K: usize> Default for GravityGame<R, C, K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const R: usize, const C: usize, const K: usize> fmt::Display for GravityGame<R, C, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
