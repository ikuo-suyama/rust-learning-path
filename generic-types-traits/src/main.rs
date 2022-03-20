struct Person {
    name: String,
    age: u8,
}

trait AsJson {
    fn as_json(&self) -> String;
}

impl AsJson for Person {
    fn as_json(&self) -> String {
        format! {
            r#"{{ "type": "person", "name": "{}", "age": {}}}"#,
            self.name, self.age
        }
    }
}

fn send_data_as_json(value: &impl AsJson) {
    println!("-> {}", value.as_json());
}

fn send_data_as_json2<T: AsJson>(value: &T) {
    println!("-> {}", value.as_json());
}

#[derive(Debug)]
struct Counter {
    length: usize,
    count: usize,
}

impl Counter {
    fn new(length: usize) -> Counter {
        Counter { count: 0, length }
    }
}

impl Iterator for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.length {
            Some(self.count)
        } else {
            None
        }
    }
}

struct Container<T> {
    value: T,
}

impl <T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
}

fn main() {
    let clapton = Person {
        name: "Eric Clapton".to_string(),
        age: 38,
    };
    send_data_as_json(&clapton);
    send_data_as_json2(&clapton);

    // Iterator
    let counter = Counter::new(6);
    for c in counter {
        println!("{}", c);
    }

    let powers_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
    assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256]);

    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new(2).value, 2);
}
