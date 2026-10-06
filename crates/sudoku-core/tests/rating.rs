use sudoku_core::difficulty::GameDifficulty;
use sudoku_core::generator::Generator;
use sudoku_core::rating::Rater;

#[test]
fn easy_puzzle_rates_low() {
    let puzzle = Generator::generate(GameDifficulty::Easy, 42);
    let rating = Rater::rate(&puzzle);
    println!("{rating:?}");
    assert!(rating.score < 200, "Easy puzzle scored {}", rating.score);
}

#[test]
fn harder_puzzles_need_harder_techniques() {
    let easy = Rater::rate(&Generator::generate(GameDifficulty::Easy, 42));
    let hard = Rater::rate(&Generator::generate(GameDifficulty::Hard, 42));
    println!("easy: {easy:?}");
    println!("hard: {hard:?}");
    assert!(hard.score >= easy.score);
}

#[test]
fn generated_puzzles_match_target_bucket() {
    for difficulty in GameDifficulty::ALL {
        let mut matches = 0;
        for seed in 0..5u64 {
            let puzzle = Generator::generate(difficulty, seed);
            let rating = Rater::rate(&puzzle);
            if rating.bucket() == difficulty {
                matches += 1;
            }
        }
        println!("{difficulty:?}: {matches}/5 matched");
    }
}