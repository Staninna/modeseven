use crate::physics::CarInput;
use pix_win_loop::{Context, KeyCode};

pub struct Controls {
    w_pressed: bool,
    s_pressed: bool,
    a_pressed: bool,
    d_pressed: bool,
}

impl Controls {
    pub fn new() -> Self {
        Self {
            w_pressed: false,
            s_pressed: false,
            a_pressed: false,
            d_pressed: false,
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        // Update key states
        self.w_pressed = ctx.input.is_physical_key_down(KeyCode::KeyW);
        self.s_pressed = ctx.input.is_physical_key_down(KeyCode::KeyS);
        self.a_pressed = ctx.input.is_physical_key_down(KeyCode::KeyA);
        self.d_pressed = ctx.input.is_physical_key_down(KeyCode::KeyD);
    }

    pub fn get_car_input(&self) -> CarInput {
        // Calculate throttle
        let mut throttle = 0.0;
        if self.w_pressed { throttle += 1.0; }
        if self.s_pressed { throttle -= 1.0; }

        // Calculate steering
        let mut turn = 0.0;
        if self.a_pressed { turn += 1.0; }
        if self.d_pressed { turn -= 1.0; }

        // Calculate brake
        let mut brake = 0.0;
        if self.w_pressed { brake -= 1.0; }
        if self.s_pressed { brake += 1.0; }
        
        CarInput::new(throttle, turn, brake)
    }
}