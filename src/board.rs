use std::error::Error;
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
    /// Writes "X" for [`Player::X`] and "O" for [`Player::O`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}

/// A space that can be played on by a [`Player`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Space {
    /// A space that has not been played on yet.
    #[default]
    Empty,
    /// A space that has been taken by the indicated [`Player`].
    Stone(Player),
}

impl fmt::Display for Space {
    /// Writes a space character for [`Space::Empty`] and the player name for a [`Space::Stone`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Empty => write!(f, " "),
            Self::Stone(player) => write!(f, "{player}"),
        }
    }
}

impl From<Option<Player>> for Space {
    /// Maps [`None`] and [`Some`] to [`Space::Empty`] and [`Space::Stone`], respectively.
    fn from(player: Option<Player>) -> Self {
        player.map_or(Self::Empty, Self::Stone)
    }
}

impl From<Space> for Option<Player> {
    /// Maps [`Space::Empty`] and [`Space::Stone`] to [`None`] and [`Some`],
    /// respectively.
    fn from(space: Space) -> Self {
        match space {
            Space::Empty => None,
            Space::Stone(player) => Some(player),
        }
    }
}

/// An error which can occur when trying to place a stone.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlaceError {
    /// An error which can occur when the location already contains a [`Space::Stone`].
    Occupied {
        /// The player who owns the blocking [`Space::Stone`].
        player: Player,
    },
    /// An error which can occur when the intended location is not within the board's bounds.
    OutOfBounds {
        /// The intended (potentially out-of-bounds) row.
        row: usize,
        /// The intended (potentially out-of-bounds) column.
        column: usize,
    },
}

impl fmt::Display for PlaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Occupied { player } => {
                write!(f, "already occupied by {player}")
            }
            Self::OutOfBounds { row, column } => {
                write!(f, "out of bounds (row {row}, column {column})")
            }
        }
    }
}

impl Error for PlaceError {}

/// The board state of an [*m,n,k*-game].
///
/// *M,n,k*-games are two-player games played on an *m*-by-*n* board. Each [`Player`] takes turns
/// placing a stone in an empty [`Space`] on the board. A player wins when they have placed *k*
/// consecutive stones across a row, column, or diagonal. The game is drawn if there are no free
/// spaces and neither player has won.
///
/// An `MnkBoard<R, C, K>` struct has `R` rows and `C` columns of spaces. It considers a winner
/// to be a player with `K` stones in a row. However, the choice of which dimension represents the
/// number of rows is arbitrary; it is okay for other structs employing an `MnkBoard<R, C, K>` to
/// reinterpret `R` to be the number of columns and `C` to be the number of rows as long as
/// user-facing behavior is consistent with this assignment.
///
/// Methods for this struct are 0-indexed. Row indices at least `R` and column indices at least `C`
/// are considered out of bounds.
///
/// [*m,n,k*-game]: https://en.wikipedia.org/wiki/M,n,k-game
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MnkBoard<const R: usize, const C: usize, const K: usize> {
    row_array: [[Space; C]; R],
}

