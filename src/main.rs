use itertools::Itertools;
use std::collections::HashMap;

static GREEN: &'static str = "green";
static YELLOW: &'static str = "yellow";
static GRAY: &'static str = "gray";


struct PermutationScore {
    probability: i32,
    information: i32,
}

fn main() {
    let solutions = vec!["joker".to_string(), "poker".to_string(), "loner".to_string()];

    for solution in solutions.clone() {
        let mut permutations = generate_color_permutations();
        for guess in solutions.clone() {
            let feedback = get_feedback(solution.clone(), guess.clone());
            let trimmed = trim_possibles(solutions.clone(), feedback.clone(), guess.clone());
            let eliminated_count = (solutions.len() - trimmed.len()) as i32;
            permutations.insert(feedback.clone(), PermutationScore{
                information: permutations.get_key_value(&feedback.clone()).unwrap().1.information + eliminated_count,
                probability: permutations.get_key_value(&feedback.clone()).unwrap().1.probability + 1
            });
        }

        for p in permutations {
            println!("{:?}", p.1.probability);
        }
    }


}

//how probably is sequence (green, yellow, gray) * bits of information log2(1/p)

//gets every combo feedback could be returned
fn generate_color_permutations() -> HashMap<Vec<String>, PermutationScore>{
    let combinations = [GREEN.to_string(), YELLOW.to_string(), GRAY.to_string()].into_iter().combinations_with_replacement(5).collect_vec();
    let mut permutations: HashMap<Vec<String>, PermutationScore> = HashMap::new();
    for combination in combinations{
        let ps = combination.into_iter().permutations(5).unique().collect_vec();
        for permutation in ps {
                let perm = PermutationScore{
                    information: 0,
                    probability: 0
                };
            permutations.insert(permutation, perm);
        }

    }
    return permutations;
}

fn get_feedback(solution:String, possible:String) -> Vec<String> {
    let mut feedback: Vec<String> = Vec::new();
    for i in 0..5 {
        let solution_char = solution.chars().nth(i).unwrap();
        let possible_char = possible.chars().nth(i).unwrap();

        if solution_char == possible_char{
            feedback.push(GREEN.to_string());
        }

        if solution_char != possible_char{
            if solution.contains(possible_char){ //TODO: Gray occurence if occurenes in possible > occurences in solution gray
                feedback.push(YELLOW.to_string())
            }

            if !solution.contains(possible_char){
                feedback.push(GRAY.to_string())
            }
        }
    }
    return feedback
}

fn trim_possibles(possibles: Vec<String>, feedback: Vec<String>, guess: String) -> Vec<String> {
    let mut trimmed: Vec<String> = Vec::new();
    for possible in possibles.clone() {
        let mut add = true;
        for i in 0..5 {
            let color = feedback.iter().nth(i).unwrap();
            let guess_char = guess.chars().nth(i).unwrap();
            let possible_char = possible.chars().nth(i).unwrap();
           
            if color == GREEN {
                if guess_char != possible_char{
                    add = false;
                }
            }

            if color == YELLOW {
                if guess_char == possible_char{
                    add = false;
                }
                if !possible.contains(guess_char) {
                    add = false;
                }
            }

            if color == GRAY {
                if possible.contains(guess_char) { //TODO: gray occurences
                    add = false;
                }
            }
            
        }
        if add == true {
            trimmed.push(possible);
        }
    }
    return trimmed
}