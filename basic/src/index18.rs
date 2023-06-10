use std::collections::HashMap;
fn main() {
    // hash map
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let key: String = String::from("Red");
    scores.entry(key).or_insert(70);
    println!("{:?}", scores);

    let team_name: String = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let teams: Vec<String> = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores: Vec<i32> = vec![10, 50];
    let scores1: HashMap<&String, &i32> = teams
        .iter()
        .zip(initial_scores.iter())
        .collect::<HashMap<_, _>>();
    println!("{:?}", scores1);
}
