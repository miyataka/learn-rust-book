use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(intial_scores.into_iter()).collect();

    println!("{:?}", scores);
}
