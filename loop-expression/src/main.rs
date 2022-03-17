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

    let mut vals = vec![2, 3, 1, 2, 2];
    while let Some(v @ 1) | Some(v @ 2) = vals.pop() {
        // Prints 2, 2, then 1
        println!("{}", v);
    }
}
