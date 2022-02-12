mod reader;
mod solver;

fn main() {
    let possibles = reader::get_words("./src/assets/solution-lexicon.json");
    let solution = "flank";
    solver::solve(solution, possibles)
}