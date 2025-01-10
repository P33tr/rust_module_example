
mod domain_types;
use crate::domain_types::*;

fn main() {
    println!("Hello, world!");

    let my_car = Car::build_car("dinky".to_string());
    println!("{}", my_car.name);

    let a_car = Car::new();
    println!("{}", a_car.name);

    let _ = my_car.start();
    a_car.start();
}
