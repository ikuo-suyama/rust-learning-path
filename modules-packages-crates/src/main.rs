mod dir;
mod mysecondmodule;

mod myfirstmodule {
    #[derive(Debug)]
    pub struct User {
        pub username: String,
    }

    impl User {
        pub fn new(username: &str) -> User {
            User {
                username: username.to_string(),
            }
        }
    }
}

mod text_processing {
    pub mod letters {
        pub fn count_letters(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_alphabetic()).count()
        }
    }

    pub mod numbers {
        pub fn count_numbers(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_numeric()).count()
        }
    }
}

fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    (
        text_processing::letters::count_letters(text),
        text_processing::numbers::count_numbers(text),
    )
}

fn main() {
    let user = myfirstmodule::User::new("Duke");

    println!("Hello, {:?}!", user);
    println!("Hello, {}!", user.username);

    let pass = mysecondmodule::Password::new("hogehoge");
    println!("{}", pass.password);

    println!("{:?}", dir::lib::Foo);

    println!("{:?}", count_letters_and_numbers("123aaaaa"))
}
