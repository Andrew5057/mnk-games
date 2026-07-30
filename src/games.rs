use crate::{MnkBoard, PlaceError, Player};
use std::error::Error;
use std::fmt;

/// Errors which may occur when playing a move.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlayError {
    /// An error which may occur when the game is already over.
    GameOver(GameStatus),
    /// An error which may occur when a stone cannot be placed at the indicated position.
    PlaceError(PlaceError),
    /// An error which may occur when a move is against a game's rules.
    RuleError {
        /// An informative message about the violated rule.
        message: String,
    },
}

impl fmt::Display for PlayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GameOver(status) => write!(f, "game already over: {status}"),
            Self::PlaceError(place_error) => write!(f, "impossible move: {place_error}"),
            Self::RuleError { message } => write!(f, "illegal move: {message}"),
        }
    }
}

impl Error for PlayError {}

/// The current status of a game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameStatus {
    /// The game is over and is a draw.
    Drawn,
    /// The game is not over.
    Ongoing {
        /// The [`Player`] who will play the next move.
        next: Player,
    },
    /// The game is over and has been won by the indicated [`Player`].
    Won(Player),
}

impl fmt::Display for GameStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Drawn => write!(f, "Draw"),
            Self::Ongoing { next } => write!(f, "Next: {next}"),
            Self::Won(player) => write!(f, "{player} won!"),
        }
    }
}

/// Returns the first [`Player`] to have `win_length` consecutive [`Some`]s in the
/// [`Iterator`].
#[must_use]
fn winner_in_run<'a>(
    run: impl IntoIterator<Item = &'a Option<Player>>,
    win_length: usize,
) -> Option<Player> {
    let mut consecutive = 0;
    let mut previous = &None;
    for space in run {
        match *space {
            None => {
                consecutive = 0;
            }
            Some(player) => {
                if space == previous {
                    consecutive += 1;
                } else {
                    consecutive = 1;
                }
                if consecutive == win_length {
                    return Some(player);
                }
            }
        }
        previous = space;
    }
    None
}

/// Returns the first [`Player`] to be a winner in any of the passed runs.
#[must_use]
fn winner_in_runs<'a>(
    runs: impl IntoIterator<Item = impl IntoIterator<Item = &'a Option<Player>>>,
    win_length: usize,
) -> Option<Player> {
    let mut winners = runs.into_iter().map(|run| winner_in_run(run, win_length));
    winners.find(Option::is_some).flatten()
}

/// A standard [*m,n,k*-game].
///
/// [`Player::X`] and [`Player::O`] alternate placing stones, in that order, on a board with `R`
/// rows and `C` columns until one wins by having `K` stones in a row, column, or diagonal.
///
/// Methods are zero-indexed. Rows at least `R` and columns at least `C` are considered out of
/// bounds.
///
/// [*m,n,k*-game]: https://en.wikipedia.org/wiki/M,n,k-game
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MnkGame<const R: usize, const C: usize, const K: usize> {
    board: MnkBoard<R, C>,
    status: GameStatus,
}

impl<const R: usize, const C: usize, const K: usize> MnkGame<R, C, K> {
    /// Returns a [`GameStatus::Ongoing`] `MnkGame<R, C, K>` with an empty board and current player
    /// [`Player::X`].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            board: MnkBoard::<R, C>::new(),
            status: GameStatus::Ongoing { next: Player::X },
        }
    }

    /// The current state of the game's [`MnkBoard`].
    #[must_use]
    pub const fn board(&self) -> &MnkBoard<R, C> {
        &self.board
    }

    /// The current [`GameStatus`] of the game.
    #[must_use]
    pub const fn status(&self) -> GameStatus {
        self.status
    }

    /// Attempts to play at the indicated location.
    ///
    /// If successful, plays a stone at the indicated location, switches the current [`Player`],
    /// and checks whether the [`GameStatus`] has changed. Never plays a stone if it also returns an
    /// error.
    ///
    /// # Errors
    ///
    /// - [`PlayError::GameOver`] if the game is [`GameStatus::Drawn`] or [`GameStatus::Won`].
    /// - [`PlayError::PlaceError`] if the indicated location is not a valid move.
    pub fn play_at(&mut self, row: usize, column: usize) -> Result<(), PlayError> {
        match self.status {
            GameStatus::Drawn | GameStatus::Won(_) => Err(PlayError::GameOver(self.status)),
            GameStatus::Ongoing { next } => self.board.place(next, row, column).map_or_else(
                |err| Err(PlayError::PlaceError(err)),
                |()| {
                    self.status = GameStatus::Ongoing { next: !next };
                    self.update_status();
                    Ok(())
                },
            ),
        }
    }

    /// Changes the `status` field.
    ///
    /// [`GameStatus::Won`] if the game has been won. Otherwise, [`GameStatus::Drawn`] if the board
    /// is full and [`GameStatus::Ongoing`] otherwise.
    fn update_status(&mut self) {
        self.status = self.winner().map_or_else(
            || {
                if self.board.full() {
                    GameStatus::Drawn
                } else {
                    self.status // To retain the wrapped Player
                }
            },
            GameStatus::Won,
        );
    }

    /// Returns the winner of the game, or [`None`] if neither [`Player`] has won.
    #[must_use]
    fn winner(&self) -> Option<Player> {
        if C >= K {
            let winner = winner_in_runs(self.board.rows(), K);
            if winner.is_some() {
                return winner;
            }
        }
        if R >= K {
            let winner = winner_in_runs(self.board.columns(), K);
            if winner.is_some() {
                return winner;
            }
        }
        if R >= K && C >= K {
            let mut winner = winner_in_runs(self.board.top_right_diagonals(K), K);
            if winner.is_some() {
                return winner;
            }
            winner = winner_in_runs(self.board.left_down_diagonals(K), K);
            if winner.is_some() {
                return winner;
            }
            winner = winner_in_runs(self.board.top_left_diagonals(K), K);
            if winner.is_some() {
                return winner;
            }
            winner_in_runs(self.board.right_down_diagonals(K), K)
        } else {
            None
        }
    }
}

