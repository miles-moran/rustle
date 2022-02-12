use itertools::Itertools;
use std::collections::HashMap;
use std::time::Instant;

static GREEN: u8 = 1;
static YELLOW: u8 = 2;
static GRAY: u8 = 3;

mod reader;

#[derive(Debug)]
struct Suggestion {
    word: String,
    score: f32,
}

fn main() {
    let solutions = get_words("./src/assets/solution-lexicon.json");
    solve(solutions);
}

fn solve(solutions:Vec<String>) {
    let now = Instant::now();
    
    let mut chars_solution = vec![];

    for solution in solutions.clone() {
        let solution_vec = solution.chars().collect_vec();
        chars_solution.push(solution_vec);
    }

    let colors = generate_color_permutations();
    let mut suggestions = vec![];

    for s in 0..chars_solution.len() - 1 {
        let solution = chars_solution.iter().nth(s).unwrap();
        let mut permutations = colors.clone();

        for guess in chars_solution.clone() {
            let feedback = get_feedback(solution.clone(), guess);
            permutations.insert(
                feedback.clone(),
                permutations.get_key_value(&feedback).unwrap().1 + 1,
            );
        }
        let mut score = 0.0;
        for permutation in permutations {
            let information = permutation.1 as f32 / solutions.len() as f32;
            if information != 0.0 {
                let c = (1.0 / information).log2() * information;
                score += c;
            }
        }
        let suggestion = Suggestion {
            word: solutions[s].to_string(),
            score: score,
        };
        suggestions.push(suggestion);
    }
    suggestions.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
    for s in suggestions {
        println!("{:?}", s);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn generate_color_permutations() -> HashMap<[u8; 5], u16> {
    let combinations = [GREEN, YELLOW, GRAY]
        .into_iter()
        .combinations_with_replacement(5)
        .collect_vec();
    let mut permutations: HashMap<[u8; 5], u16> = HashMap::new();
    for combination in combinations {
        let ps = combination
            .into_iter()
            .permutations(5)
            .unique()
            .collect_vec();
        for permutation in ps {
            let perm = 0;
            let feedback = [
                permutation[0],
                permutation[1],
                permutation[2],
                permutation[3],
                permutation[4],
            ];
            permutations.insert(feedback, perm);
        }
    }
    return permutations;
}

fn get_feedback(solution: Vec<char>, possible: Vec<char>) -> [u8; 5] {
    let mut feedback: [u8; 5] = [0, 0, 0, 0, 0];

    for i in 0..5 {
        let solution_char = solution[i];
        let possible_char = possible[i];

        if solution_char == possible_char {
            feedback[i] = GREEN;
        } else {
            if solution.contains(&possible_char) {
                //TODO: Gray occurence if occurenes in possible > occurences in solution gray
                feedback[i] = YELLOW;
            } else {
                feedback[i] = GRAY;
            }
        }
    }
    return feedback;
}

fn get_words(path:&str) -> Vec<String>{
    return reader::read_words_from_file(path)
    .unwrap()
    .words;
}