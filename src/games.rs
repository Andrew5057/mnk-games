use crate::board::{MnkBoard, Player};

/// A standard [*m,n,k*-game].
///
/// [`Player::X`] and [`Player::O`] alternate placing stones, in that order, on a board with `R`
/// rows and `C` columns until one gets `K` stones in a row.
///
/// [*m,n,k*-game]: https://en.wikipedia.org/wiki/M,n,k-game
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MnkGame<const R: usize, const C: usize, const K: usize> {
    board: MnkBoard<R, C, K>,
    current_player: Player,
    winner: Option<Player>,
}

impl<const R: usize, const C: usize, const K: usize> MnkGame<R, C, K> {
    /// Returns an `MnkGame<R, C, K>` with an empty board and current player [`Player::X`].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            board: MnkBoard::<R, C, K>::new(),
            current_player: Player::X,
            winner: None,
        }
    }
}

impl<const R: usize, const C: usize, const K: usize> Default for MnkGame<R, C, K> {
    fn default() -> Self {
        Self::new()
    }
}
