# AI.md

Context file for resuming work on this project in a fresh session.

## Project

Sudoku web app. Rust workspace with a `no_std` engine crate and a Yew frontend compiled to WebAssembly.

Repository: https://github.com/Jozefpodlecki/Sudoku

## Goals

- A Sudoku engine that generates unique-solution puzzles and rates them by solving technique, not clue count.
- A clean Yew UI on top of it, dark-mode first.
- Nightly Rust for the engine (const generics, portable SIMD).
- Everything testable without a browser.

## Constraints and Preferences

- `sudoku-core` is `#![no_std]` with `extern crate alloc;`. No `std`. Heap types come from `alloc`.
- Nightly Rust, pinned via `rust-toolchain.toml`. Target features in use: `generic_const_exprs`, `portable_simd`.
- No code comments anywhere. Prefer expressive names and small functions.
- Complex implementations preferred over toy versions.
- Callbacks use `Callback<MouseEvent>` with `data-*` attributes read from the event target. Not `Callback<u8>` per button.
- Variables use full names, not abbreviations. `event`, not `e`. `element`, not `el`.
- `Self` inside impl blocks, not the type name repeated.
- Associated methods over free functions. If a function's first parameter is a type the crate owns, it belongs in an `impl` block on that type.
- Type casts on `JsValue` use `unchecked_into::<T>()` when the runtime type is guaranteed by construction, not `dyn_into().ok()`. Prefer `unchecked_into` for DOM event targets and `web_sys` handles where the element type is known.
- Module `mod.rs` files declare submodules and use glob re-exports (`pub use layout::*;`), not named re-exports.
- Tailwind v4 for styling. Dark mode via `dark:` variants, `prefers-color-scheme`, and a manual toggle.
- GitHub repo link appears exactly once, in the footer. Not in the navbar, not in page content.
- No GitHub link in the navbar. Version string lives in the navbar on the right.
- Difficulty picker lives in the navbar, not on the game page.
- Home page is hero-only: icon, title, tagline, Play button. No feature cards.
- Navbar has no Play link. Brand link with `LUCIDE_GRID_3X3` icon plus label on the left, `DifficultyPicker` and version on the right.

## Repository Layout

```
crates/
  sudoku-core/          engine, no_std + alloc, nightly
    src/
      lib.rs
      bits.rs
      cell.rs
      difficulty.rs
      peers.rs
      grid.rs
      error.rs
      solver.rs
      generator.rs
      dlx/
        mod.rs
        build.rs
        cover.rs
        query.rs
        placement.rs
        bridge.rs
      techniques.rs     stub
      rating.rs         stub
web/
  index.html
  Trunk.toml
  Cargo.toml
  styles/
    main.css
  src/
    main.rs
    app.rs
    routes.rs
    storage.rs
    utils.rs
    components/
      mod.rs
      layout.rs
      navbar.rs
      footer.rs
      difficulty_picker.rs
      sudoku/
        mod.rs
        cell.rs
        grid.rs
        number_pad.rs
        controls.rs     empty
        status_bar.rs   empty
    pages/
      mod.rs
      home.rs
      game.rs
      not_found.rs
    state/              planned
```

## What Exists

### Engine (`crates/sudoku-core`)

