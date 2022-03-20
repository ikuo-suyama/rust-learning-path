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

fn main() {
    let clapton = Person {
        name: "Eric Clapton".to_string(),
        age: 38,
    };
    send_data_as_json(&clapton);
    send_data_as_json2(&clapton);
}
