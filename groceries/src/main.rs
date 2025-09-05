use groceries::{insert, at_index};

fn main() {
    let mut groceries = vec!["Milk".to_string(), "Eggs".to_string()];

    insert(&mut groceries, "Bread".to_string());

    println!("{:?}", groceries);
    println!("third item: {}", at_index(&groceries, 2));
}
