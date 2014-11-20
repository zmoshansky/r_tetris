# R Tetris
========

A Tetris clone written in Rust. Updated & Adapted from https://github.com/bachm/rusty-tetris

The fall speed increases every 10 tetrominoes.

Keys:
- W / Up rotates the active tetromino clockwise.
- Q rotates the active tetromino anti-clockwise.
- Left / Right / A / D moves the active tetromino.
- Down / S drops the active tetromino.
- F1 to restart after losing.
- P to pause


## Building Instructions

To build this repository, you need [Cargo](https://github.com/rust-lang/cargo).

Clone this repository
```
git clone https://github.com/zmoshansky/r_tetris.git
```

Use Cargo to build
```
cargo build
```

Play!
```
./bin/r_tetris
```

Compiled Succesfully with `rustc 0.13.0-nightly (399ff259e 2014-11-20 00:27:07 +0000)`

