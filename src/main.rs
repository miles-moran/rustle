use itertools::Itertools;
use std::collections::HashMap;
use std::time::Instant;
static GREEN: u8 = 1;
static YELLOW: u8 = 2;
static GRAY: u8 = 3;

mod reader;
#[derive(Clone)]
struct PermutationScore {
    count: u16,
}
#[derive(Debug)]
struct Suggestion {
    word: String,
    score: f32,
}

fn main() {
    // let solutions = vec!["joker".to_string(), "poker".to_string(), "loner".to_string()];
    let now = Instant::now();
    let solutions = reader::read_words_from_file("./src/assets/solution-lexicon.json")
        .unwrap()
        .words;
    let mut chars_solution = vec![];
    // let mut chars_guesses = vec![];

    let mut suggestions = vec![];
    for s in solutions.clone() {
        let s_vec = s.chars().collect_vec();
        chars_solution.push(s_vec);
    }

    let colors = generate_color_permutations();
    for s in 0..chars_solution.len() - 1 {
        let solution = chars_solution.iter().nth(s).unwrap();
        let mut permutations = colors.clone();

        for guess in chars_solution.clone() {
            let feedback = get_feedback(solution.clone(), guess.clone());
            permutations.insert(
                feedback.clone(),
                PermutationScore {
                    count: permutations
                        .get_key_value(&feedback.clone())
                        .unwrap()
                        .1
                        .count
                        + 1,
                },
            );
        }
        let mut score = 0.0;
        for permutation in permutations {
            let information = permutation.1.count as f32 / solutions.len() as f32;
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

//how probably is sequence (green, yellow, gray) * bits of information log2(1/p)

//gets every combo feedback could be returned
fn generate_color_permutations() -> HashMap<[u8; 5], PermutationScore> {
    let combinations = [GREEN, YELLOW, GRAY]
        .into_iter()
        .combinations_with_replacement(5)
        .collect_vec();
    let mut permutations: HashMap<[u8; 5], PermutationScore> = HashMap::new();
    for combination in combinations {
        let ps = combination
            .into_iter()
            .permutations(5)
            .unique()
            .collect_vec();
        for permutation in ps {
            let perm = PermutationScore { count: 0 };
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
        }

        if solution_char != possible_char {
            if solution.contains(&possible_char) {
                //TODO: Gray occurence if occurenes in possible > occurences in solution gray
                feedback[i] = YELLOW;
            }

            if !solution.contains(&possible_char) {
                feedback[i] = GRAY;
            }
        }
    }
    return feedback;
}
