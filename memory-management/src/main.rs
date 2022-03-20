#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn erase(_: String) {}

fn main() {
    let text = String::from("The quick borwn fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);

    //     error[E0505]: cannot move out of `text` because it is borrowed
    //   --> src/main.rs:11:11
    //    |
    // 8  |     let fox = Highlight(&text[4..19]);
    //    |                          ---- borrow of `text` occurs here
    // ...
    // 11 |     erase(text);
    //    |           ^^^^ move out of `text` occurs here
    // 12 |
    // 13 |     println!("{:?} {:?}", fox, dog);
    //    |                           --- borrow later used here
    // erase(text);

    println!("{:?} {:?}", fox, dog);

    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));

    assert_eq!(names, vec!["Joe".to_string(),])
}

fn copy_and_return(vector: &mut Vec<String>, value: &str) -> String {
    vector.push(String::from(value));
    String::from(value)
}
