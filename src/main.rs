use itertools::Itertools;

fn main() {
    let solutions = vec!["joker", "poker", "loner"];
    let permutations = generate_color_permutations();

    for solution in solutions {
        for permutation in permutations.iter() {
            for i in 0..5 {
                let solution_char = solution.chars().nth(i).unwrap();
                let color = permutation.iter().nth(i).unwrap();
                println!("{}", solution_char);
                println!("{}", color);
            }
        }
    }
}

//how probably is sequence (green, yellow, gray) * bits of information log2(1/p)

//gets every combo feedback could be returned
fn generate_color_permutations() -> Vec<Vec<String>>{
    let combinations = ["green".to_string(), "yellow".to_string(), "gray".to_string()].into_iter().combinations_with_replacement(5).collect_vec();
    let mut permutations = vec![];
    for combination in combinations{
       permutations.append(&mut combination.into_iter().permutations(5).unique().collect_vec());
    }
    return permutations;
}