//! m-by-n game boards themselves and simple foundational infrastructure for them.

use std::error::Error;
use std::ops::Not;
use std::{fmt, iter};

/// One of two players.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Player {
    /// The player who makes the first move.
    X,
    /// The player who makes the second move.
    O,
}

impl fmt::Display for Player {
    /// Writes `"X"` for [`Player::X`] and `"O"` for [`Player::O`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}

impl Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

/// An error which can occur when the intended location is not within the board's bounds.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct OutOfBounds {
    /// If [`Some`], the intended row, which is out of bounds.
    ///
    /// If [`None`], the intended row may or may not be out of bounds.
    pub row: Option<usize>,
    /// If [`Some`], the intended column, which is out of bounds.
    ///
    /// If [`None`], the intended column may or may not be out of bounds.
    pub column: Option<usize>,
}

impl fmt::Display for OutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.row, self.column) {
            (Some(row), Some(col)) => write!(f, "row {row} and column {col} are out of bounds"),
            (Some(row), None) => write!(f, "row {row} is out of bounds"),
            (None, Some(col)) => write!(f, "column {col} is out of bounds"),
            (None, None) => write!(f, "location is out of bounds"),
        }
    }
}

impl Error for OutOfBounds {}

/// An error which can occur when trying to place a stone.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlaceError {
    /// An error which can occur when the location is already occupied.
    Occupied {
        /// The player who is occupying the location, if known.
        player: Option<Player>,
    },
    /// An error which can occur when the intended location is not within the board's bounds.
    OutOfBounds(OutOfBounds),
}

impl From<OutOfBounds> for PlaceError {
    fn from(oob: OutOfBounds) -> Self {
        Self::OutOfBounds(oob)
    }
}

impl fmt::Display for PlaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Occupied { player } => match player {
                Some(player) => write!(f, "already occupied by {player}"),
                None => write!(f, "already occupied"),
            },
            Self::OutOfBounds(oob) => oob.fmt(f),
        }
    }
}

impl Error for PlaceError {}

/// A game board with `R` rows and `C` columns of spaces, each represented by an
/// [`Option<Player>`]s.
///
/// Methods for this struct are 0-indexed. Row indices at least `R` and column indices at least `C`
/// are considered out of bounds.
///
/// This struct performs very little input validation. It is intended to be wrapped by other types
/// that perform more thorough validation based on a particular game's rules, not used in
/// user-facing code directly.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MnkBoard<const R: usize, const C: usize> {
    /// A row-major array representing the spaces of the board.
    row_array: [[Option<Player>; C]; R],
}

