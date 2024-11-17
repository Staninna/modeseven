use pix_win_loop::{Context, KeyCode};
use crate::world::CarInput;

pub struct Inputs {
    // car 1
    w: bool,
    s: bool,
    a: bool,
    d: bool,
    // car 2
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            w: false,
            s: false,
            a: false,
            d: false,
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    pub fn update(&mut self, ctx: &Context) -> &Self {
        // Update key states
        self.w = ctx.input.is_physical_key_down(KeyCode::KeyW);
        self.s = ctx.input.is_physical_key_down(KeyCode::KeyS);
        self.a = ctx.input.is_physical_key_down(KeyCode::KeyA);
        self.d = ctx.input.is_physical_key_down(KeyCode::KeyD);
        self.up = ctx.input.is_physical_key_down(KeyCode::ArrowUp);
        self.down = ctx.input.is_physical_key_down(KeyCode::ArrowDown);
        self.left = ctx.input.is_physical_key_down(KeyCode::ArrowLeft);
        self.right = ctx.input.is_physical_key_down(KeyCode::ArrowRight);
        
        self
    }
    
    // helper method to get car input for both cars
    /// Returns an array of car inputs for both cars
    /// The first element is for car 1, the second for car 2
    pub fn get_car_inputs(&self) -> [CarInput; 2] {
        let car1_input = self.get_car1_input();
        let car2_input = self.get_car2_input();
        [car1_input, car2_input]
    }
    
    fn get_car1_input(&self) -> CarInput {
        // Calculate throttle
        let mut throttle = 0.0;
        if self.w { throttle += 1.0; }
        if self.s { throttle -= 1.0; }

        // Calculate steering
        let mut turn = 0.0;
        if self.a { turn += 1.0; }
        if self.d { turn -= 1.0; }

        // Calculate brake
        let mut brake = 0.0;
        if self.w { brake -= 1.0; }
        if self.s { brake += 1.0; }
        
        CarInput::new(throttle, turn, brake)
    }
    
    fn get_car2_input(&self) -> CarInput {
        // Calculate throttle
        let mut throttle = 0.0;
        if self.up { throttle += 1.0; }
        if self.down { throttle -= 1.0; }

        // Calculate steering
        let mut turn = 0.0;
        if self.left { turn += 1.0; }
        if self.right { turn -= 1.0; }

        // Calculate brake
        let mut brake = 0.0;
        if self.up { brake -= 1.0; }
        if self.down { brake += 1.0; }
        
        CarInput::new(throttle, turn, brake)
    }
}