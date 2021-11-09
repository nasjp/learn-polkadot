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
    Car {
        color,
        transmission,
        convertible,
        mileage: 0,
    }
}

fn main() {
    let car1 = car_factory(String::from("red"), Transmission::Manual, true);
    let car2 = car_factory(String::from("blue"), Transmission::SemiAuto, true);
    let car3 = car_factory(String::from("green"), Transmission::Automatic, true);

    println!("{:?}", car1);
    println!("{:?}", car2);
    println!("{:?}", car3);
}