impl<const R: usize, const C: usize> MnkBoard<R, C> {
    /// Returns a board filled with [`None`].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            row_array: [[None; C]; R],
        }
    }

    /// Returns `true` if every space on the board is [`Some`] and `false` otherwise.
    #[must_use]
    pub fn full(&self) -> bool {
        self.row_array
            .iter()
            .all(|row| row.iter().all(Option::is_some))
    }

    /// Attempts to place a stone on the board.
    ///
    /// If and only if the space at the specified row and column is [`None`], replaces
    /// it with [`Some(player)`][Some].
    ///
    /// # Errors
    ///
    ///  - [`PlaceError::Occupied`] if the corresponding space is occupied.
    ///  - [`PlaceError::OutOfBounds`] if either index is out of bounds.
    pub fn place(&mut self, player: Player, row: usize, column: usize) -> Result<(), PlaceError> {
        let space = self
            .row_array
            .get_mut(row)
            .ok_or(OutOfBounds {
                row: Some(row),
                column: None,
            })?
            .get_mut(column)
            .ok_or(OutOfBounds {
                row: None,
                column: Some(column),
            })?;
        space.map_or_else(
            || {
                *space = Some(player);
                Ok(())
            },
            |player| {
                Err(PlaceError::Occupied {
                    player: Some(player),
                })
            },
        )
    }

    /// Place [`Some(Player)`][Some] on the board without bounds or overlap checking.
    ///
    /// Replaces occupied spaces. [`MnkBoard::place`] is a safe alternative.
    ///
    /// # Safety
    ///
    /// Both `row` and `column` must be in bounds.
    pub unsafe fn place_unchecked(&mut self, player: Player, row: usize, column: usize) {
        let location;
        unsafe {
            location = self
                .row_array
                .get_unchecked_mut(row)
                .get_unchecked_mut(column);
        }
        *location = Some(player);
    }

    /// Returns the [`Option<Player>`] at the specified row and column.
    ///
    /// # Errors
    /// [`OutOfBounds`] if either index is out of bounds.
    pub fn get(&self, row: usize, column: usize) -> Result<&Option<Player>, OutOfBounds> {
        self.row_array
            .get(row)
            .ok_or(OutOfBounds {
                row: Some(row),
                column: None,
            })?
            .get(column)
            .ok_or(OutOfBounds {
                row: None,
                column: Some(column),
            })
    }

    /// Returns the [`Option<Player>`] at the specified row and column, without checking bounds.
    ///
    /// [`MnkBoard::get`] is a safe alternative.
    ///
    /// # Safety
    ///
    /// Both `row` and `column` must be in bounds.
    #[must_use]
    pub unsafe fn get_unchecked(&self, row: usize, column: usize) -> &Option<Player> {
        unsafe { self.row_array.get_unchecked(row).get_unchecked(column) }
    }

    /// Converts (row, column) pairs to their corresponding [`Option<Player>`]s.
    ///
    /// # Panics
    ///
    /// If a coordinate pair is out of bounds.
    pub(crate) fn coords_to_spaces(
        &self,
        coords: impl Iterator<Item=(usize, usize)>,
    ) -> impl Iterator<Item=&'_ Option<Player>> {
        coords.map(move |(r, c)| &self.row_array[r][c])
    }

    /// Returns an [`Iterator`] over the rows of the board.
    pub(crate) fn rows(&self) -> impl Iterator<Item=impl Iterator<Item=&'_ Option<Player>>> {
        self.row_array.iter().map(|row| row.iter())
    }

    /// Returns an [`Iterator`] over the columns of the board.
    pub(crate) fn columns(&self) -> impl Iterator<Item=impl Iterator<Item=&'_ Option<Player>>> {
        (0..C).map(move |c| self.row_array.iter().map(move |row| &row[c]))
    }

    /// Returns an [`Iterator`] over diagonals that start at the top and move right.
    ///
    /// Only iterates over diagonals of length at least `min_length`.
    pub(crate) fn top_right_diagonals(
        &self,
        min_length: usize,
    ) -> impl Iterator<Item=impl Iterator<Item=&'_ Option<Player>>> {
        (0..=(C - min_length))
            .map(move |left_col| self.coords_to_spaces(iter::zip(0..R, left_col..C)))
    }

    /// Returns an [`Iterator`] over diagonals that start on the left and move down.
    ///
    /// Only iterates over diagonals of length at least `min_length`. Skips the highest such
    /// diagonal. (This avoids overlap with [`MnkBoard::top_right_diagonals`].)
    pub(crate) fn left_down_diagonals(
        &self,
        min_length: usize,
    ) -> impl Iterator<Item=impl Iterator<Item=&'_ Option<Player>>> {
        (1..=(R - min_length))
            .map(move |top_row| self.coords_to_spaces(iter::zip(top_row..R, 0..C)))
    }

    /// Returns an [`Iterator`] over the diagonals that start at the top and move left.
    ///
    /// Only iterates over diagonals of length at least `min_length`.
    pub(crate) fn top_left_diagonals(
        &self,
        min_length: usize,
    ) -> impl Iterator<Item=impl Iterator<Item=&'_ Option<Player>>> {
        ((min_length - 1)..C)
            .map(move |last_col| self.coords_to_spaces(iter::zip(0..R, (0..=last_col).rev())))
    }

    /// Returns an [`Iterator`] over the diagonals that start on the right and move down.
    ///
    /// Only iterates over diagonals of length at least `min_length`. Skips the highest such
    /// diagonal. (This avoids overlap with [`MnkBoard::top_left_diagonals`].)
    pub(crate) fn right_down_diagonals(
        &self,
        min_length: usize,
    ) -> impl Iterator<Item=impl Iterator<Item=&'_ Option<Player>>> {
        (1..=(R - min_length))
            .map(move |last_row| self.coords_to_spaces(iter::zip(last_row..R, (0..C).rev())))
    }
}