- `lib.rs` with `#![no_std]`, `extern crate alloc;`, feature gate for `portable_simd` behind a `simd` feature.
- `bits.rs` — `Digit` newtype over `u8` (1..=9), `Mask` newtype over `u16` with bits 1..=9 active. Inherent `const fn`s for `union`, `intersect`, `xor`, `complement`, `difference`, plus `contains`, `insert`, `remove`, `toggle`, `lowest`, `highest`, `iter`, `to_digits`. Operator trait impls forward to the inherent methods. `Mask::EMPTY` and `Mask::FULL` constants. `MaskIter` iterator yields digits ascending.
- `cell.rs` — `Cell(pub(crate) u8)`, index in 0..81. `new`, `from_index`, `index`, `get`, `row`, `col`, `box_index`, `is_peer`, `peers`, `all`, `cells()`. `Display` renders as `r1c1`. `CellIter`. `Index<Cell> for [u8; 81]` impl.
- `difficulty.rs` — `GameDifficulty` enum: `Easy` (default), `Medium`, `Hard`, `Expert`, `Extreme`. `ALL`, `label`, `slug`, `target_clues`, `from_slug`, `Display`. Uses `Self` internally.
- `peers.rs` — `PEERS: [[u8; 20]; 81]`, `ROW_CELLS`, `COL_CELLS`, `BOX_CELLS`, `HOUSE_CELLS: [[u8; 9]; 27]`. All const-evaluated.
- `grid.rs` — `Grid` with `values: [u8; 81]`, `row_masks`, `col_masks`, `box_masks` as `[Mask; 9]`, and `solved: u8`. `empty`, `values`, `value`, `is_empty_at`, `solved_count`, `is_solved`, `candidates`, `row_mask`, `col_mask`, `box_mask`, `house_mask`, `place`, `remove`, `clear`, `candidates_of_house`, `duplicates_in_row`, `duplicates_in_col`, `duplicates_in_box`, `has_conflicts`. `place` returns `Result<(), SudokuError>`. `PlacementError` was removed; all errors route through `SudokuError`.
- `error.rs` — `SudokuError` enum with `CellOccupied(Cell)`, `PlacementConflict(Cell)`, `UnknownDifficulty`, `InvalidPuzzleLength(usize)`, `InvalidPuzzleCharacter(char)`, `NoSolution`, `MultipleSolutions`. `Display` and `impl core::error::Error`.
- `solver.rs` — `Solver::solve(&Grid) -> Option<Grid>`, `solve_into`, `count_solutions(&Grid, limit)`, `is_unique(&Grid)`. Delegates to `ExactCover`.
- `generator.rs` — `Generator::generate(GameDifficulty, u64) -> Grid`. Randomized MRV fill with `Pcg32` seeded from the `u64`, then carve with `Solver::is_unique` after each removal. `fill` inlines the MRV selection, distinguishes "no empty cells" (success) from "empty cell with zero candidates" (dead end), and checks `place` result before recursing. `carve` uses `let Some(digit) = grid.remove(cell) else { continue }` and restores on failure.
- `dlx/` — `ExactCover` with `nodes: Vec<Node>` and `givens: [u8; 81]`. `Node` has `left`, `right`, `up`, `down`, `column`, `row` as `u32`. Constants `CELLS`, `DIGITS`, `ROWS`, `COLS`, and column offsets live in `cover.rs`. Public API: `new`, `from_grid`, `solve`, `count_solutions`. Internal: `cover`, `uncover`, `choose_column`, `unlink_row`. `Index<u32>` and `IndexMut<u32>` impls.

Not yet implemented: `techniques.rs`, `rating.rs`.

### Web (`web`)

