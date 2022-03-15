#[derive(Debug)]
struct KeyPress(String, char);

#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress),
}

fn main() {
    let we_load = WebEvent::WELoad(true);
    let click = MouseClick {x:100, y:250};
    let we_click = WebEvent::WEClick(click);
    println!("{:#?}\n{:#?}", we_load, we_click);
    println!("{:?}", we_load)
}
