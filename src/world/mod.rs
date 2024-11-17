pub use car::{Car, CarInput};
use crate::input::Inputs;

mod car;

pub struct World {
    pub cars: [Car; 2],
}

impl World {
    pub fn new() -> Self {
        let car1 = Car::new(1024.0 / 3.0, 1024.0 / 3.0);
        let car2 = Car::new(1024.0 / 3.0, 1024.0 / 3.0);
        
        Self {
            cars: [car1, car2],
        }
    }

    pub fn update(&mut self, inputs: &Inputs, dt: f32) {
        // update cars
        let [car1, car2] = &mut self.cars;
        let [car1_input, car2_input] = inputs.get_car_inputs();
        car1.update(dt, car1_input.throttle, car1_input.brake, car1_input.turn);
        car2.update(dt, car2_input.throttle, car2_input.brake, car2_input.turn);
    }
}