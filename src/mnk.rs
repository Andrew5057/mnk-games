use std::fmt;

/// One of two players.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Player {
    /// The player who makes the first move.
    X,
    /// The player who makes the second move.
    O,
}

impl fmt::Display for Player {
    /// Writes "X" for [`Player::X`] and "O" for [`Player::O`].
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

/// A space that can be played on by a [`Player`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Space {
    /// A space that has not been played on yet.
    #[default]
    Empty,
    /// A space that has been taken by the indicated [`Player`].
    Stone(Player),
}

impl fmt::Display for Space {
    /// Writes a space for [`Space::Empty`] and the player for a [`Space::Stone`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Space::Empty => write!(f, " "),
            Space::Stone(player) => write!(f, "{}", player),
        }
    }
}

impl From<Option<Player>> for Space {
    /// Maps [`None`] and [`Some`] to [`Space::Empty`] and [`Space::Stone`], respectively.
    fn from(player: Option<Player>) -> Self {
        match player {
            None => Space::Empty,
            Some(player) => Space::Stone(player),
        }
    }
}

impl From<Space> for Option<Player> {
    /// Maps [`Space::Empty`] and [`Space::Stone`] to [`None`] and [`Some`],
    /// respectively.
    fn from(space: Space) -> Option<Player> {
        match space {
            Space::Empty => None,
            Space::Stone(player) => Some(player),
        }
    }
}

/// An [*m,n,k*-game].
///
/// *M,n,k*-games are two-player games played on an *m*-by-*n* board. Each [`Player`] takes turns
/// placing a [`Stone`][Space::Stone] in a free [`Space`] on the board. A player wins when they have
/// placed *k* consecutive stones across a row, column, or diagonal. The game is drawn if there are
/// no free spaces and neither player has won.
///
/// Although *m,n,k*-games do not necessarily have a meaningful notion of orientation, an
/// `MnkBoard<R, C, K>` has `R` rows and `C` columns, with a winner declared after a player has `K`
/// stones in a row. Another type can redefine this orientation if such an inversion is more
/// convenient.
///
/// [*m,n,k*-game]: https://en.wikipedia.org/wiki/M,n,k-game
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MnkBoard<const R: usize, const C: usize, const K: usize> {
    row_array: [[Space; C]; R],
}

impl<const R: usize, const C: usize, const K: usize> MnkBoard<R, C, K> {
    /// Returns a board filled with [`Space::Empty`].
    pub fn new() -> Self {
        Self {
            row_array: [[Space::Empty; C]; R],
        }
    }
}

impl<const R: usize, const C: usize, const K: usize> Default for MnkBoard<R, C, K> {
    /// Returns a board filled with [`Space::Empty`].
    fn default() -> Self {
        Self::new()
    }
}

impl<const R: usize, const C: usize, const K: usize> From<[[Space; C]; R]> for MnkBoard<R, C, K> {
    /// Converts a row-major array into an `MnkBoard`.
    fn from(rows: [[Space; C]; R]) -> Self {
        Self { row_array: rows }
    }
}

impl<const R: usize, const C: usize, const K: usize> From<MnkBoard<R, C, K>> for [[Space; C]; R] {
    /// Converts an `MnkBoard` into a row-major array.
    fn from(game: MnkBoard<R, C, K>) -> Self {
        game.row_array
    }
}
