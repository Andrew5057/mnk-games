use mnk_games::variants::{Gomoku, TicTacToe};
use mnk_games::{GameStatus, MnkGame};
use std::env::args;
use std::fmt::Display;
use std::process::exit;
use std::{error, io};

#[derive(Debug)]
enum Error {
    ArgumentCount { expected: String, actual: usize },
    IllegalArgument { expected: String, actual: String },
    Io(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgumentCount { expected, actual } => {
                write!(f, "expected {expected} arguments, not {actual}")
            }
            Self::IllegalArgument { expected, actual } => {
                write!(f, "expected {expected}, not {actual}")
            }
            Self::Io(error) => write!(f, "{error}"),
        }
    }
}

impl error::Error for Error {}

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{error}");
        exit(1);
    }
}

fn try_main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => play(TicTacToe::new),
        2 => match args[1].as_str() {
            "tic-tac-toe" => play(TicTacToe::new),
            "gomoku" => play(Gomoku::new),
            argument => Err(Error::IllegalArgument {
                expected: "tic-tac-toe or gomoku".to_string(),
                actual: argument.to_string(),
            }),
        },
        n => Err(Error::ArgumentCount {
            expected: "0 or 1".to_string(),
            actual: n - 1,
        }),
    }
}

/// Starts an [`MnkGame`] and play it to completion.
///
/// # Errors
///
/// Propagates IO errors as [`Error::Io`].
fn play<const R: usize, const C: usize, const K: usize>(
    game_builder: impl FnOnce() -> MnkGame<R, C, K>,
) -> Result<(), Error> {
    let mut game = game_builder();

    while game.status() == GameStatus::Ongoing {
        println!("{game}");
        request_move(&mut game)?;
        println!();
    }
    println!("{game}");
    Ok(())
}

/// Plays the first legal move that the user enters.
///
/// If the user enters an illegal or improperly formatted move, retries.
///
/// # Errors
///
/// Propagates IO errors as [`Error::Io`].
fn request_move<const R: usize, const C: usize, const K: usize>(
    game: &mut MnkGame<R, C, K>,
) -> Result<(), Error> {
    println!("Your move:");
    loop {
        let mut input = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            return Err(Error::Io(error));
        }
        let coords: Vec<Result<usize, _>> = input.trim().split(' ').map(str::parse).collect();
        match coords[..] {
            [Ok(row), Ok(column)] => match game.play_at(row, column) {
                Ok(()) => {
                    return Ok(());
                }
                Err(_) => println!("Illegal move"),
            },
            [_, _] => println!("Must enter non-negative integers"),
            _ => println!("Must enter 2 coordinates"),
        }
    }
}
