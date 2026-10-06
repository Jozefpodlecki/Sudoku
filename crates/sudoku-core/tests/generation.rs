use sudoku_core::difficulty::GameDifficulty;
use sudoku_core::generator::Generator;
use sudoku_core::solver::Solver;

#[test]
fn generated_puzzle_has_unique_solution() {
    let puzzle = Generator::generate(GameDifficulty::Easy, 42);
    assert!(Solver::is_unique(&puzzle));
}

#[test]
fn generated_puzzle_is_solvable() {
    let puzzle = Generator::generate(GameDifficulty::Easy, 42);
    assert!(Solver::solve(&puzzle).is_some());
}

#[test]
fn generated_puzzle_respects_clue_target() {
    let puzzle = Generator::generate(GameDifficulty::Easy, 42);
    let clues = (0..81).filter(|&i| puzzle.value(sudoku_core::Cell::from_index(i).unwrap()) != 0).count();
    assert!(clues <= GameDifficulty::Easy.target_clues() + 5);
}

#[test]
fn same_seed_gives_same_puzzle() {
    let a = Generator::generate(GameDifficulty::Medium, 7);
    let b = Generator::generate(GameDifficulty::Medium, 7);
    assert_eq!(a.values(), b.values());
}

#[test]
fn different_seeds_give_different_puzzles() {
    let a = Generator::generate(GameDifficulty::Medium, 1);
    let b = Generator::generate(GameDifficulty::Medium, 2);
    assert_ne!(a.values(), b.values());
}

#[test]
fn full_grid_is_unique() {
    let puzzle = Generator::generate(GameDifficulty::Easy, 42);
    assert!(Solver::is_unique(&puzzle), "full grid should be unique");
}

#[test]
fn full_grid_counts_one() {
    let puzzle = Generator::generate(GameDifficulty::Easy, 42);
    let count = Solver::count_solutions(&puzzle, 10);
    assert_eq!(count, 1, "expected exactly 1 solution, got {count}");
}

#[test]
fn debug_generated_puzzle() {
    let puzzle = Generator::generate(GameDifficulty::Easy, 42);
    let values = puzzle.values();
    for row in 0..9 {
        for col in 0..9 {
            print!("{} ", values[row * 9 + col]);
        }
        println!();
    }
    let clues = values.iter().filter(|&&v| v != 0).count();
    println!("clues: {clues}");
}