impl<const R: usize, const C: usize> Default for MnkBoard<R, C> {
    /// Returns a board filled with [`None`].
    fn default() -> Self {
        Self::new()
    }
}

impl<const R: usize, const C: usize> From<[[Option<Player>; C]; R]> for MnkBoard<R, C> {
    /// Converts a row-major array into an `MnkBoard`.
    fn from(rows: [[Option<Player>; C]; R]) -> Self {
        Self { row_array: rows }
    }
}

impl<const R: usize, const C: usize> From<MnkBoard<R, C>> for [[Option<Player>; C]; R] {
    /// Converts an `MnkBoard` into a row-major array.
    fn from(game: MnkBoard<R, C>) -> Self {
        game.row_array
    }
}

impl<const R: usize, const C: usize> fmt::Display for MnkBoard<R, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let border = "+-".repeat(C) + "+";
        let vertical_sep = "\n".to_owned() + &border + "\n";
        let middle_rows = self
            .row_array
            .map(|row| format!("|{}|", row.map(space_to_string).join("|")))
            .join(&vertical_sep);
        write!(f, "{border}\n{middle_rows}\n{border}")
    }
}

/// Returns a single-character representation of the space.
fn space_to_string(space: Option<Player>) -> String {
    space.map_or_else(|| " ".to_string(), |player| player.to_string())
}

#[cfg(test)]
mod test_placers {
    use super::*;

    #[test]
    fn place_success() {
        let mut empty = MnkBoard::<2, 2>::new();

        let top_left = empty.place(Player::X, 0, 0);
        assert_eq!(top_left, Ok(()));
        assert_eq!(empty.row_array, [[Some(Player::X), None], [None, None]]);

        let top_right = empty.place(Player::O, 0, 1);
        assert_eq!(top_right, Ok(()));
        assert_eq!(
            empty.row_array,
            [[Some(Player::X), Some(Player::O)], [None, None]]
        );

        let bottom_left = empty.place(Player::O, 1, 0);
        assert_eq!(bottom_left, Ok(()));
        assert_eq!(
            empty.row_array,
            [[Some(Player::X), Some(Player::O)], [Some(Player::O), None]]
        );

        let bottom_right = empty.place(Player::X, 1, 1);
        assert_eq!(bottom_right, Ok(()));
        assert_eq!(
            empty.row_array,
            [
                [Some(Player::X), Some(Player::O)],
                [Some(Player::O), Some(Player::X)]
            ]
        );
    }

    #[test]
    fn place_occupied() {
        let mut full = MnkBoard::from([
            [Some(Player::X), Some(Player::O)],
            [Some(Player::O), Some(Player::X)],
        ]);

        let top_left_x = full.place(Player::X, 0, 0);
        assert_eq!(
            top_left_x,
            Err(PlaceError::Occupied {
                player: Some(Player::X)
            })
        );
        let top_left_o = full.place(Player::O, 0, 0);
        assert_eq!(
            top_left_o,
            Err(PlaceError::Occupied {
                player: Some(Player::X)
            })
        );

        let top_right_x = full.place(Player::X, 0, 1);
        assert_eq!(
            top_right_x,
            Err(PlaceError::Occupied {
                player: Some(Player::O)
            })
        );
        let top_right_o = full.place(Player::O, 0, 1);
        assert_eq!(
            top_right_o,
            Err(PlaceError::Occupied {
                player: Some(Player::O)
            })
        );

        let bottom_left_x = full.place(Player::X, 1, 0);
        assert_eq!(
            bottom_left_x,
            Err(PlaceError::Occupied {
                player: Some(Player::O)
            })
        );
        let bottom_left_o = full.place(Player::O, 1, 0);
        assert_eq!(
            bottom_left_o,
            Err(PlaceError::Occupied {
                player: Some(Player::O)
            })
        );

        let bottom_right_x = full.place(Player::X, 1, 1);
        assert_eq!(
            bottom_right_x,
            Err(PlaceError::Occupied {
                player: Some(Player::X)
            })
        );
        let bottom_right_o = full.place(Player::O, 1, 1);
        assert_eq!(
            bottom_right_o,
            Err(PlaceError::Occupied {
                player: Some(Player::X)
            })
        );
    }