- `main.rs`: builds `AppContext`, initializes `wasm_logger` from `localStorage` key `RUST_LOG`, sets `console_error_panic_hook`, mounts `App` via `Renderer::with_root_and_props` into `document.body`.
- `app.rs`: `AppContext` (window, document, body, local_storage, navigator, app_name, version as `Rc<str>`), `AppContextError` via `thiserror`, `AppProps`, `App` function component wrapping routes in `ContextProvider<AppContext>` + `HashRouter` + `Switch`. `AppContext` has a manual `impl PartialEq` returning `true`.
- `routes.rs`: `Route` enum. Difficulty reconciliation still pending.
- `components/mod.rs`: declares `layout`, `navbar`, `footer`, `difficulty_picker`, `sudoku`, and glob re-exports each with `pub use module::*;`.
- `components/layout.rs`: flex column, `min-h-screen`, background and text color, navbar on top, main `flex-1`, footer at bottom. No centering inside main.
- `components/navbar.rs`: brand link with `LUCIDE_GRID_3X3` icon and "Sudoku" label to Home. Right side has `DifficultyPicker` and version string from `AppContext`, version hidden below `sm`. No Play link.
- `components/footer.rs`: copyright line on the left, GitHub link with `yew_icons::Icon` on the right.
- `components/difficulty_picker.rs`: `DifficultyPicker` (public) and `DifficultyOption` (private) in one file. Segmented control of `<Link<Route>>` per difficulty.
- `components/sudoku/mod.rs`: declares `cell`, `grid`, `number_pad`, and glob re-exports each with `pub use module::*;`.
- `components/sudoku/cell.rs`: `CellState` enum (Given/Filled/Empty), `SudokuCell` with props for index, state, selected, peer, conflict, borders, `on_click: Callback<MouseEvent>`. Carries `data-index`. Borders passed as `&'static str` from the grid.
- `components/sudoku/grid.rs`: `SudokuGrid` renders 81 cells, computes peer highlighting and 3x3 box borders, passes `on_click` down. Border classes moved into the cell to avoid double borders. Uses `tabular-nums` and responsive `text-xl sm:text-2xl`.
- `components/sudoku/number_pad.rs`: `NumberPad` with a single `on_click: Callback<MouseEvent>`. Buttons carry `data-digit` for 1-9 and `data-action="clear"`. All buttons `type="button"`.
- `pages/home.rs`: hero section with `LUCIDE_GRID_3X3` icon, title, tagline, and a Play CTA.
- `pages/game.rs`: generates on mount and on difficulty change via `Generator::generate`. State: `seed: u64`, `grid: Grid`, `givens: [bool; 81]`, `selected: Option<Cell>`. `on_cell_click` reads `data-index`, `on_pad_click` reads `data-digit` or `data-action` and guards against editing givens. `cell_from_event` and `compute_conflicts` helpers at the bottom. `compute_conflicts` iterates `cell.peers()`.
- `pages/not_found.rs`: exists, not yet reviewed.

### Styling (`web/styles/main.css`)

- `@import "tailwindcss";`
- `@custom-variant dark (&:where(.dark, .dark *));`
- Base layer sets pointer cursor on enabled buttons and `[role="button"]`.
- Custom scrollbar styling with light and dark variants.
- The `:root` CSS variable block and `body { background-color; color }` rule were removed because they hardcoded black on black and overrode Tailwind utilities.

### `web/index.html`

- Tailwind CSS linked via `<link data-trunk rel="tailwind-css" href="styles/main.css"/>`.
- Google Fonts: Oswald and Roboto.
- Inline `<style>` block for a 3x3 loader animation shown while WASM loads.
- Inline `<script>` reads `localStorage` key `dark`, falls back to `prefers-color-scheme`, and toggles the `dark` class. Two bugs identified: the computed `useDarkMode` variable is not used (the raw `dark` value is passed to `classList.toggle`), and the class is applied to `document.body` instead of `document.documentElement`. The script should also be moved to `<head>` to avoid a flash of the wrong theme.
- `body { background: #000 }` in the inline style hardcodes black and should be removed or made theme-aware.

### Tooling

- `rustup target add wasm32-unknown-unknown` needed for the build.
- Build via `trunk serve` from the `web` directory.
- Tailwind v4 wired in via Trunk (v4.2.2 seen in build output).
- `console_error_panic_hook` set once at the start of `main`.

## What Was Discussed but Not Decided

- Whether switching difficulty mid-game should confirm before discarding progress. Leaning toward accepting the reset silently, since the URL changes and that is the mental model.
- Whether `Extreme` is a meaningful tier or should be dropped. Definition under consideration: requires backtracking, i.e. the logical solver cannot complete it without guessing.
- Default difficulty. Currently `Easy` via `#[default]`. Discussion leaned toward `Medium`, not yet applied.
- Whether `app_name` and `version` on `AppContext` should be `&'static str` instead of `Rc<str>`.
- Whether the difficulty picker should be visible on all routes or only on game routes. Currently planned as always visible in the navbar.
- Exact shape of the `Route` enum for difficulty. The difficulty picker constructs a route variant that may not exist in `routes.rs` and needs reconciliation.

