mod mysecondmodule;
mod dir;

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

fn main() {
    let user = myfirstmodule::User::new("Duke");

    println!("Hello, {:?}!", user);
    println!("Hello, {}!", user.username);

    let pass = mysecondmodule::Password::new("hogehoge");
    println!("{}", pass.password);

    println!("{:?}", dir::lib::Foo)
}
