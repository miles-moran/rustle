use itertools::Itertools;

static GREEN: &'static str = "green";
static YELLOW: &'static str = "yellow";
static GRAY: &'static str = "gray";

fn main() {
    let solutions = vec!["joker", "poker", "loner"];
    let permutations = generate_color_permutations();

    for solution in solutions.clone() {
        for possible in solutions.clone().iter() {
            let mut possibles = vec![];
            for permutation in permutations.iter() {
                
            
                let mut add = true;
                for i in 0..5 {
                    let solution_char = solution.chars().nth(i).unwrap();
                    let possible_char = possible.chars().nth(i).unwrap();
                    let color = permutation.iter().nth(i).unwrap();
                    if color == GREEN {
                        if solution_char != possible_char {
                            add = false
                        }
                    }
                    if color == YELLOW {
                        if solution_char == possible_char {
                            add = false
                        }
                        if !possible.contains(solution_char) {
                            add = false
                        }
                    }
                    if color == GRAY {
                        if possible.contains(solution_char) {
                            add = false
                        }
                    }
                }
                if add == true {
                    possibles.push(possible.clone());
                }
            }
            let eliminated = solutions.len() - possibles.len();
            if eliminated != 3 {
                println!("{}", eliminated);
            }
        }
    }
}

//how probably is sequence (green, yellow, gray) * bits of information log2(1/p)

//gets every combo feedback could be returned
fn generate_color_permutations() -> Vec<Vec<String>>{
    let combinations = [GREEN.to_string(), YELLOW.to_string(), GRAY.to_string()].into_iter().combinations_with_replacement(5).collect_vec();
    let mut permutations = vec![];
    for combination in combinations{
       permutations.append(&mut combination.into_iter().permutations(5).unique().collect_vec());
    }
    return permutations;
}