impl<const R: usize, const C: usize, const K: usize> MnkBoard<R, C, K> {
    /// Returns a board filled with [`Space::Empty`].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            row_array: [[Space::Empty; C]; R],
        }
    }

    /// Attempts to place a stone on the board.
    ///
    /// If the [`Space`] at the specified row and column is [`Space::Empty`], replaces it with a
    /// [`Space::Stone`] corresponding to `player`.
    ///
    /// # Errors
    ///
    ///  - [`PlaceError::Occupied`] if the corresponding `Space` is already a `Space::Stone`.
    ///  - [`PlaceError::OutOfBounds`] if either index is out of bounds.
    pub fn place(&mut self, player: Player, row: usize, column: usize) -> Result<(), PlaceError> {
        let location = self
            .row_array
            .get_mut(row)
            .and_then(|row| row.get_mut(column));
        location.map_or(
            Err(PlaceError::OutOfBounds { row, column }),
            |space| match space {
                Space::Stone(player) => Err(PlaceError::Occupied { player: *player }),
                Space::Empty => {
                    *space = Space::Stone(player);
                    Ok(())
                }
            },
        )
    }

    /// Place a stone on the board without bounds or overlap checking.
    ///
    /// Places a new [`Space::Stone`] even if the [`Space`] is already one. [`MnkBoard::place`] is
    /// a safe alternative.
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
        *location = Space::Stone(player);
    }

    /// Returns the [`Space`] at the specified row and column.
    ///
    /// Returns [`None`] either index is out of bounds.
    #[must_use]
    pub fn get(&self, row: usize, column: usize) -> Option<&Space> {
        self.row_array.get(row).and_then(|row| row.get(column))
    }

    /// Returns the [`Space`] at the specified row and column, without checking bounds.
    ///
    /// [`MnkBoard::get`] is a safe alternative.
    ///
    /// # Safety
    ///
    /// Both `row` and `column` must be in bounds.
    #[must_use]
    pub unsafe fn get_unchecked(&self, row: usize, column: usize) -> &Space {
        unsafe { self.row_array.get_unchecked(row).get_unchecked(column) }
    }

    /// Returns the winner of the game, or [`None`] if neither player has won.
    ///
    /// It is possible, but ill-advised, for a board to have multiple winners. In such a case, the
    /// value returned by this is an arbitrary [`Player`] but not `None`.
    #[must_use]
    pub fn winner(&self) -> Option<Player> {
        if C >= K {
            let winner = Self::winner_in_runs(self.rows());
            if winner.is_some() {
                return winner;
            }
        }
        if R >= K {
            let winner = Self::winner_in_runs(self.columns());
            if winner.is_some() {
                return winner;
            }
        }
        if R >= K && C >= K {
            let mut winner = Self::winner_in_runs(self.top_right_diagonals());
            if winner.is_some() {
                return winner;
            }
            winner = Self::winner_in_runs(self.left_down_diagonals());
            if winner.is_some() {
                return winner;
            }
            winner = Self::winner_in_runs(self.top_left_diagonals());
            if winner.is_some() {
                return winner;
            }
            Self::winner_in_runs(self.right_down_diagonals())
        } else {
            None
        }
    }

    /// Returns the first [`Player`] to be a winner in any of the passed runs.
    #[must_use]
    fn winner_in_runs<'a>(
        runs: impl IntoIterator<Item = impl IntoIterator<Item = &'a Space>>,
    ) -> Option<Player> {
        let mut winners = runs.into_iter().map(Self::winner_in_run);
        winners.find(Option::is_some).flatten()
    }

    /// Returns the first [`Player`] to have `K` consecutive [`Space`] instances in the
    /// [`Iterator`].
    #[must_use]
    fn winner_in_run<'a>(run: impl IntoIterator<Item = &'a Space>) -> Option<Player> {
        let mut consecutive = 0;
        let mut previous = &Space::Empty;
        for space in run {
            match *space {
                Space::Empty => {
                    consecutive = 0;
                }
                Space::Stone(player) => {
                    if space == previous {
                        consecutive += 1;
                    } else {
                        consecutive = 1;
                    }
                    if consecutive == K {
                        return Some(player);
                    }
                }
            }
            previous = space;
        }
        None
    }

    /// Converts (row, column) pairs to their corresponding [`Space`] instances.
    ///
    /// # Panics
    ///
    /// If a coordinate pair is out of bounds.
    fn coords_to_spaces(
        &self,
        coords: impl Iterator<Item = (usize, usize)>,
    ) -> impl Iterator<Item = &'_ Space> {
        coords.map(move |(r, c)| &self.row_array[r][c])
    }

    /// Returns an [`Iterator`] over the rows of the board.
    fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &'_ Space>> {
        self.row_array.iter().map(|row| row.iter())
    }

    /// Returns an [`Iterator`] over the columns of the board.
    fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &'_ Space>> {
        (0..C).map(move |c| self.row_array.iter().map(move |row| &row[c]))
    }

    /// Returns an [`Iterator`] over diagonals that start at the top and move right.
    ///
    /// Only iterates over diagonals of length at least `K`.
    fn top_right_diagonals(&self) -> impl Iterator<Item = impl Iterator<Item = &'_ Space>> {
        (0..=(C - K)).map(move |left_col| self.coords_to_spaces(iter::zip(0..R, left_col..C)))
    }

    /// Returns an [`Iterator`] over diagonals that start on the left and move down.
    ///
    /// Skips the highest such diagonal. Only iterates over diagonals of length at least `K`.
    fn left_down_diagonals(&self) -> impl Iterator<Item = impl Iterator<Item = &'_ Space>> {
        (1..=(R - K)).map(move |top_row| self.coords_to_spaces(iter::zip(top_row..R, 0..C)))
    }

    /// Returns an [`Iterator`] over the diagonals that start at the top and move left.
    ///
    /// Only iterates over diagonals of length at least `K`.
    fn top_left_diagonals(&self) -> impl Iterator<Item = impl Iterator<Item = &'_ Space>> {
        ((K - 1)..C)
            .map(move |last_col| self.coords_to_spaces(iter::zip(0..R, (0..=last_col).rev())))
    }

    /// Returns an [`Iterator`] over the diagonals that start on the right and move down.
    ///
    /// Skips the highest such diagonal. Only iterates over diagonals of length at least `K`.
    fn right_down_diagonals(&self) -> impl Iterator<Item = impl Iterator<Item = &'_ Space>> {
        (1..=(R - K))
            .map(move |last_row| self.coords_to_spaces(iter::zip(last_row..R, (0..C).rev())))
    }
}