impl<const R: usize, const C: usize, const K: usize> Default for MnkGame<R, C, K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const R: usize, const C: usize, const K: usize> From<MnkGame<R, C, K>> for MnkBoard<R, C> {
    fn from(game: MnkGame<R, C, K>) -> Self {
        game.board
    }
}

impl<const R: usize, const C: usize, const K: usize> fmt::Display for MnkGame<R, C, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}\n{}", self.board, self.status)
    }
}

#[cfg(test)]
mod test_winner_in_run {
    use super::*;

    #[test]
    fn trivial() {
        let empty: [&Option<Player>; 0] = [];
        assert!(winner_in_run(empty, 1).is_none());
        assert!(winner_in_run(empty, 2).is_none());
        assert!(winner_in_run(empty, 3).is_none());

        let one_empty = [&None];
        assert!(winner_in_run(one_empty, 1).is_none());
        assert!(winner_in_run(one_empty, 2).is_none());

        let one_x = [&Some(Player::X)];
        assert_eq!(winner_in_run(one_x, 1), Some(Player::X));
        assert!(winner_in_run(one_x, 2).is_none());

        let one_o = [&Some(Player::O)];
        assert_eq!(winner_in_run(one_o, 1), Some(Player::O));
        assert!(winner_in_run(one_o, 2).is_none());
    }

    #[test]
    fn single_player() {
        let right_run = [
            &None,
            &None,
            &Some(Player::X),
            &Some(Player::X),
            &Some(Player::X),
        ];
        assert_eq!(winner_in_run(right_run, 3), Some(Player::X));
        assert!(winner_in_run(right_run, 4).is_none());

        let interrupted = [
            &Some(Player::X),
            &Some(Player::X),
            &None,
            &Some(Player::X),
            &Some(Player::X),
        ];
        assert_eq!(winner_in_run(interrupted, 2), Some(Player::X));
        assert!(winner_in_run(interrupted, 3).is_none());

        let bookend = [
            &None,
            &Some(Player::X),
            &Some(Player::X),
            &Some(Player::X),
            &None,
        ];
        assert_eq!(winner_in_run(bookend, 3), Some(Player::X));
        assert!(winner_in_run(bookend, 4).is_none());
    }

    #[test]
    fn two_player() {
        let left_heavy = [&Some(Player::X), &Some(Player::X), &Some(Player::O)];
        assert_eq!(winner_in_run(left_heavy, 2), Some(Player::X));
        assert!(winner_in_run(left_heavy, 3).is_none());

        let right_heavy = [&Some(Player::O), &Some(Player::X), &Some(Player::X)];
        assert_eq!(winner_in_run(right_heavy, 2), Some(Player::X));
        assert!(winner_in_run(right_heavy, 3).is_none());

        let interrupted = [&Some(Player::X), &Some(Player::O), &Some(Player::X)];
        assert!(winner_in_run(interrupted, 2).is_none());
        assert!(winner_in_run(interrupted, 3).is_none());
    }
}

#[cfg(test)]
mod test_winner_in_runs {
    use super::*;
    use std::iter;

    #[test]
    fn trivial() {
        let empty: iter::Empty<iter::Empty<&Option<Player>>> = iter::empty();
        assert!(winner_in_runs(empty, 1).is_none());

        let single = iter::once(iter::once(&Some(Player::X)));
        assert_eq!(winner_in_runs(single, 1), Some(Player::X));
    }