## Planned Work, Roughly Ordered

1. Reconcile `routes.rs` with the difficulty picker. Add a route variant that carries difficulty, or drop the picker's per-difficulty links and use query params.
2. Verify `Game` resets state via `use_effect_with(difficulty, ...)` when the difficulty prop changes. Currently seed is hardcoded to 0 inside the effect; wire it to a real source (`Date::now()` or URL parameter).
3. Fix `index.html` dark mode script: use `documentElement`, use the computed `useDarkMode` variable, move to `<head>`.
4. Remove or make theme-aware the `body { background: #000 }` rule in the inline style.
5. `sudoku-core::techniques` — logical techniques in increasing difficulty order: naked/hidden singles, locked candidates, naked/hidden subsets, X-Wing, Swordfish, XY-Wing. Each returns a `Deduction` with cells and a `Technique` tag.
6. `sudoku-core::rating` — difficulty score from the technique trace plus backtrack count. Buckets into `GameDifficulty`. Integer math only, since `portable_simd` float transcendentals require `std`.
7. Generator biasing toward target difficulty via the rating, not just clue count.
8. Keyboard input: arrow navigation, 1-9 to fill, Backspace to clear, H for hint, Ctrl+Z for undo.
9. Hint button with technique explanation and cell highlighting.
10. Status bar: timer, difficulty, mistake count, pause.
11. Undo/redo with a bounded history stack.
12. Persistence of game state to `localStorage`.
13. `state/` module in the web crate: `GameState` reducer, `Selection`, `History`, `Persistence`.

## Features Discussed for Later

- Mistake counter with assist mode and strict mode.
- Auto-candidate tracking: placing a digit eliminates it from peer candidates.
- Pencil-mark mode.
- Post-game analysis showing techniques used and rating.
- Custom puzzle import from an 81-character string.
- Daily puzzle seeded by date.
- Stats and streaks per difficulty.
- Themes beyond dark and light.
- Sound effects.
- Responsive layout for mobile.
- PWA / offline support.
- Shareable URLs for custom puzzles.

## Known Issues and Gotchas

- Yew Router's `Routable` derive does not allow multiple `#[at(...)]` attributes on one variant. Two variants are required to serve `/game` and `/game/:difficulty`, or use query params.
- `#[derive(Properties)]` requires `PartialEq` on all fields. `web_sys` types do not implement `PartialEq`, so `AppContext` has a manual `impl PartialEq` returning `true`. There is exactly one `AppContext` per page lifetime, so this is correct.
- `Route` carries `difficulty` as `String`, not `GameDifficulty`. Parsing happens in `switch` or in `Game`. The string in the route is a URL slug, not a display label. Always construct links via `.slug()`, never hand-type.
- `Link<Route>` accepts `String` or `&'static str` for the `classes` prop via `Into<Classes>`. `format!` output works. The `classes!` macro is a Yew construct, not a Tailwind one, and is only needed for conditional joins.
- `wasm32-unknown-unknown` target must be installed for the active toolchain. `rustup target add wasm32-unknown-unknown`.
- With `no_std` libraries, unit tests inside `#[cfg(test)] mod tests` need `extern crate std;` at the top of the test module, or integration tests should live in `tests/` where `std` is linked by default.
- `portable_simd` float transcendental methods (`sqrt`, `floor`, `round`) require `std`. Integer SIMD and bitwise ops work in `no_std`. Keep rating math integer-based or gate the SIMD path behind a feature.
- `portable_simd` supports `wasm32`, not `wasm64`.
- `rand` needs `default-features = false` and a compatible `rand_pcg` version to stay `no_std`. Seed `Pcg32` from a `u64` supplied by the web layer, not from `thread_rng`.
- The `dark` class must be on `<html>` (i.e. `document.documentElement`), not `<body>`, for Tailwind's `dark:` variant to behave consistently.
- The inline dark mode script in `index.html` must run in `<head>` before any content renders, otherwise there is a flash of the wrong theme.
- `Renderer::<App>::with_root_and_props(root, props)` is the correct way to mount a Yew root component with props. `App` takes `&AppProps`, not `&AppContext`.
- Sudoku cell borders must live on the cell itself, not on a wrapper `<div>`. A wrapper with borders around a bordered cell produces double lines.
- The DLX matrix has a header node at index 0. Cell constraint columns start at index 1, so `COL_OFFSET_CELL = 1`, not 0. The row nodes for logical row `r` start at node index `(COLS + 1) + r * 4`. Passing a logical row index where a node index is expected corrupts the matrix.
- `unlink_row` removes a row's vertical links without touching the column headers. `cover` removes a column from the header list. They are not interchangeable.
- Trait impls cannot be `const fn`. Any operator used inside a `const fn` body needs an inherent `const fn` counterpart on the type. `Mask` has `union`, `intersect`, `xor`, `complement`, `difference` for this reason.
- `Index<u32> for ExactCover` requires `Node` to be `pub`, not `pub(crate)`, because the trait impl is a public interface.
- Glob re-exports (`pub use module::*;`) in `mod.rs` files mean a name collision between two submodules becomes a compile error at the re-export site. If `cell.rs` and `grid.rs` both export `CellState`, the globs collide. Keep public type names unique across submodules or switch to named re-exports.