#[cfg(test)]
mod test_placers {
    use super::*;

    #[test]
    fn place_success() {
        let mut empty = MnkBoard::<2, 2, 2>::new();

        let top_left = empty.place(Player::X, 0, 0);
        assert_eq!(top_left, Ok(()));
        assert_eq!(
            empty.row_array,
            [
                [Space::Stone(Player::X), Space::Empty],
                [Space::Empty, Space::Empty]
            ]
        );

        let top_right = empty.place(Player::O, 0, 1);
        assert_eq!(top_right, Ok(()));
        assert_eq!(
            empty.row_array,
            [
                [Space::Stone(Player::X), Space::Stone(Player::O)],
                [Space::Empty, Space::Empty]
            ]
        );

        let bottom_left = empty.place(Player::O, 1, 0);
        assert_eq!(bottom_left, Ok(()));
        assert_eq!(
            empty.row_array,
            [
                [Space::Stone(Player::X), Space::Stone(Player::O)],
                [Space::Stone(Player::O), Space::Empty]
            ]
        );

        let bottom_right = empty.place(Player::X, 1, 1);
        assert_eq!(bottom_right, Ok(()));
        assert_eq!(
            empty.row_array,
            [
                [Space::Stone(Player::X), Space::Stone(Player::O)],
                [Space::Stone(Player::O), Space::Stone(Player::X)]
            ]
        );
    }

    #[test]
    fn place_occupied() {
        let mut full = MnkBoard::<2, 2, 2>::from([
            [Space::Stone(Player::X), Space::Stone(Player::O)],
            [Space::Stone(Player::O), Space::Stone(Player::X)],
        ]);

        let top_left_x = full.place(Player::X, 0, 0);
        assert_eq!(top_left_x, Err(PlaceError::Occupied { player: Player::X }));
        let top_left_o = full.place(Player::O, 0, 0);
        assert_eq!(top_left_o, Err(PlaceError::Occupied { player: Player::X }));

        let top_right_x = full.place(Player::X, 0, 1);
        assert_eq!(top_right_x, Err(PlaceError::Occupied { player: Player::O }));
        let top_right_o = full.place(Player::O, 0, 1);
        assert_eq!(top_right_o, Err(PlaceError::Occupied { player: Player::O }));

        let bottom_left_x = full.place(Player::X, 1, 0);
        assert_eq!(
            bottom_left_x,
            Err(PlaceError::Occupied { player: Player::O })
        );
        let bottom_left_o = full.place(Player::O, 1, 0);
        assert_eq!(
            bottom_left_o,
            Err(PlaceError::Occupied { player: Player::O })
        );

        let bottom_right_x = full.place(Player::X, 1, 1);
        assert_eq!(
            bottom_right_x,
            Err(PlaceError::Occupied { player: Player::X })
        );
        let bottom_right_o = full.place(Player::O, 1, 1);
        assert_eq!(
            bottom_right_o,
            Err(PlaceError::Occupied { player: Player::X })
        );
    }

    #[test]
    fn place_out_of_bounds() {
        let mut empty = MnkBoard::<2, 2, 2>::new();

        let high_row_x = empty.place(Player::X, 2, 0);
        assert_eq!(
            high_row_x,
            Err(PlaceError::OutOfBounds { row: 2, column: 0 })
        );
        let high_row_o = empty.place(Player::O, 2, 0);
        assert_eq!(
            high_row_o,
            Err(PlaceError::OutOfBounds { row: 2, column: 0 })
        );

        let high_column_x = empty.place(Player::X, 0, 2);
        assert_eq!(
            high_column_x,
            Err(PlaceError::OutOfBounds { row: 0, column: 2 })
        );
        let high_column_o = empty.place(Player::O, 0, 2);
        assert_eq!(
            high_column_o,
            Err(PlaceError::OutOfBounds { row: 0, column: 2 })
        );
    }

    #[test]
    fn place_unchecked_empty() {
        let mut empty = MnkBoard::<2, 2, 2>::new();

        unsafe {
            empty.place_unchecked(Player::X, 0, 0);
        }
        assert_eq!(
            empty.row_array,
            [
                [Space::Stone(Player::X), Space::Empty],
                [Space::Empty, Space::Empty]
            ]
        );

        unsafe {
            empty.place_unchecked(Player::O, 0, 1);
        }
        assert_eq!(
            empty.row_array,
            [
                [Space::Stone(Player::X), Space::Stone(Player::O)],
                [Space::Empty, Space::Empty]
            ]
        );

        unsafe {
            empty.place_unchecked(Player::O, 1, 0);
        }
        assert_eq!(
            empty.row_array,
            [
                [Space::Stone(Player::X), Space::Stone(Player::O)],
                [Space::Stone(Player::O), Space::Empty]
            ]
        );

        unsafe {
            empty.place_unchecked(Player::X, 1, 1);
        }
        assert_eq!(
            empty.row_array,
            [
                [Space::Stone(Player::X), Space::Stone(Player::O)],
                [Space::Stone(Player::O), Space::Stone(Player::X)]
            ]
        );
    }

