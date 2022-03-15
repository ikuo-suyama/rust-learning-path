use crate::Transmission::Manual;

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

// Declare Car struct to describe vehicle with four named fields
#[derive(Debug)]
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

// Declare enum for Car transmission type
#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    let car: Car = Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 1
    };
    car
}

fn main() {
    let car = car_factory("car".to_string(), Manual, true);
    print!("{:#?}", car);

    let we_load = WebEvent::WELoad(true);
    let click = MouseClick {x:100, y:250};
    let we_click = WebEvent::WEClick(click);
    println!("{:#?}\n{:#?}", we_load, we_click);
    println!("{:?}", we_load)
}
