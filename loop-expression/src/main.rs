use std::collections::HashMap;

fn main() {
    let mut reviews = HashMap::new();
    reviews.insert("Rust".to_string(), "Cool");
    reviews.insert("Scala".to_string(), "Awesome");

    let key = "Scala".to_string();
    println!("{:?}", reviews.get(&key));

    let mut reviews2 = HashMap::new();
    reviews2.insert("Rust", "A");
    reviews2.insert("Scala", "A");

    let key: &str = "Rust";
    println!("{:?}", reviews2.get(key));

    // loops
    let mut counter = 1;
    let a = loop {
        counter += 1;
        if counter > 10 {
            break counter;
        }
    };
    println!("{}", a);

    while counter < 20 {
        counter += 1;
    }
    println!("{}", counter);

    let b = ["a", "b", "c"];
    for c in b.iter() {
        println!("{}", c);
    }
}