    #[test]
    fn place_unchecked_occupied() {
        let mut all_x = MnkBoard::<2, 2, 2>::from([
            [Space::Stone(Player::X), Space::Stone(Player::X)],
            [Space::Stone(Player::X), Space::Stone(Player::X)],
        ]);

        unsafe {
            all_x.place_unchecked(Player::O, 0, 0);
        }
        assert_eq!(
            all_x.row_array,
            [
                [Space::Stone(Player::O), Space::Stone(Player::X)],
                [Space::Stone(Player::X), Space::Stone(Player::X)],
            ]
        );

        unsafe {
            all_x.place_unchecked(Player::O, 0, 1);
        }
        assert_eq!(
            all_x.row_array,
            [
                [Space::Stone(Player::O), Space::Stone(Player::O)],
                [Space::Stone(Player::X), Space::Stone(Player::X)],
            ]
        );

        unsafe {
            all_x.place_unchecked(Player::O, 1, 0);
        }
        assert_eq!(
            all_x.row_array,
            [
                [Space::Stone(Player::O), Space::Stone(Player::O)],
                [Space::Stone(Player::O), Space::Stone(Player::X)],
            ]
        );

        unsafe {
            all_x.place_unchecked(Player::O, 1, 1);
        }
        assert_eq!(
            all_x.row_array,
            [
                [Space::Stone(Player::O), Space::Stone(Player::O)],
                [Space::Stone(Player::O), Space::Stone(Player::O)],
            ]
        );
    }
}

#[cfg(test)]
mod test_getters {
    use super::*;

    fn square() -> MnkBoard<2, 2, 2> {
        MnkBoard::<2, 2, 2>::from([
            [Space::Stone(Player::X), Space::Empty],
            [Space::Empty, Space::Stone(Player::O)],
        ])
    }

    #[test]
    fn get_in_bounds() {
        let board = square();

        assert_eq!(board.get(0, 0), Some(&Space::Stone(Player::X)));
        assert_eq!(board.get(0, 1), Some(&Space::Empty));
        assert_eq!(board.get(1, 0), Some(&Space::Empty));
        assert_eq!(board.get(1, 1), Some(&Space::Stone(Player::O)));
    }

    #[test]
    fn get_out_of_bounds() {
        let board = square();

        assert_eq!(board.get(2, 0), None);
        assert_eq!(board.get(0, 2), None);
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
        assert_eq!(top_left, &Space::Stone(Player::X));
        assert_eq!(top_right, &Space::Empty);
        assert_eq!(bottom_left, &Space::Empty);
        assert_eq!(bottom_right, &Space::Stone(Player::O));
    }
}

#[cfg(test)]
mod test_winner {
    use super::*;

    #[test]
    fn draws() {
        let empty_0x0 = MnkBoard::<0, 0, 1>::new();
        assert!(empty_0x0.winner().is_none());

        let empty_3x3 = MnkBoard::<3, 3, 3>::new();
        assert!(empty_3x3.winner().is_none());

        let drawn_3x3 = MnkBoard::<3, 3, 3>::from([
            [
                Space::Stone(Player::X),
                Space::Stone(Player::O),
                Space::Stone(Player::X),
            ],
            [
                Space::Stone(Player::X),
                Space::Stone(Player::O),
                Space::Stone(Player::O),
            ],
            [
                Space::Stone(Player::O),
                Space::Stone(Player::X),
                Space::Stone(Player::X),
            ],
        ]);
        assert!(drawn_3x3.winner().is_none());
    }

    #[test]
    fn row_win() {
        let row_win = MnkBoard::<3, 3, 3>::from([
            [
                Space::Stone(Player::X),
                Space::Stone(Player::X),
                Space::Stone(Player::X),
            ],
            [Space::Empty, Space::Empty, Space::Empty],
            [Space::Empty, Space::Empty, Space::Empty],
        ]);
        assert_eq!(row_win.winner(), Some(Player::X));
    }

    #[test]
    fn column_win() {
        let column_win = MnkBoard::<3, 3, 3>::from([
            [Space::Stone(Player::X), Space::Empty, Space::Empty],
            [Space::Stone(Player::X), Space::Empty, Space::Empty],
            [Space::Stone(Player::X), Space::Empty, Space::Empty],
        ]);
        assert_eq!(column_win.winner(), Some(Player::X));
    }