    #[test]
    fn place_out_of_bounds() {
        let mut empty: MnkBoard<2, 2> = MnkBoard::new();

        let high_row_x = empty.place(Player::X, 2, 0);
        assert_eq!(
            high_row_x,
            Err(PlaceError::OutOfBounds(OutOfBounds {
                row: Some(2),
                column: None
            }))
        );
        let high_row_o = empty.place(Player::O, 2, 0);
        assert_eq!(
            high_row_o,
            Err(PlaceError::OutOfBounds(OutOfBounds {
                row: Some(2),
                column: None
            }))
        );

        let high_column_x = empty.place(Player::X, 0, 2);
        assert_eq!(
            high_column_x,
            Err(PlaceError::OutOfBounds(OutOfBounds {
                row: None,
                column: Some(2)
            }))
        );
        let high_column_o = empty.place(Player::O, 0, 2);
        assert_eq!(
            high_column_o,
            Err(PlaceError::OutOfBounds(OutOfBounds {
                row: None,
                column: Some(2)
            }))
        );
    }

    #[test]
    fn place_unchecked_empty() {
        let mut empty = MnkBoard::<2, 2>::new();

        unsafe {
            empty.place_unchecked(Player::X, 0, 0);
        }
        assert_eq!(empty.row_array, [[Some(Player::X), None], [None, None]]);

        unsafe {
            empty.place_unchecked(Player::O, 0, 1);
        }
        assert_eq!(
            empty.row_array,
            [[Some(Player::X), Some(Player::O)], [None, None]]
        );

        unsafe {
            empty.place_unchecked(Player::O, 1, 0);
        }
        assert_eq!(
            empty.row_array,
            [[Some(Player::X), Some(Player::O)], [Some(Player::O), None]]
        );

        unsafe {
            empty.place_unchecked(Player::X, 1, 1);
        }
        assert_eq!(
            empty.row_array,
            [
                [Some(Player::X), Some(Player::O)],
                [Some(Player::O), Some(Player::X)]
            ]
        );
    }

    #[test]
    fn place_unchecked_occupied() {
        let mut all_x = MnkBoard::from([
            [Some(Player::X), Some(Player::X)],
            [Some(Player::X), Some(Player::X)],
        ]);

        unsafe {
            all_x.place_unchecked(Player::O, 0, 0);
        }
        assert_eq!(
            all_x.row_array,
            [
                [Some(Player::O), Some(Player::X)],
                [Some(Player::X), Some(Player::X)],
            ]
        );

        unsafe {
            all_x.place_unchecked(Player::O, 0, 1);
        }
        assert_eq!(
            all_x.row_array,
            [
                [Some(Player::O), Some(Player::O)],
                [Some(Player::X), Some(Player::X)],
            ]
        );

        unsafe {
            all_x.place_unchecked(Player::O, 1, 0);
        }
        assert_eq!(
            all_x.row_array,
            [
                [Some(Player::O), Some(Player::O)],
                [Some(Player::O), Some(Player::X)],
            ]
        );

        unsafe {
            all_x.place_unchecked(Player::O, 1, 1);
        }
        assert_eq!(
            all_x.row_array,
            [
                [Some(Player::O), Some(Player::O)],
                [Some(Player::O), Some(Player::O)],
            ]
        );
    }
}

#[cfg(test)]
mod test_getters {
    use super::*;

    fn square() -> MnkBoard<2, 2> {
        MnkBoard::from([[Some(Player::X), None], [None, Some(Player::O)]])
    }

    #[test]
    fn get_in_bounds() {
        let board = square();

        assert_eq!(board.get(0, 0), Ok(&Some(Player::X)));
        assert_eq!(board.get(0, 1), Ok(&None));
        assert_eq!(board.get(1, 0), Ok(&None));
        assert_eq!(board.get(1, 1), Ok(&Some(Player::O)));
    }

    #[test]
    fn get_out_of_bounds() {
        let board = square();

        assert_eq!(
            board.get(2, 0),
            Err(OutOfBounds {
                row: Some(2),
                column: None
            })
        );
        assert_eq!(
            board.get(0, 2),
            Err(OutOfBounds {
                row: None,
                column: Some(2)
            })
        );
    }

