[![Rust][Rust badge]][Rust workflow]

# *m,n,k*-Games

Provides programmatic support for [*m,n,k*-games][*m,n,k*-game], in which two players compete to get *k* stones in a row
on an *m*-by- *n* board.

This project currently includes representations of boards, standard *m,n,k*-games, gravity-enabled *m,n,k*-games, and a
CLI for tic-tac-toe, gomoku, and [Connect Four]. Goals include:

- GUIs for playing games.
- Tools for solving *m,n,k*-games.

## CLI

The command line tool takes one argument, which must be `tic-tac-toe`, `gomoku`, or `connect-four`. This will start a
round of the chosen game. Player play by entering a space-separated, zero-indexed row and column (or just a column for
`connect-four`).

[Rust badge]: https://github.com/Andrew5057/mnk-games/actions/workflows/rust.yml/badge.svg

[Rust workflow]: https://github.com/Andrew5057/mnk-games/actions/workflows/rust.yml

[*m,n,k*-game]: https://en.wikipedia.org/wiki/M,n,k-game

[Connect Four]: https://en.wikipedia.org/wiki/Connect_Four
