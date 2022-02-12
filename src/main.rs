mod reader;
mod solver;
mod test;

fn main() {
    let solutions = reader::get_words("./src/assets/solution-lexicon.json");
    let guesses = reader::get_words("./src/assets/guess-lexicon.json");
    let result = solver::solve("craze", solutions, guesses);
    for r in result {
        println!("{}", r.word);
    }
}