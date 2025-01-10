
mod domain_types;
use crate::domain_types::*;

fn main() {
    println!("Hello, world!");

    let mut  my_car = Car::build_car("dinky".to_string());
    println!("{}", my_car.name);

    let mut a_car = Car::new();
    println!("{}", a_car.name);

    let _ = my_car.start()
        .drive();

    a_car.start()
        .drive();
}
