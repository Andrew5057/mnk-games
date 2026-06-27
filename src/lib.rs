//! Representations of k-in-a-row games.

mod board;
pub use board::{MnkBoard, PlaceError, Player, Space};

mod games;
pub use games::{GameStatus, MnkGame, PlayError};
