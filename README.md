
# Sudoku

[![Build Status](https://github.com/Jozefpodlecki/Sudoku/actions/workflows/ci.yml/badge.svg)](https://github.com/Jozefpodlecki/Sudoku/actions)
[![Rust](https://img.shields.io/badge/rust-1.101.0_nightly-blue.svg?maxAge=3600)](https://github.com/Jozefpodlecki/Sudoku)

A Sudoku engine and web client written in Rust. The solver, generator, and
difficulty rater live in a standalone `sudoku-core` crate; the Yew frontend
renders the board and drives input.

## Features

- **Generation** — puzzles carved from a randomly-filled grid, each removal
  verified unique via Knuth's Algorithm X over an exact-cover matrix
- **Logical solver** — naked/hidden singles, locked candidates, subsets,
  fish patterns (X-Wing, Swordfish), and wings (XY-Wing)
- **Difficulty rating** — scored from the technique trace and guess count,
  then bucketed into Easy / Medium / Hard / Expert / Extreme
- **Hints** — name the technique the solver would use next and highlight
  the cells involved
- **Solve Step** — apply one logical deduction at a time
- **Candidate display** — empty cells show which digits remain possible
- **Keyboard-driven** — arrow navigation, 1–9 to place, H for hint,
  S for solve step
- **Timer** with pause
- **Dark mode** with `localStorage` persistence

## Credits

<a href="https://www.flaticon.com/free-icons/sudoku" title="sudoku icons">Sudoku icons - surang - Flaticon</a>