    #[test]
    fn get_unchecked() {
        let board = square();

        let top_left;
        let top_right;
        let bottom_left;
        let bottom_right;
        unsafe {
            top_left = board.get_unchecked(0, 0);
            top_right = board.get_unchecked(0, 1);
            bottom_left = board.get_unchecked(1, 0);
            bottom_right = board.get_unchecked(1, 1);
        }
        assert_eq!(top_left, &Some(Player::X));
        assert_eq!(top_right, &None);
        assert_eq!(bottom_left, &None);
        assert_eq!(bottom_right, &Some(Player::O));
    }
}

#[cfg(test)]
mod test_square_board {
    // These tests use `Vec::contains` for durability against changes in iteration order.
    use super::*;

    fn square_board() -> MnkBoard<5, 5> {
        MnkBoard::from([
            [
                None,
                Some(Player::X),
                Some(Player::O),
                None,
                Some(Player::X),
            ],
            [
                Some(Player::X),
                Some(Player::O),
                None,
                Some(Player::X),
                Some(Player::O),
            ],
            [
                Some(Player::O),
                None,
                Some(Player::X),
                Some(Player::O),
                None,
            ],
            [
                Some(Player::O),
                Some(Player::X),
                None,
                Some(Player::O),
                Some(Player::X),
            ],
            [
                Some(Player::X),
                Some(Player::O),
                None,
                Some(Player::O),
                Some(Player::X),
            ],
        ])
    }

    #[test]
    fn rows() {
        let board = square_board();
        let rows: Vec<Vec<&Option<Player>>> = board.rows().map(Iterator::collect).collect();
        assert_eq!(rows.len(), 5);

        let top_row = vec![
            &None,
            &Some(Player::X),
            &Some(Player::O),
            &None,
            &Some(Player::X),
        ];
        assert!(rows.contains(&top_row));

        let second_row = vec![
            &Some(Player::X),
            &Some(Player::O),
            &None,
            &Some(Player::X),
            &Some(Player::O),
        ];
        assert!(rows.contains(&second_row));

        let third_row = vec![
            &Some(Player::O),
            &None,
            &Some(Player::X),
            &Some(Player::O),
            &None,
        ];
        assert!(rows.contains(&third_row));

        let fourth_row = vec![
            &Some(Player::O),
            &Some(Player::X),
            &None,
            &Some(Player::O),
            &Some(Player::X),
        ];
        assert!(rows.contains(&fourth_row));

        let fifth_row = vec![
            &Some(Player::X),
            &Some(Player::O),
            &None,
            &Some(Player::O),
            &Some(Player::X),
        ];
        assert!(rows.contains(&fifth_row));
    }

    #[test]
    fn columns() {
        let board = square_board();
        let columns: Vec<Vec<&Option<Player>>> = board.columns().map(Iterator::collect).collect();
        assert_eq!(columns.len(), 5);

        let first_col = vec![
            &None,
            &Some(Player::X),
            &Some(Player::O),
            &Some(Player::O),
            &Some(Player::X),
        ];
        assert!(columns.contains(&first_col));

        let second_col = vec![
            &Some(Player::X),
            &Some(Player::O),
            &None,
            &Some(Player::X),
            &Some(Player::O),
        ];
        assert!(columns.contains(&second_col));

        let third_col = vec![&Some(Player::O), &None, &Some(Player::X), &None, &None];
        assert!(columns.contains(&third_col));

        let fourth_col = vec![
            &None,
            &Some(Player::X),
            &Some(Player::O),
            &Some(Player::O),
            &Some(Player::O),
        ];
        assert!(columns.contains(&fourth_col));

        let fifth_col = vec![
            &Some(Player::X),
            &Some(Player::O),
            &None,
            &Some(Player::X),
            &Some(Player::X),
        ];
        assert!(columns.contains(&fifth_col));
    }

