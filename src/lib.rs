//! Representations of k-in-a-row games.

mod board;
pub use board::{MnkBoard, OutOfBounds, PlaceError, Player};

mod games;
pub use games::{GameStatus, MnkGame, PlayError};

pub mod gravity;

/// Well-known [`MnkGame`]s.
pub mod variants;
