use crate::{GameStatus, MnkBoard, MnkGame, PlaceError, PlayError, Space};
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

    /// Attempts to play in the indicated column.
    ///
    /// If the column has any [`Space::Empty`] in it, plays at the physically lowest one (that is,
    /// the one with the highest row index).
    ///
    /// # Errors
    /// - A [`PlayError::RuleError`] if the column is full.
    /// - A [`PlayError::PlaceError`] with [`PlaceError::OutOfBounds`] and an arbitrary `row` if the
    ///   column is out of bounds.
    /// - Any errors thrown by [`MnkGame::play_at`].
    pub fn play_in(&mut self, column: usize) -> Result<(), PlayError> {
        let row = lowest_row_in(self.board(), column)?;
        self.0.play_at(row, column)
    }
}

/// Find the physically lowest (numerically greatest) row index corresponding to a [`Space::Empty`].
fn lowest_row_in<const R: usize, const C: usize, const K: usize>(
    board: &MnkBoard<R, C, K>,
    column: usize,
) -> Result<usize, PlayError> {
    if column >= C {
        return Err(PlayError::PlaceError(PlaceError::OutOfBounds {
            row: 0,
            column,
        }));
    }

    let mut rows = (0..R).rev();
    rows.find(|&r| {
        board
            .get(r, column)
            .expect("column guard should guarantee in-bounds")
            == &Space::Empty
    })
    .ok_or_else(|| PlayError::RuleError {
        message: format!("column {column} is full"),
    })
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

#[cfg(test)]
mod test_lowest_row_in {
    use super::*;
    use crate::Player;
    use std::assert_matches;

    #[test]
    fn empty_yields_r_minus_one() {
        let one_one: MnkBoard<1, 1, 1> = MnkBoard::new();
        assert_eq!(lowest_row_in(&one_one, 0), Ok(0));

        let two_one: MnkBoard<2, 1, 1> = MnkBoard::new();
        assert_eq!(lowest_row_in(&two_one, 0), Ok(1));

        let two_two: MnkBoard<2, 2, 1> = MnkBoard::new();
        assert_eq!(lowest_row_in(&two_two, 1), Ok(1));
    }

    #[test]
    fn non_empty_yields_lowest() {
        let mut two_one: MnkBoard<2, 1, 1> = MnkBoard::new();
        _ = two_one.place(Player::X, 1, 0);
        assert_eq!(lowest_row_in(&two_one, 0), Ok(0));

        let mut two_two: MnkBoard<2, 2, 1> = MnkBoard::new();
        _ = two_two.place(Player::X, 1, 1);
        assert_eq!(lowest_row_in(&two_two, 1), Ok(0));
    }

    #[test]
    fn out_of_bounds_fails() {
        let one_one: MnkBoard<1, 1, 1> = MnkBoard::new();
        assert_matches!(
            lowest_row_in(&one_one, 1),
            Err(PlayError::PlaceError(PlaceError::OutOfBounds {
                column: 1,
                row: _
            }))
        );
    }

    #[test]
    fn full_column_fails() {
        let mut one_one: MnkBoard<1, 1, 1> = MnkBoard::new();
        _ = one_one.place(Player::X, 0, 0);
        assert_matches!(
            lowest_row_in(&one_one, 0),
            Err(PlayError::RuleError { message: _ })
        );
    }
}