    #[test]
    fn top_right_win() {
        let top_right_win = MnkBoard::<3, 3, 2>::from([
            [Space::Stone(Player::X), Space::Empty, Space::Empty],
            [Space::Empty, Space::Stone(Player::X), Space::Empty],
            [Space::Empty, Space::Empty, Space::Empty],
        ]);
        assert_eq!(top_right_win.winner(), Some(Player::X));
    }

    #[test]
    fn left_down_win() {
        let left_down_win = MnkBoard::<4, 3, 3>::from([
            [Space::Empty, Space::Empty, Space::Empty],
            [Space::Stone(Player::X), Space::Empty, Space::Empty],
            [Space::Empty, Space::Stone(Player::X), Space::Empty],
            [Space::Empty, Space::Empty, Space::Stone(Player::X)],
        ]);
        assert_eq!(left_down_win.winner(), Some(Player::X));
    }

    #[test]
    fn top_left_win() {
        let top_left_win = MnkBoard::<3, 3, 2>::from([
            [Space::Empty, Space::Empty, Space::Stone(Player::X)],
            [Space::Empty, Space::Stone(Player::X), Space::Empty],
            [Space::Empty, Space::Empty, Space::Empty],
        ]);
        assert_eq!(top_left_win.winner(), Some(Player::X));
    }

    #[test]
    fn right_down_win() {
        let right_down_win = MnkBoard::<4, 3, 3>::from([
            [Space::Empty, Space::Empty, Space::Empty],
            [Space::Empty, Space::Empty, Space::Stone(Player::X)],
            [Space::Empty, Space::Stone(Player::X), Space::Empty],
            [Space::Stone(Player::X), Space::Empty, Space::Empty],
        ]);
        assert_eq!(right_down_win.winner(), Some(Player::X));
    }
}

#[cfg(test)]
mod test_winner_in_runs {
    use super::*;

    #[test]
    fn trivial() {
        let empty: iter::Empty<iter::Empty<&Space>> = iter::empty();
        assert!(MnkBoard::<0, 0, 1>::winner_in_runs(empty).is_none());

        let single = iter::once(iter::once(&Space::Stone(Player::X)));
        assert_eq!(MnkBoard::<0, 0, 1>::winner_in_runs(single), Some(Player::X));
    }

    #[test]
    fn several_runs() {
        let delayed = [
            iter::once(&Space::Empty),
            iter::once(&Space::Stone(Player::X)),
        ];
        assert_eq!(
            MnkBoard::<0, 0, 1>::winner_in_runs(delayed),
            Some(Player::X)
        );

        let all_empty = [
            iter::once(&Space::Empty),
            iter::once(&Space::Empty),
            iter::once(&Space::Empty),
        ];
        assert!(MnkBoard::<0, 0, 1>::winner_in_runs(all_empty).is_none());
    }
}

#[cfg(test)]
mod test_winner_in_run {
    use super::*;

    #[test]
    fn trivial() {
        let empty: [&Space; 0] = [];
        assert!(MnkBoard::<0, 0, 1>::winner_in_run(empty).is_none());
        assert!(MnkBoard::<0, 0, 2>::winner_in_run(empty).is_none());
        assert!(MnkBoard::<0, 0, 3>::winner_in_run(empty).is_none());

        let one_empty = [&Space::Empty];
        assert!(MnkBoard::<0, 0, 1>::winner_in_run(one_empty).is_none());
        assert!(MnkBoard::<0, 0, 2>::winner_in_run(one_empty).is_none());

        let one_x = [&Space::Stone(Player::X)];
        assert_eq!(MnkBoard::<1, 1, 1>::winner_in_run(one_x), Some(Player::X));
        assert!(MnkBoard::<1, 1, 2>::winner_in_run(one_x).is_none());

        let one_o = [&Space::Stone(Player::O)];
        assert_eq!(MnkBoard::<1, 1, 1>::winner_in_run(one_o), Some(Player::O));
        assert!(MnkBoard::<1, 1, 2>::winner_in_run(one_o).is_none());
    }

    #[test]
    fn single_player() {
        let right_run = [
            &Space::Empty,
            &Space::Empty,
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
        ];
        assert_eq!(
            MnkBoard::<0, 0, 3>::winner_in_run(right_run),
            Some(Player::X)
        );
        assert!(MnkBoard::<0, 0, 4>::winner_in_run(right_run).is_none());

        let interrupted = [
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
            &Space::Empty,
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
        ];
        assert_eq!(
            MnkBoard::<0, 0, 2>::winner_in_run(interrupted),
            Some(Player::X)
        );
        assert!(MnkBoard::<0, 0, 3>::winner_in_run(interrupted).is_none());

        let bookend = [
            &Space::Empty,
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
            &Space::Empty,
        ];
        assert_eq!(MnkBoard::<0, 0, 3>::winner_in_run(bookend), Some(Player::X));
        assert!(MnkBoard::<0, 0, 4>::winner_in_run(bookend).is_none());
    }

