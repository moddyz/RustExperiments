use std::collections::HashMap;

fn main() {
    // Create a new hash map and populate it.
    let mut ratings = HashMap::new();
    ratings.insert(String::from("red"), 10);
    ratings.insert(String::from("blue"), 50);
    println!("{:?}", ratings);

    // Zip two vectors into a hash map.
    let colors = vec!["red", "blue"];
    let scores = vec![10, 50];
    let ratings: HashMap<_, _> = colors.into_iter().zip(scores.into_iter()).collect();
    println!("{:?}", ratings);

    // Access a stored value by key.
    let key = "red";
    let score = ratings.get(&key);
    println!("ratings[\"{}\"] = {}", key, score.unwrap());

    // Iterating over a hashmap.
    for (key, value) in &ratings {
        println!("{}: {}", key, value);
    }

    // Overwriting an existing value.
    let mut ratings = HashMap::new();
    ratings.insert(String::from("red"), 10);
    ratings.insert(String::from("red"), 20);
    println!("{:?}", ratings);

    // Only setting value if not key is not in map.
    ratings.entry(String::from("red")).or_insert(30);
    println!("{:?}", ratings);

    // Updating existing value.
    let mut score = ratings.entry(String::from("foo")).or_insert(0);
    *score += 5;
    *score += 5;
    println!("{:?}", ratings);
}
