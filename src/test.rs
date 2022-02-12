use crate::reader;
use crate::solver;


pub fn test() {
    let solutions = reader::get_words("./src/assets/solution-lexicon.json");
    let guesses = reader::get_words("./src/assets/guess-lexicon.json");
    let mut count = 0;
    for s in solutions.clone() {
        let attempts = solver::solve(&s, solutions.clone(), guesses.clone());
        count += attempts.len();
        if count == 10{
            break
        }
    }

    let average =  count as f32 / solutions.len() as f32;
    println!("-------");
    println!("Solutions: {}", solutions.len()); 
    println!("Count: {}", count); 
    println!("Average: {}", average); 
}


