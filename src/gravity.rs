//! [`MnkGame`]s with gravity, which restricts vertical stone placement.

use crate::{GameStatus, MnkBoard, MnkGame, OutOfBounds, PlaceError, PlayError};
use std::fmt;

/// A restricted [`MnkGame`] where stones must be placed at the bottom of a column.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GravityGame<const R: usize, const C: usize, const K: usize>(MnkGame<R, C, K>);

impl<const R: usize, const C: usize, const K: usize> GravityGame<R, C, K> {
    /// Returns a [`GameStatus::Ongoing`] `GravityGame<R, C, K>` with an empty board and current
    /// player X.
    #[must_use]
    pub const fn new() -> Self {
        Self(MnkGame::new())
    }

    /// The current state of the game's [`MnkBoard`].
    #[must_use]
    pub const fn board(&self) -> &MnkBoard<R, C> {
        self.0.board()
    }

    /// The current [`GameStatus`] of the game.
    #[must_use]
    pub const fn status(&self) -> GameStatus {
        self.0.status()
    }

    /// Attempts to play in the indicated column.
    ///
    /// If the column has any [`None`] in it, plays at the physically lowest one (that is,
    /// the one with the highest row index).
    ///
    /// # Errors
    /// - [`PlayError::Place(PlaceError::Occupied)`][PlayError::Place] if the column is full.
    /// - A [`PlayError::Place(PlaceError::OutOfBounds)`][PlayError::Place] if the column is out of
    ///   bounds.
    /// - Any errors thrown by [`MnkGame::play_at`].
    pub fn play_in(&mut self, column: usize) -> Result<(), PlayError> {
        let row = lowest_row_in(self.board(), column)?;
        self.0.play_at(row, column)
    }
}

/// Find the physically lowest (numerically greatest) row index corresponding to a [`None`].
fn lowest_row_in<const R: usize, const C: usize>(
    board: &MnkBoard<R, C>,
    column: usize,
) -> Result<usize, PlayError> {
    if column >= C {
        return Err(PlayError::Place(PlaceError::OutOfBounds(OutOfBounds {
            row: None,
            column: Some(column),
        })));
    }

    let mut rows = (0..R).rev();
    let lowest_row = rows.find(|&r| {
        board
            .get(r, column)
            .expect("column guard should guarantee in-bounds")
            .is_none()
    });
    lowest_row.ok_or(PlayError::Place(PlaceError::Occupied { player: None }))
}

impl<const R: usize, const C: usize, const K: usize> Default for GravityGame<R, C, K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const R: usize, const C: usize, const K: usize> From<GravityGame<R, C, K>> for MnkBoard<R, C> {
    fn from(game: GravityGame<R, C, K>) -> Self {
        game.0.into()
    }
}

impl<const R: usize, const C: usize, const K: usize> From<GravityGame<R, C, K>>
    for MnkGame<R, C, K>
{
    fn from(game: GravityGame<R, C, K>) -> Self {
        game.0
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
        let one_one: MnkBoard<1, 1> = MnkBoard::new();
        assert_eq!(lowest_row_in(&one_one, 0), Ok(0));

        let two_one: MnkBoard<2, 1> = MnkBoard::new();
        assert_eq!(lowest_row_in(&two_one, 0), Ok(1));

        let two_two: MnkBoard<2, 2> = MnkBoard::new();
        assert_eq!(lowest_row_in(&two_two, 1), Ok(1));
    }

    #[test]
    fn non_empty_yields_lowest() {
        let mut two_one: MnkBoard<2, 1> = MnkBoard::new();
        _ = two_one.place(Player::X, 1, 0);
        assert_eq!(lowest_row_in(&two_one, 0), Ok(0));

        let mut two_two: MnkBoard<2, 2> = MnkBoard::new();
        _ = two_two.place(Player::X, 1, 1);
        assert_eq!(lowest_row_in(&two_two, 1), Ok(0));
    }

    #[test]
    fn out_of_bounds_fails() {
        let one_one: MnkBoard<1, 1> = MnkBoard::new();
        assert_matches!(
            lowest_row_in(&one_one, 1),
            Err(PlayError::Place(PlaceError::OutOfBounds(OutOfBounds {
                row: None,
                column: Some(1)
            })))
        );
    }

    #[test]
    fn full_column_fails() {
        let mut one_one: MnkBoard<1, 1> = MnkBoard::new();
        _ = one_one.place(Player::X, 0, 0);
        assert_matches!(
            lowest_row_in(&one_one, 0),
            Err(PlayError::Place(PlaceError::Occupied { player: None }))
        );
    }
}