    #[test]
    fn top_right() {
        let board = square_board();
        let diags: Vec<Vec<&Option<Player>>> = board
            .top_right_diagonals(3)
            .map(Iterator::collect)
            .collect();
        assert_eq!(diags.len(), 3);

        let first_diag = vec![
            &None,
            &Some(Player::O),
            &Some(Player::X),
            &Some(Player::O),
            &Some(Player::X),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![&Some(Player::X), &None, &Some(Player::O), &Some(Player::X)];
        assert!(diags.contains(&second_diag));

        let third_diag = vec![&Some(Player::O), &Some(Player::X), &None];
        assert!(diags.contains(&third_diag));
    }

    #[test]
    fn left_down() {
        let board = square_board();
        let diags: Vec<Vec<&Option<Player>>> = board
            .left_down_diagonals(3)
            .map(Iterator::collect)
            .collect();
        assert_eq!(diags.len(), 2);

        let first_diag = vec![&Some(Player::X), &None, &None, &Some(Player::O)];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![&Some(Player::O), &Some(Player::X), &None];
        assert!(diags.contains(&second_diag));
    }

    #[test]
    fn top_left() {
        let board = square_board();
        let diags: Vec<Vec<&Option<Player>>> =
            board.top_left_diagonals(3).map(Iterator::collect).collect();
        assert_eq!(diags.len(), 3);

        let first_diag = vec![&Some(Player::O), &Some(Player::O), &Some(Player::O)];
        assert!(diags.contains(&first_diag));
        let second_diag = vec![&None, &None, &None, &Some(Player::O)];
        assert!(diags.contains(&second_diag));

        let third_diag = vec![
            &Some(Player::X),
            &Some(Player::X),
            &Some(Player::X),
            &Some(Player::X),
            &Some(Player::X),
        ];
        assert!(diags.contains(&third_diag));
    }

    #[test]
    fn right_down() {
        let board = square_board();
        let diags: Vec<Vec<&Option<Player>>> = board
            .right_down_diagonals(3)
            .map(Iterator::collect)
            .collect();
        assert_eq!(diags.len(), 2);

        let first_diag = vec![&Some(Player::O), &Some(Player::O), &None, &Some(Player::O)];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![&None, &Some(Player::O), &None];
        assert!(diags.contains(&second_diag));
    }
}

#[cfg(test)]
mod test_rectangular_boards {
    use super::*;

    fn tall_board() -> MnkBoard<5, 4> {
        MnkBoard::from([
            [None, Some(Player::X), Some(Player::O), None],
            [Some(Player::X), Some(Player::O), None, Some(Player::X)],
            [Some(Player::O), None, Some(Player::X), Some(Player::O)],
            [Some(Player::O), Some(Player::X), None, Some(Player::O)],
            [Some(Player::X), Some(Player::O), None, Some(Player::O)],
        ])
    }

    fn wide_board() -> MnkBoard<4, 5> {
        MnkBoard::from([
            [
                None,
                Some(Player::X),
                Some(Player::O),
                None,
                Some(Player::X),
            ],
            [
                Some(Player::X),
                Some(Player::O),
                None,
                Some(Player::X),
                Some(Player::O),
            ],
            [
                Some(Player::O),
                None,
                Some(Player::X),
                Some(Player::O),
                None,
            ],
            [
                Some(Player::O),
                Some(Player::X),
                None,
                Some(Player::O),
                Some(Player::X),
            ],
        ])
    }

    #[test]
    fn tall_top_right_diags() {
        let board = tall_board();
        let diags: Vec<Vec<&Option<Player>>> = board
            .top_right_diagonals(3)
            .map(Iterator::collect)
            .collect();
        assert_eq!(diags.len(), 2);

        let first_diag = vec![&None, &Some(Player::O), &Some(Player::X), &Some(Player::O)];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![&Some(Player::X), &None, &Some(Player::O)];
        assert!(diags.contains(&second_diag));
    }

    #[test]
    fn tall_left_down_diags() {
        let board = tall_board();
        let diags: Vec<Vec<&Option<Player>>> = board
            .left_down_diagonals(3)
            .map(Iterator::collect)
            .collect();
        assert_eq!(diags.len(), 2);

        let first_diag = vec![&Some(Player::X), &None, &None, &Some(Player::O)];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![&Some(Player::O), &Some(Player::X), &None];
        assert!(diags.contains(&second_diag));
    }

    #[test]
    fn tall_top_left_diags() {
        let board = tall_board();
        let diags: Vec<Vec<&Option<Player>>> =
            board.top_left_diagonals(3).map(Iterator::collect).collect();
        assert_eq!(diags.len(), 2);

        let first_diag = vec![&None, &None, &None, &Some(Player::O)];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![&Some(Player::O), &Some(Player::O), &Some(Player::O)];
        assert!(diags.contains(&second_diag));
    }

