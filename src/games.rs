use crate::{MnkBoard, PlaceError, Player};

/// Errors which may occur when playing an *m,n,k*-game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayError {
    /// An error which may occur when trying to play a game that is over.
    GameOver(GameStatus),
    /// An error which may occur when placing a stone illegally.
    PlaceError(PlaceError),
}

/// The current status of a game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameStatus {
    /// The game is over and is a draw.
    Drawn,
    /// The game is not over.
    Ongoing,
    /// The game is over and has been won by the indicated [`Player`].
    Won(Player),
}

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
    status: GameStatus,
}

impl<const R: usize, const C: usize, const K: usize> MnkGame<R, C, K> {
    /// Returns an `MnkGame<R, C, K>` with an empty board and current player [`Player::X`].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            board: MnkBoard::<R, C, K>::new(),
            current_player: Player::X,
            status: GameStatus::Ongoing,
        }
    }

    /// The [`Player`] who will take the next turn.
    #[must_use]
    pub const fn current_player(&self) -> Player {
        self.current_player
    }

    /// The current [`GameStatus`] of the game.
    #[must_use]
    pub const fn status(&self) -> GameStatus {
        self.status
    }

    /// Attempt to play at a certain square.
    ///
    /// If successful, plays a [`Space::Stone`] at the indicated location and switches the current
    /// player. Never places a stone if it also returns an error.
    ///
    /// # Errors
    ///
    /// - [`PlayError::GameOver`] if the game is [`GameStatus::Drawn`] or [`GameStatus::Won`].
    /// - [`PlayError::PlaceError`] if the indicated location is not a valid move.
    pub fn play_at(&mut self, row: usize, column: usize) -> Result<(), PlayError> {
        match self.status {
            GameStatus::Drawn | GameStatus::Won(_) => Err(PlayError::GameOver(self.status)),
            GameStatus::Ongoing => self
                .board
                .place(self.current_player, row, column)
                .map_or_else(
                    |err| Err(PlayError::PlaceError(err)),
                    |()| {
                        self.current_player = !self.current_player;
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
        self.status = self.board.winner().map_or_else(
            || {
                if self.board.full() {
                    GameStatus::Drawn
                } else {
                    GameStatus::Ongoing
                }
            },
            GameStatus::Won,
        );
    }
}

#[cfg(test)]
mod test_play_at {
    use super::*;
    use crate::Space;

    #[test]
    fn rejects_finished_games() {
        let mut drawn = MnkGame::<1, 1, 1>::new();
        drawn.status = GameStatus::Drawn;
        assert_eq!(
            drawn.play_at(0, 0),
            Err(PlayError::GameOver(GameStatus::Drawn))
        );
        assert_eq!(drawn.board, MnkBoard::<1, 1, 1>::new());

        let mut x_won = MnkGame::<1, 1, 1>::new();
        x_won.status = GameStatus::Won(Player::X);
        assert_eq!(
            x_won.play_at(0, 0),
            Err(PlayError::GameOver(GameStatus::Won(Player::X)))
        );
        assert_eq!(x_won.board, MnkBoard::<1, 1, 1>::new());

        let mut o_won = MnkGame::<1, 1, 1>::new();
        o_won.status = GameStatus::Won(Player::O);
        assert_eq!(
            o_won.play_at(0, 0),
            Err(PlayError::GameOver(GameStatus::Won(Player::O)))
        );
        assert_eq!(o_won.board, MnkBoard::<1, 1, 1>::new());
    }

    #[test]
    fn rejects_place_errors() {
        let mut empty = MnkGame::<1, 1, 1>::new();
        assert_eq!(
            empty.play_at(1, 0),
            Err(PlayError::PlaceError(PlaceError::OutOfBounds {
                row: 1,
                column: 0
            }))
        );
    }

    #[test]
    fn depends_on_current_player() {
        let mut x_plays = MnkGame::<1, 1, 1>::new();
        assert_eq!(x_plays.play_at(0, 0), Ok(()));
        assert_eq!(x_plays.board.get(0, 0), Some(&Space::Stone(Player::X)));

        let mut o_plays = MnkGame::<1, 1, 1>::new();
        o_plays.current_player = Player::O;
        assert_eq!(o_plays.play_at(0, 0), Ok(()));
        assert_eq!(o_plays.board.get(0, 0), Some(&Space::Stone(Player::O)));
    }

    #[test]
    fn swaps_current_player() {
        let mut x_plays = MnkGame::<1, 1, 1>::new();
        assert_eq!(x_plays.play_at(0, 0), Ok(()));
        assert_eq!(x_plays.current_player, Player::O);

        let mut o_plays = MnkGame::<1, 1, 1>::new();
        o_plays.current_player = Player::O;
        assert_eq!(o_plays.play_at(0, 0), Ok(()));
        assert_eq!(o_plays.current_player, Player::X);
    }

    #[test]
    fn updates_status() {
        let mut x_wins = MnkGame::<1, 1, 1>::new();
        assert_eq!(x_wins.play_at(0, 0), Ok(()));
        assert_eq!(x_wins.status, GameStatus::Won(Player::X));
    }
}

#[cfg(test)]
mod test_update_status {
    use super::*;
    use crate::Space;

    #[test]
    fn detects_wins() {
        let mut x_wins = MnkGame::<1, 1, 1>::new();
        x_wins.board = MnkBoard::from([[Space::Stone(Player::X)]]);
        x_wins.update_status();
        assert_eq!(x_wins.status, GameStatus::Won(Player::X));

        let mut o_wins = MnkGame::<1, 1, 1>::new();
        o_wins.board = MnkBoard::from([[Space::Stone(Player::O)]]);
        o_wins.update_status();
        assert_eq!(o_wins.status, GameStatus::Won(Player::O));
    }

    #[test]
    fn detects_draws() {
        let mut drawn = MnkGame::<1, 1, 2>::new();
        drawn.board = MnkBoard::from([[Space::Stone(Player::X)]]);
        drawn.update_status();
        assert_eq!(drawn.status, GameStatus::Drawn);
    }

    #[test]
    fn detects_ongoing() {
        let mut ongoing = MnkGame::<1, 1, 1>::new();
        ongoing.update_status();
        assert_eq!(ongoing.status, GameStatus::Ongoing);
    }
}

impl<const R: usize, const C: usize, const K: usize> Default for MnkGame<R, C, K> {
    fn default() -> Self {
        Self::new()
    }
}