    #[test]
    fn several_runs() {
        let delayed = [iter::once(&None), iter::once(&Some(Player::X))];
        assert_eq!(winner_in_runs(delayed, 1), Some(Player::X));

        let all_empty = [iter::once(&None), iter::once(&None), iter::once(&None)];
        assert!(winner_in_runs(all_empty, 1).is_none());
    }
}

#[cfg(test)]
mod test_play_at {
    use super::*;

    #[test]
    fn rejects_finished_games() {
        let mut drawn: MnkGame<1, 1, 1> = MnkGame::new();
        drawn.status = GameStatus::Drawn;
        assert_eq!(
            drawn.play_at(0, 0),
            Err(PlayError::GameOver(GameStatus::Drawn))
        );
        assert_eq!(drawn.board, MnkBoard::<1, 1>::new());

        let mut x_won: MnkGame<1, 1, 1> = MnkGame::new();
        x_won.status = GameStatus::Won(Player::X);
        assert_eq!(
            x_won.play_at(0, 0),
            Err(PlayError::GameOver(GameStatus::Won(Player::X)))
        );
        assert_eq!(x_won.board, MnkBoard::<1, 1>::new());

        let mut o_won: MnkGame<1, 1, 1> = MnkGame::new();
        o_won.status = GameStatus::Won(Player::O);
        assert_eq!(
            o_won.play_at(0, 0),
            Err(PlayError::GameOver(GameStatus::Won(Player::O)))
        );
        assert_eq!(o_won.board, MnkBoard::<1, 1>::new());
    }

    #[test]
    fn rejects_place_errors() {
        let mut empty: MnkGame<1, 1, 1> = MnkGame::new();
        assert_eq!(
            empty.play_at(1, 0),
            Err(PlayError::PlaceError(PlaceError::OutOfBounds))
        );
    }

    #[test]
    fn depends_on_next_player() {
        let mut x_plays: MnkGame<1, 1, 1> = MnkGame::new();
        assert_eq!(x_plays.play_at(0, 0), Ok(()));
        assert_eq!(x_plays.board.get(0, 0), Ok(&Some(Player::X)));

        let mut o_plays: MnkGame<1, 1, 1> = MnkGame::new();
        o_plays.status = GameStatus::Ongoing { next: Player::O };
        assert_eq!(o_plays.play_at(0, 0), Ok(()));
        assert_eq!(o_plays.board.get(0, 0), Ok(&Some(Player::O)));
    }

    #[test]
    fn swaps_next_player() {
        let mut x_plays: MnkGame<2, 2, 2> = MnkGame::new();
        assert_eq!(x_plays.play_at(0, 0), Ok(()));
        assert_eq!(x_plays.status, GameStatus::Ongoing { next: Player::O });

        let mut o_plays: MnkGame<2, 2, 2> = MnkGame::new();
        o_plays.status = GameStatus::Ongoing { next: Player::O };
        assert_eq!(o_plays.play_at(0, 0), Ok(()));
        assert_eq!(o_plays.status, GameStatus::Ongoing { next: Player::X });
    }

    #[test]
    fn updates_status() {
        let mut x_wins: MnkGame<1, 1, 1> = MnkGame::new();
        assert_eq!(x_wins.play_at(0, 0), Ok(()));
        assert_eq!(x_wins.status, GameStatus::Won(Player::X));
    }
}

#[cfg(test)]
mod test_update_status {
    use super::*;

    #[test]
    fn detects_wins() {
        let mut x_wins: MnkGame<1, 1, 1> = MnkGame::new();
        x_wins.board = MnkBoard::from([[Some(Player::X)]]);
        x_wins.update_status();
        assert_eq!(x_wins.status, GameStatus::Won(Player::X));

        let mut o_wins: MnkGame<1, 1, 1> = MnkGame::new();
        o_wins.board = MnkBoard::from([[Some(Player::O)]]);
        o_wins.update_status();
        assert_eq!(o_wins.status, GameStatus::Won(Player::O));
    }

    #[test]
    fn detects_draws() {
        let mut drawn: MnkGame<1, 1, 2> = MnkGame::new();
        drawn.board = MnkBoard::from([[Some(Player::X)]]);
        drawn.update_status();
        assert_eq!(drawn.status, GameStatus::Drawn);
    }

    #[test]
    fn detects_ongoing() {
        let mut ongoing: MnkGame<1, 1, 1> = MnkGame::new();
        ongoing.update_status();
        assert_eq!(ongoing.status, GameStatus::Ongoing { next: Player::X });
    }
}

