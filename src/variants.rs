use crate::MnkGame;

/// The 3,3,3-game, or [tic-tac-toe].
///
/// [tic-tac-toe]: https://en.wikipedia.org/wiki/Tic-tac-toe
pub type TicTacToe = MnkGame<3, 3, 3>;
/// The 15,15,5-game, or [gomoku].
///
/// [gomoku]: https://en.wikipedia.org/wiki/Gomoku
pub type Gomoku = MnkGame<15, 15, 5>;