    #[test]
    fn two_player() {
        let left_heavy = [
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
        ];
        assert_eq!(
            MnkBoard::<0, 0, 2>::winner_in_run(left_heavy),
            Some(Player::X)
        );
        assert!(MnkBoard::<0, 0, 3>::winner_in_run(left_heavy).is_none());

        let right_heavy = [
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
        ];
        assert_eq!(
            MnkBoard::<0, 0, 2>::winner_in_run(right_heavy),
            Some(Player::X)
        );
        assert!(MnkBoard::<0, 0, 3>::winner_in_run(right_heavy).is_none());

        let interrupted = [
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
        ];
        assert!(MnkBoard::<0, 0, 2>::winner_in_run(interrupted).is_none());
        assert!(MnkBoard::<0, 0, 3>::winner_in_run(interrupted).is_none());
    }
}

#[cfg(test)]
mod test_square_board {
    // These tests use `Vec::contains` for durability against changes in iteration order.
    use super::*;

    fn square_board() -> MnkBoard<5, 5, 3> {
        MnkBoard::from([
            [
                Space::Empty,
                Space::Stone(Player::X),
                Space::Stone(Player::O),
                Space::Empty,
                Space::Stone(Player::X),
            ],
            [
                Space::Stone(Player::X),
                Space::Stone(Player::O),
                Space::Empty,
                Space::Stone(Player::X),
                Space::Stone(Player::O),
            ],
            [
                Space::Stone(Player::O),
                Space::Empty,
                Space::Stone(Player::X),
                Space::Stone(Player::O),
                Space::Empty,
            ],
            [
                Space::Stone(Player::O),
                Space::Stone(Player::X),
                Space::Empty,
                Space::Stone(Player::O),
                Space::Stone(Player::X),
            ],
            [
                Space::Stone(Player::X),
                Space::Stone(Player::O),
                Space::Empty,
                Space::Stone(Player::O),
                Space::Stone(Player::X),
            ],
        ])
    }

    #[test]
    fn rows() {
        let board = square_board();
        let rows: Vec<Vec<&Space>> = board.rows().map(Iterator::collect).collect();
        assert_eq!(rows.len(), 5);

        let top_row = vec![
            &Space::Empty,
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
            &Space::Empty,
            &Space::Stone(Player::X),
        ];
        assert!(rows.contains(&top_row));

        let second_row = vec![
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
            &Space::Empty,
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
        ];
        assert!(rows.contains(&second_row));

        let third_row = vec![
            &Space::Stone(Player::O),
            &Space::Empty,
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
            &Space::Empty,
        ];
        assert!(rows.contains(&third_row));

        let fourth_row = vec![
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
            &Space::Empty,
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
        ];
        assert!(rows.contains(&fourth_row));

        let fifth_row = vec![
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
            &Space::Empty,
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
        ];
        assert!(rows.contains(&fifth_row));
    }

    #[test]
    fn columns() {
        let board = square_board();
        let columns: Vec<Vec<&Space>> = board.columns().map(Iterator::collect).collect();
        assert_eq!(columns.len(), 5);

        let first_col = vec![
            &Space::Empty,
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
        ];
        assert!(columns.contains(&first_col));

        let second_col = vec![
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
            &Space::Empty,
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
        ];
        assert!(columns.contains(&second_col));

        let third_col = vec![
            &Space::Stone(Player::O),
            &Space::Empty,
            &Space::Stone(Player::X),
            &Space::Empty,
            &Space::Empty,
        ];
        assert!(columns.contains(&third_col));

        let fourth_col = vec![
            &Space::Empty,
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
            &Space::Stone(Player::O),
            &Space::Stone(Player::O),
        ];
        assert!(columns.contains(&fourth_col));

        let fifth_col = vec![
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
            &Space::Empty,
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
        ];
        assert!(columns.contains(&fifth_col));
    }