    #[test]
    fn tall_right_down_diags() {
        let board = tall_board();
        let diags: Vec<Vec<&Option<Player>>> = board
            .right_down_diagonals(3)
            .map(Iterator::collect)
            .collect();
        assert_eq!(diags.len(), 2);

        let first_diag = vec![
            &Some(Player::X),
            &Some(Player::X),
            &Some(Player::X),
            &Some(Player::X),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![&Some(Player::O), &None, &Some(Player::O)];
        assert!(diags.contains(&second_diag));
    }

    #[test]
    fn wide_top_right_diags() {
        let board = wide_board();
        let diags: Vec<Vec<&Option<Player>>> = board
            .top_right_diagonals(3)
            .map(Iterator::collect)
            .collect();
        assert_eq!(diags.len(), 3);

        let first_diag = vec![&None, &Some(Player::O), &Some(Player::X), &Some(Player::O)];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![&Some(Player::X), &None, &Some(Player::O), &Some(Player::X)];
        assert!(diags.contains(&second_diag));

        let third_diag = vec![&Some(Player::O), &Some(Player::X), &None];
        assert!(diags.contains(&third_diag));
    }

    #[test]
    fn wide_left_down_diags() {
        let board = wide_board();
        let diags: Vec<Vec<&Option<Player>>> = board
            .left_down_diagonals(3)
            .map(Iterator::collect)
            .collect();
        let diag = vec![&Some(Player::X), &None, &None];
        assert_eq!(diags, [diag]);
    }

    #[test]
    fn wide_top_left_diags() {
        let board = wide_board();
        let diags: Vec<Vec<&Option<Player>>> =
            board.top_left_diagonals(3).map(Iterator::collect).collect();
        assert_eq!(diags.len(), 3);

        let first_diag = vec![
            &Some(Player::X),
            &Some(Player::X),
            &Some(Player::X),
            &Some(Player::X),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![&None, &None, &None, &Some(Player::O)];
        assert!(diags.contains(&second_diag));

        let third_diag = vec![&Some(Player::O), &Some(Player::O), &Some(Player::O)];
        assert!(diags.contains(&third_diag));
    }

    #[test]
    fn wide_right_up_diags() {
        let board = wide_board();
        let diags: Vec<Vec<&Option<Player>>> = board
            .right_down_diagonals(3)
            .map(Iterator::collect)
            .collect();
        let diag = vec![&Some(Player::O), &Some(Player::O), &None];
        assert_eq!(diags, [diag]);
    }
}

#[cfg(test)]
mod test_mnk_board_display {
    use super::*;

    #[test]
    fn squares() {
        let one = MnkBoard::from([[Some(Player::X)]]);
        assert_eq!(
            one.to_string(),
            "+-+\n\
             |X|\n\
             +-+"
        );

        let two = MnkBoard::from([[Some(Player::X), None], [None, Some(Player::O)]]);
        assert_eq!(
            two.to_string(),
            "+-+-+\n\
             |X| |\n\
             +-+-+\n\
             | |O|\n\
             +-+-+"
        );

        let three = MnkBoard::from([
            [Some(Player::X), None, Some(Player::O)],
            [Some(Player::O), Some(Player::X), None],
            [Some(Player::X), Some(Player::O), None],
        ]);
        assert_eq!(
            three.to_string(),
            "+-+-+-+\n\
             |X| |O|\n\
             +-+-+-+\n\
             |O|X| |\n\
             +-+-+-+\n\
             |X|O| |\n\
             +-+-+-+"
        );
    }

    #[test]
    fn rectangles() {
        let tall = MnkBoard::from([
            [Some(Player::X), None],
            [None, Some(Player::O)],
            [Some(Player::X), Some(Player::O)],
        ]);
        assert_eq!(
            tall.to_string(),
            "+-+-+\n\
             |X| |\n\
             +-+-+\n\
             | |O|\n\
             +-+-+\n\
             |X|O|\n\
             +-+-+"
        );

        let wide = MnkBoard::from([
            [Some(Player::X), None, Some(Player::X)],
            [None, Some(Player::O), Some(Player::O)],
        ]);
        assert_eq!(
            wide.to_string(),
            "+-+-+-+\n\
             |X| |X|\n\
             +-+-+-+\n\
             | |O|O|\n\
             +-+-+-+"
        );
    }
}