#[cfg(test)]
mod test_winner {
    use super::*;

    fn ongoing_game<const R: usize, const C: usize, const K: usize>(
        board: MnkBoard<R, C>,
    ) -> MnkGame<R, C, K> {
        MnkGame {
            board,
            status: GameStatus::Ongoing { next: Player::X },
        }
    }

    #[test]
    fn draws() {
        let empty_0x0: MnkGame<0, 0, 1> = ongoing_game(MnkBoard::new());
        assert!(empty_0x0.winner().is_none());

        let empty_3x3: MnkGame<3, 3, 3> = ongoing_game(MnkBoard::new());
        assert!(empty_3x3.winner().is_none());

        let drawn_3x3: MnkGame<3, 3, 3> = ongoing_game(MnkBoard::from([
            [Some(Player::X), Some(Player::O), Some(Player::X)],
            [Some(Player::X), Some(Player::O), Some(Player::O)],
            [Some(Player::O), Some(Player::X), Some(Player::X)],
        ]));
        assert!(drawn_3x3.winner().is_none());
    }

    #[test]
    fn row_win() {
        let row_win: MnkGame<3, 3, 3> = ongoing_game(MnkBoard::from([
            [Some(Player::X), Some(Player::X), Some(Player::X)],
            [None, None, None],
            [None, None, None],
        ]));
        assert_eq!(row_win.winner(), Some(Player::X));
    }

    #[test]
    fn column_win() {
        let column_win: MnkGame<3, 3, 3> = ongoing_game(MnkBoard::from([
            [Some(Player::X), None, None],
            [Some(Player::X), None, None],
            [Some(Player::X), None, None],
        ]));
        assert_eq!(column_win.winner(), Some(Player::X));
    }

    #[test]
    fn top_right_win() {
        let top_right_win: MnkGame<3, 3, 2> = ongoing_game(MnkBoard::from([
            [Some(Player::X), None, None],
            [None, Some(Player::X), None],
            [None, None, None],
        ]));
        assert_eq!(top_right_win.winner(), Some(Player::X));
    }

    #[test]
    fn left_down_win() {
        let left_down_win: MnkGame<4, 3, 3> = ongoing_game(MnkBoard::from([
            [None, None, None],
            [Some(Player::X), None, None],
            [None, Some(Player::X), None],
            [None, None, Some(Player::X)],
        ]));
        assert_eq!(left_down_win.winner(), Some(Player::X));
    }

    #[test]
    fn top_left_win() {
        let top_left_win: MnkGame<3, 3, 2> = ongoing_game(MnkBoard::from([
            [None, None, Some(Player::X)],
            [None, Some(Player::X), None],
            [None, None, None],
        ]));
        assert_eq!(top_left_win.winner(), Some(Player::X));
    }

    #[test]
    fn right_down_win() {
        let right_down_win: MnkGame<4, 3, 3> = ongoing_game(MnkBoard::from([
            [None, None, None],
            [None, None, Some(Player::X)],
            [None, Some(Player::X), None],
            [Some(Player::X), None, None],
        ]));
        assert_eq!(right_down_win.winner(), Some(Player::X));
    }
}

#[cfg(test)]
mod test_mnk_game_display {
    use crate::{GameStatus, MnkGame, Player};

    #[test]
    fn draw() {
        let mut draw: MnkGame<1, 1, 1> = MnkGame::new();
        draw.status = GameStatus::Drawn;
        assert_eq!(
            draw.to_string(),
            "+-+\n\
             | |\n\
             +-+\n\
             Draw"
        );
    }

    #[test]
    fn ongoing() {
        let x_next: MnkGame<1, 1, 1> = MnkGame::new();
        assert_eq!(
            x_next.to_string(),
            "+-+\n\
             | |\n\
             +-+\n\
             Next: X"
        );

        let mut o_next: MnkGame<1, 1, 1> = MnkGame::new();
        o_next.status = GameStatus::Ongoing { next: Player::O };
        assert_eq!(
            o_next.to_string(),
            "+-+\n\
             | |\n\
             +-+\n\
             Next: O"
        );
    }

    #[test]
    fn won() {
        let mut x_won: MnkGame<1, 1, 1> = MnkGame::new();
        x_won.status = GameStatus::Won(Player::X);
        assert_eq!(
            x_won.to_string(),
            "+-+\n\
             | |\n\
             +-+\n\
             X won!"
        );

        let mut o_won: MnkGame<1, 1, 1> = MnkGame::new();
        o_won.status = GameStatus::Won(Player::O);
        assert_eq!(
            o_won.to_string(),
            "+-+\n\
             | |\n\
             +-+\n\
             O won!"
        );
    }
}
