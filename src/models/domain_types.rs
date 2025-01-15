
// we start with a structure
// just data no methods
pub struct Car {
    pub name: String
}

// we have a default constructor, and a build method that can
// build a car. We can put methods that are particular to
// all cars here like pump_up_tyres
impl Car {
    pub fn new() -> Self {
        Car{name:"default".to_string()}
    }

    pub fn build_car(_name: String) -> Self {
        Car {name: _name}
    }
    pub fn pump_up_tyres(&self) -> bool{
        true
    }
}

// we define a trait that applies to all vehicles
// and car is a vehicle. Equivalent to an interface in c#
// I am not sure if a car HAS to have an implementation of start
pub trait Vehicle {
   fn start(&mut self ) -> &mut Self;
    fn drive(&mut self ) -> &mut Self;
}

impl Vehicle for Car {
    fn start(&mut self) -> &mut Self {
        println!("started {}",self.name);
        self
    }
    fn drive(&mut self) -> &mut Self {
        println!("I am driving my  {}",self.name);
        self
    }
}