    #[test]
    fn top_right() {
        let board = square_board();
        let diags: Vec<Vec<&Space>> = board.top_right_diagonals().map(Iterator::collect).collect();
        assert_eq!(diags.len(), 3);

        let first_diag = vec![
            &Space::Empty,
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![
            &Space::Stone(Player::X),
            &Space::Empty,
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
        ];
        assert!(diags.contains(&second_diag));

        let third_diag = vec![
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
            &Space::Empty,
        ];
        assert!(diags.contains(&third_diag));
    }

    #[test]
    fn left_down() {
        let board = square_board();
        let diags: Vec<Vec<&Space>> = board.left_down_diagonals().map(Iterator::collect).collect();
        assert_eq!(diags.len(), 2);

        let first_diag = vec![
            &Space::Stone(Player::X),
            &Space::Empty,
            &Space::Empty,
            &Space::Stone(Player::O),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
            &Space::Empty,
        ];
        assert!(diags.contains(&second_diag));
    }

    #[test]
    fn top_left() {
        let board = square_board();
        let diags: Vec<Vec<&Space>> = board.top_left_diagonals().map(Iterator::collect).collect();
        assert_eq!(diags.len(), 3);

        let first_diag = vec![
            &Space::Stone(Player::O),
            &Space::Stone(Player::O),
            &Space::Stone(Player::O),
        ];
        assert!(diags.contains(&first_diag));
        let second_diag = vec![
            &Space::Empty,
            &Space::Empty,
            &Space::Empty,
            &Space::Stone(Player::O),
        ];
        assert!(diags.contains(&second_diag));

        let third_diag = vec![
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
        ];
        assert!(diags.contains(&third_diag));
    }

    #[test]
    fn right_down() {
        let board = square_board();
        let diags: Vec<Vec<&Space>> = board
            .right_down_diagonals()
            .map(Iterator::collect)
            .collect();
        assert_eq!(diags.len(), 2);

        let first_diag = vec![
            &Space::Stone(Player::O),
            &Space::Stone(Player::O),
            &Space::Empty,
            &Space::Stone(Player::O),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![&Space::Empty, &Space::Stone(Player::O), &Space::Empty];
        assert!(diags.contains(&second_diag));
    }
}

#[cfg(test)]
mod test_rectangular_boards {
    use super::*;

    fn tall_board() -> MnkBoard<5, 4, 3> {
        MnkBoard::from([
            [
                Space::Empty,
                Space::Stone(Player::X),
                Space::Stone(Player::O),
                Space::Empty,
            ],
            [
                Space::Stone(Player::X),
                Space::Stone(Player::O),
                Space::Empty,
                Space::Stone(Player::X),
            ],
            [
                Space::Stone(Player::O),
                Space::Empty,
                Space::Stone(Player::X),
                Space::Stone(Player::O),
            ],
            [
                Space::Stone(Player::O),
                Space::Stone(Player::X),
                Space::Empty,
                Space::Stone(Player::O),
            ],
            [
                Space::Stone(Player::X),
                Space::Stone(Player::O),
                Space::Empty,
                Space::Stone(Player::O),
            ],
        ])
    }

    fn wide_board() -> MnkBoard<4, 5, 3> {
        MnkBoard::from([
            [
                Space::Empty,
                Space::Stone(Player::X),
                Space::Stone(Player::O),
                Space::Empty,
                Space::Stone(Player::X),
            ],
            [
                Space::Stone(Player::X),
                Space::Stone(Player::O),
                Space::Empty,
                Space::Stone(Player::X),
                Space::Stone(Player::O),
            ],
            [
                Space::Stone(Player::O),
                Space::Empty,
                Space::Stone(Player::X),
                Space::Stone(Player::O),
                Space::Empty,
            ],
            [
                Space::Stone(Player::O),
                Space::Stone(Player::X),
                Space::Empty,
                Space::Stone(Player::O),
                Space::Stone(Player::X),
            ],
        ])
    }

    #[test]
    fn tall_top_right_diags() {
        let board = tall_board();
        let diags: Vec<Vec<&Space>> = board.top_right_diagonals().map(Iterator::collect).collect();
        assert_eq!(diags.len(), 2);

        let first_diag = vec![
            &Space::Empty,
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![
            &Space::Stone(Player::X),
            &Space::Empty,
            &Space::Stone(Player::O),
        ];
        assert!(diags.contains(&second_diag));
    }

    #[test]
    fn tall_left_down_diags() {
        let board = tall_board();
        let diags: Vec<Vec<&Space>> = board.left_down_diagonals().map(Iterator::collect).collect();
        assert_eq!(diags.len(), 2);

        let first_diag = vec![
            &Space::Stone(Player::X),
            &Space::Empty,
            &Space::Empty,
            &Space::Stone(Player::O),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
            &Space::Empty,
        ];
        assert!(diags.contains(&second_diag));
    }

    #[test]
    fn tall_top_left_diags() {
        let board = tall_board();
        let diags: Vec<Vec<&Space>> = board.top_left_diagonals().map(Iterator::collect).collect();
        assert_eq!(diags.len(), 2);

        let first_diag = vec![
            &Space::Empty,
            &Space::Empty,
            &Space::Empty,
            &Space::Stone(Player::O),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![
            &Space::Stone(Player::O),
            &Space::Stone(Player::O),
            &Space::Stone(Player::O),
        ];
        assert!(diags.contains(&second_diag));
    }

    #[test]
    fn tall_right_down_diags() {
        let board = tall_board();
        let diags: Vec<Vec<&Space>> = board
            .right_down_diagonals()
            .map(Iterator::collect)
            .collect();
        assert_eq!(diags.len(), 2);

        let first_diag = vec![
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![
            &Space::Stone(Player::O),
            &Space::Empty,
            &Space::Stone(Player::O),
        ];
        assert!(diags.contains(&second_diag));
    }

    #[test]
    fn wide_top_right_diags() {
        let board = wide_board();
        let diags: Vec<Vec<&Space>> = board.top_right_diagonals().map(Iterator::collect).collect();
        assert_eq!(diags.len(), 3);

        let first_diag = vec![
            &Space::Empty,
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
            &Space::Stone(Player::O),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![
            &Space::Stone(Player::X),
            &Space::Empty,
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
        ];
        assert!(diags.contains(&second_diag));

        let third_diag = vec![
            &Space::Stone(Player::O),
            &Space::Stone(Player::X),
            &Space::Empty,
        ];
        assert!(diags.contains(&third_diag));
    }

    #[test]
    fn wide_left_down_diags() {
        let board = wide_board();
        let diags: Vec<Vec<&Space>> = board.left_down_diagonals().map(Iterator::collect).collect();
        let diag = vec![&Space::Stone(Player::X), &Space::Empty, &Space::Empty];
        assert_eq!(diags, [diag]);
    }

    #[test]
    fn wide_top_left_diags() {
        let board = wide_board();
        let diags: Vec<Vec<&Space>> = board.top_left_diagonals().map(Iterator::collect).collect();
        assert_eq!(diags.len(), 3);

        let first_diag = vec![
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
            &Space::Stone(Player::X),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![
            &Space::Empty,
            &Space::Empty,
            &Space::Empty,
            &Space::Stone(Player::O),
        ];
        assert!(diags.contains(&second_diag));

        let third_diag = vec![
            &Space::Stone(Player::O),
            &Space::Stone(Player::O),
            &Space::Stone(Player::O),
        ];
        assert!(diags.contains(&third_diag));
    }

    #[test]
    fn wide_right_up_diags() {
        let board = wide_board();
        let diags: Vec<Vec<&Space>> = board
            .right_down_diagonals()
            .map(Iterator::collect)
            .collect();
        let diag = vec![
            &Space::Stone(Player::O),
            &Space::Stone(Player::O),
            &Space::Empty,
        ];
        assert_eq!(diags, [diag]);
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

impl<const R: usize, const C: usize, const K: usize> fmt::Display for MnkBoard<R, C, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let border = "+-".repeat(C) + "+";
        let vertical_sep = "\n".to_owned() + &border + "\n";
        let middle_rows = self
            .row_array
            .map(|row| format!("|{}|", row.map(|square| square.to_string()).join("|")))
            .join(&vertical_sep);
        write!(f, "{border}\n{middle_rows}\n{border}")
    }
}

#[cfg(test)]
mod test_mnk_board_display {
    use super::*;

    #[test]
    fn squares() {
        let one = MnkBoard::<1, 1, 1>::from([[Space::Stone(Player::X)]]);
        assert_eq!(
            one.to_string(),
            "+-+\n\
             |X|\n\
             +-+"
        );

        let two = MnkBoard::<2, 2, 2>::from([
            [Space::Stone(Player::X), Space::Empty],
            [Space::Empty, Space::Stone(Player::O)],
        ]);
        assert_eq!(
            two.to_string(),
            "+-+-+\n\
             |X| |\n\
             +-+-+\n\
             | |O|\n\
             +-+-+"
        );

        let three = MnkBoard::<3, 3, 3>::from([
            [
                Space::Stone(Player::X),
                Space::Empty,
                Space::Stone(Player::O),
            ],
            [
                Space::Stone(Player::O),
                Space::Stone(Player::X),
                Space::Empty,
            ],
            [
                Space::Stone(Player::X),
                Space::Stone(Player::O),
                Space::Empty,
            ],
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
        let tall = MnkBoard::<3, 2, 2>::from([
            [Space::Stone(Player::X), Space::Empty],
            [Space::Empty, Space::Stone(Player::O)],
            [Space::Stone(Player::X), Space::Stone(Player::O)],
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

        let wide = MnkBoard::<2, 3, 2>::from([
            [
                Space::Stone(Player::X),
                Space::Empty,
                Space::Stone(Player::X),
            ],
            [
                Space::Empty,
                Space::Stone(Player::O),
                Space::Stone(Player::O),
            ],
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
