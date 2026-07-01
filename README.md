[![Rust][Rust badge]][Rust workflow]

# *m,n,k*-Games

Provides programmatic support for [*m,n,k*-games][*m,n,k*-game], in which two players compete to get *k* stones in a row
on an *m*-by-*n* board.

This project currently includes representations of boards and standard *m,n,k*-games, as well as a CLI
for tic-tac-toe and gomoku. Goals include:

- GUIs for playing games.
- Variants like [Connect Four].
- Tools for solving *m,n,k*-games.

## CLI

The command line tool takes one argument, which must be either `tic-tac-toe` or `gomoku`. This will start a round of the
chosen game. Player play by entering a space-separated, zero-indexed row and column.

[Rust badge]: https://github.com/Andrew5057/mnk-games/actions/workflows/rust.yml/badge.svg

[Rust workflow]: https://github.com/Andrew5057/mnk-games/actions/workflows/rust.yml

[*m,n,k*-game]: https://en.wikipedia.org/wiki/M,n,k-game

[Connect Four]: https://en.wikipedia.org/wiki/Connect_Four