## Conventions in This Codebase

- `mod.rs` files declare submodules and glob re-export them. `pub use layout::*;`, not `pub use layout::Layout;`.
- `Callback<MouseEvent>` with `data-*` attributes for anything clickable with multiple variants.
- `type="button"` on every `<button>`.
- Full variable names. No single-letter bindings except loop indices.
- `Self` inside impl blocks.
- Associated methods over free functions when the first parameter is a crate-owned type.
- `unchecked_into::<T>()` on `JsValue` when the runtime type is guaranteed. `dyn_into().ok()` only when the type is uncertain.
- `#[prop_or_default]` for optional props, with `Default` on the relevant enum.
- `unwrap_or_default()` for fallible parsing with a sensible fallback.
- Dark mode paired on every color class: `bg-white dark:bg-zinc-950`.
- `format!` for computed class strings, not the `classes!` macro, unless conditional joins become complex.
- Helpers live at the bottom of the file that uses them. Move to `utils.rs` only when a third file needs them.
- Digits use `tabular-nums` so they do not shift width.
- Given values are `font-semibold`, user-filled are `font-medium`, empty cells are dimmed.

## Build and Test

```
rustup target add wasm32-unknown-unknown
cargo install trunk

cd web
trunk serve
```

```
cargo test -p sudoku-core
cargo clippy --workspace
trunk build --release
```

## Session Handoff

The engine now generates unique-solution puzzles end to end. `Generator::generate(GameDifficulty::Easy, seed)` returns a `Grid` that passes `Solver::is_unique`. The web `pages/game.rs` consumes that, renders it via `SudokuGrid`, and writes back through `Grid::place` and `Grid::remove`.

The two active concerns are:

1. `routes.rs` still does not carry difficulty. The `DifficultyPicker` in the navbar builds a link to a route variant that needs to match `routes.rs`. Either add `GameWithDifficulty { difficulty: String }` as a second variant alongside `Game`, or drop the per-difficulty links and move difficulty into a query parameter. Once reconciled, `Game` should regenerate via `use_effect_with(difficulty, ...)`.

2. `index.html` dark mode is broken on first visit and applies the class to the wrong element. Fix the script per the Known Issues section.

After those, the next engine task is `techniques.rs`: the logical solver that detects naked singles, hidden singles, locked candidates, subsets, fish, and wings. Each technique returns a `Deduction { placements, eliminations, technique }`. That unlocks `rating.rs`, hints, and difficulty-driven generation.