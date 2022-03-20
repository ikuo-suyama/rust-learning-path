use std::io::Read;
use std::io::Error;
use std::path::PathBuf;
use std::fs::File;

struct Person {
    first: String,
    middle: Option<String>,
    last: String,
}

fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");

    if let Some(middle) = &person.middle {
        full_name.push_str(&middle);
        full_name.push_str(" ");
    }

    full_name.push_str(&person.last);
    full_name
}

fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();
    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error)
    };

    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error)
    };

    Ok(string)
}


fn main() {
    let person = Person {
        first: "Ikuo".to_string(),
        middle: None,
        last: "Suyama".to_string(),
    };

    let full_name = build_full_name(&person);
    println!("Hello, {}", full_name);

    let middle_person = Person {
        first: "Ikuo".to_string(),
        middle: Some("James".to_string()),
        last: "Suyama".to_string(),
    };
    let full_name_middle = build_full_name(&middle_person);
    println!("Hello, {}", full_name_middle);

    if read_file_contents(PathBuf::from("src/main.rs")).is_ok() {
        println!("The program found the main file.");
    }

    if let Ok(contents) = read_file_contents(PathBuf::from("src/main.rs")) {
        println!("Contents: {}", contents);
    }
}
