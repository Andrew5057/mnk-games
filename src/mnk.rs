use std::{fmt, iter};

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

    /// Converts (row, column) pairs to their corresponding [`Space`] instances.
    fn coords_to_spaces(
        &self,
        coords: impl Iterator<Item = (usize, usize)>,
    ) -> impl Iterator<Item = Space> {
        coords.map(|(r, c)| self.row_array[r][c])
    }

    /// Returns an [`Iterator`] over the rows of the board.
    fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = Space>> {
        self.row_array.into_iter().map(|row| row.into_iter())
    }

    /// Returns an [`Iterator`] over the columns of the board.
    fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = Space>> {
        (0..C).map(|c| self.row_array.iter().map(move |row| row[c]))
    }

    /// Returns an [`Iterator`] over the top-left to bottom-right diagonals of the board.
    ///
    /// Only iterates over diagonals of length at least `K`.
    fn down_right_diagonals(&self) -> impl Iterator<Item = impl Iterator<Item = Space>> {
        let top_diags =
            (0..=(C - K)).map(|left_col| self.coords_to_spaces(iter::zip(0..R, left_col..C)));
        let left_diags =
            (1..=(R - K)).map(|top_row| self.coords_to_spaces(iter::zip(top_row..R, 0..C)));
        top_diags.chain(left_diags)
    }

    /// Returns an [`Iterator`] over the top-right to bottom-left diagonals of the board.
    ///
    /// Only iterates over diagonals of length at least `K`.
    fn down_left_diagonals(&self) -> impl Iterator<Item = impl Iterator<Item = Space>> {
        type RangeMap<T> = iter::Map<std::ops::Range<usize>, Box<T>>;
        let top_diags: RangeMap<dyn FnMut(usize) -> Box<dyn Iterator<Item = Space>>> = ((K - 1)..C)
            .map(Box::new(|last_col| {
                Box::new(self.coords_to_spaces(iter::zip(0..R, (0..=last_col).rev())))
            }));
        let right_diags: RangeMap<dyn FnMut(usize) -> Box<dyn Iterator<Item = Space>>> =
            // The +1 avoids  Range/RangeInclusive mismatch
            (1..(R - K + 1)).map(Box::new(|last_row| {
                Box::new(self.coords_to_spaces(iter::zip(last_row..R, (0..C).rev())))
            }));
        top_diags.chain(right_diags)
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
    fn test_rows() {
        let board = square_board();
        let rows: Vec<Vec<Space>> = board.rows().map(|r| r.collect()).collect();
        assert_eq!(rows.len(), 5);

        let top_row = vec![
            Space::Empty,
            Space::Stone(Player::X),
            Space::Stone(Player::O),
            Space::Empty,
            Space::Stone(Player::X),
        ];
        assert!(rows.contains(&top_row));

        let second_row = vec![
            Space::Stone(Player::X),
            Space::Stone(Player::O),
            Space::Empty,
            Space::Stone(Player::X),
            Space::Stone(Player::O),
        ];
        assert!(rows.contains(&second_row));

        let third_row = vec![
            Space::Stone(Player::O),
            Space::Empty,
            Space::Stone(Player::X),
            Space::Stone(Player::O),
            Space::Empty,
        ];
        assert!(rows.contains(&third_row));

        let fourth_row = vec![
            Space::Stone(Player::O),
            Space::Stone(Player::X),
            Space::Empty,
            Space::Stone(Player::O),
            Space::Stone(Player::X),
        ];
        assert!(rows.contains(&fourth_row));

        let fifth_row = vec![
            Space::Stone(Player::X),
            Space::Stone(Player::O),
            Space::Empty,
            Space::Stone(Player::O),
            Space::Stone(Player::X),
        ];
        assert!(rows.contains(&fifth_row));
    }

    #[test]
    fn test_columns() {
        let board = square_board();
        let columns: Vec<Vec<Space>> = board.columns().map(|r| r.collect()).collect();
        assert_eq!(columns.len(), 5);

        let first_col = vec![
            Space::Empty,
            Space::Stone(Player::X),
            Space::Stone(Player::O),
            Space::Stone(Player::O),
            Space::Stone(Player::X),
        ];
        assert!(columns.contains(&first_col));

        let second_col = vec![
            Space::Stone(Player::X),
            Space::Stone(Player::O),
            Space::Empty,
            Space::Stone(Player::X),
            Space::Stone(Player::O),
        ];
        assert!(columns.contains(&second_col));

        let third_col = vec![
            Space::Stone(Player::O),
            Space::Empty,
            Space::Stone(Player::X),
            Space::Empty,
            Space::Empty,
        ];
        assert!(columns.contains(&third_col));

        let fourth_col = vec![
            Space::Empty,
            Space::Stone(Player::X),
            Space::Stone(Player::O),
            Space::Stone(Player::O),
            Space::Stone(Player::O),
        ];
        assert!(columns.contains(&fourth_col));

        let fifth_col = vec![
            Space::Stone(Player::X),
            Space::Stone(Player::O),
            Space::Empty,
            Space::Stone(Player::X),
            Space::Stone(Player::X),
        ];
        assert!(columns.contains(&fifth_col));
    }

    #[test]
    fn test_down_right() {
        let board = square_board();
        let diags: Vec<Vec<Space>> = board.down_right_diagonals().map(|r| r.collect()).collect();
        assert_eq!(diags.len(), 5);

        let first_diag = vec![
            Space::Empty,
            Space::Stone(Player::O),
            Space::Stone(Player::X),
            Space::Stone(Player::O),
            Space::Stone(Player::X),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![
            Space::Stone(Player::X),
            Space::Empty,
            Space::Stone(Player::O),
            Space::Stone(Player::X),
        ];
        assert!(diags.contains(&second_diag));

        let third_diag = vec![
            Space::Stone(Player::O),
            Space::Stone(Player::X),
            Space::Empty,
        ];
        assert!(diags.contains(&third_diag));

        let fourth_diag = vec![
            Space::Stone(Player::X),
            Space::Empty,
            Space::Empty,
            Space::Stone(Player::O),
        ];
        assert!(diags.contains(&fourth_diag));

        let fifth_diag = vec![
            Space::Stone(Player::O),
            Space::Stone(Player::X),
            Space::Empty,
        ];
        assert!(diags.contains(&fifth_diag));
    }

    #[test]
    fn test_down_left() {
        let board = square_board();
        let diags: Vec<Vec<Space>> = board.down_left_diagonals().map(|r| r.collect()).collect();
        assert_eq!(diags.len(), 5);

        let first_diag = vec![
            Space::Stone(Player::O),
            Space::Stone(Player::O),
            Space::Stone(Player::O),
        ];
        assert!(diags.contains(&first_diag));
        let second_diag = vec![
            Space::Empty,
            Space::Empty,
            Space::Empty,
            Space::Stone(Player::O),
        ];
        assert!(diags.contains(&second_diag));

        let third_diag = vec![
            Space::Stone(Player::X),
            Space::Stone(Player::X),
            Space::Stone(Player::X),
            Space::Stone(Player::X),
            Space::Stone(Player::X),
        ];
        assert!(diags.contains(&third_diag));

        let fourth_diag = vec![
            Space::Stone(Player::O),
            Space::Stone(Player::O),
            Space::Empty,
            Space::Stone(Player::O),
        ];
        assert!(diags.contains(&fourth_diag));

        let fifth_diag = vec![Space::Empty, Space::Stone(Player::O), Space::Empty];
        assert!(diags.contains(&fifth_diag));
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
    fn test_tall_down_right_diags() {
        let board = tall_board();
        let diags: Vec<Vec<Space>> = board.down_right_diagonals().map(|r| r.collect()).collect();
        assert_eq!(diags.len(), 4);

        let first_diag = vec![
            Space::Empty,
            Space::Stone(Player::O),
            Space::Stone(Player::X),
            Space::Stone(Player::O),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![
            Space::Stone(Player::X),
            Space::Empty,
            Space::Stone(Player::O),
        ];
        assert!(diags.contains(&second_diag));

        let third_diag = vec![
            Space::Stone(Player::X),
            Space::Empty,
            Space::Empty,
            Space::Stone(Player::O),
        ];
        assert!(diags.contains(&third_diag));

        let fourth_diag = vec![
            Space::Stone(Player::O),
            Space::Stone(Player::X),
            Space::Empty,
        ];
        assert!(diags.contains(&fourth_diag));
    }

    #[test]
    fn test_tall_down_left_diags() {
        let board = tall_board();
        let diags: Vec<Vec<Space>> = board.down_left_diagonals().map(|r| r.collect()).collect();
        assert_eq!(diags.len(), 4);

        let first_diag = vec![
            Space::Empty,
            Space::Empty,
            Space::Empty,
            Space::Stone(Player::O),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![
            Space::Stone(Player::O),
            Space::Stone(Player::O),
            Space::Stone(Player::O),
        ];
        assert!(diags.contains(&second_diag));

        let third_diag = vec![
            Space::Stone(Player::X),
            Space::Stone(Player::X),
            Space::Stone(Player::X),
            Space::Stone(Player::X),
        ];
        assert!(diags.contains(&third_diag));

        let fourth_diag = vec![
            Space::Stone(Player::O),
            Space::Empty,
            Space::Stone(Player::O),
        ];
        assert!(diags.contains(&fourth_diag));
    }

    #[test]
    fn test_wide_down_right_diags() {
        let board = wide_board();
        let diags: Vec<Vec<Space>> = board.down_right_diagonals().map(|r| r.collect()).collect();
        assert_eq!(diags.len(), 4);

        let first_diag = vec![
            Space::Empty,
            Space::Stone(Player::O),
            Space::Stone(Player::X),
            Space::Stone(Player::O),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![
            Space::Stone(Player::X),
            Space::Empty,
            Space::Stone(Player::O),
            Space::Stone(Player::X),
        ];
        assert!(diags.contains(&second_diag));

        let third_diag = vec![
            Space::Stone(Player::O),
            Space::Stone(Player::X),
            Space::Empty,
        ];
        assert!(diags.contains(&third_diag));

        let fourth_diag = vec![Space::Stone(Player::X), Space::Empty, Space::Empty];
        assert!(diags.contains(&fourth_diag));
    }

    #[test]
    fn test_short_down_left_diags() {
        let board = wide_board();
        let diags: Vec<Vec<Space>> = board.down_left_diagonals().map(|r| r.collect()).collect();
        assert_eq!(diags.len(), 4);

        let first_diag = vec![
            Space::Stone(Player::X),
            Space::Stone(Player::X),
            Space::Stone(Player::X),
            Space::Stone(Player::X),
        ];
        assert!(diags.contains(&first_diag));

        let second_diag = vec![
            Space::Empty,
            Space::Empty,
            Space::Empty,
            Space::Stone(Player::O),
        ];
        assert!(diags.contains(&second_diag));

        let third_diag = vec![
            Space::Stone(Player::O),
            Space::Stone(Player::O),
            Space::Stone(Player::O),
        ];
        assert!(diags.contains(&third_diag));

        let fourth_diag = vec![
            Space::Stone(Player::O),
            Space::Stone(Player::O),
            Space::Empty,
        ];
        assert!(diags.contains(&fourth_diag));
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
