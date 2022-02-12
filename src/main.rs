mod reader;
mod solver;
mod test;

fn main() {
    // let solutions = reader::get_words("./src/assets/solution-lexicon.json");
    // let guesses = reader::get_words("./src/assets/guess-lexicon.json");
    // let suggestions = solver::get_suggestions(solutions, guesses);
    // for s in suggestions {
    //     println!("{}, {}", s.word, s.score)
    // }
    test::test();
}