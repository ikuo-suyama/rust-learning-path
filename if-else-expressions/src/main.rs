use crate::Age::New;
use crate::Transmission::Manual;

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Old,
}

fn car_quality(miles: u32) -> (Age, u32) {
    let quality = (New, miles);
    quality
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

fn main() {
    let colors = ["Blue", "Green", "Red", "Silver"];
    let mut car: Car = Car {
        color: "None".to_string(),
        motor: Manual,
        roof: false,
        age: (New, 0),
    };

    let mut engine: Transmission = Manual;
    car = car_factory(colors[0].to_string(), engine, true, 0);
    println!("Car order1: {:?}", car);
}
