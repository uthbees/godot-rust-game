use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    #[export]
    gravity: real,
    #[export]
    max_horizontal_speed: real,
    #[export]
    jump_speed: real,
    #[export]
    ground_friction_factor: real,
    #[export]
    air_friction_factor: real,
    #[export]
    acceleration_factor: real,
    #[export]
    max_air_jumps: i32,
    available_air_jumps: i32,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            gravity: 1000.0,
            max_horizontal_speed: 100.0,
            jump_speed: 1000.0,
            ground_friction_factor: 0.2,
            air_friction_factor: 0.1,
            acceleration_factor: 0.025,
            max_air_jumps: 1,
            available_air_jumps: 0,
        }
    }

    fn process(&mut self, delta: f64) {
        let mut velocity = self.base().get_velocity();
        velocity.y += delta as real * self.gravity;

        let input = Input::singleton();
        let left_pressed = input.is_action_pressed("ui_left");
        let right_pressed = input.is_action_pressed("ui_right");

        let mut turning_around = false;

        if left_pressed {
            velocity.x -= self.max_horizontal_speed * self.acceleration_factor;
            turning_around = velocity.x > 0.0;
        } else if right_pressed {
            velocity.x += self.max_horizontal_speed * self.acceleration_factor;
            turning_around = velocity.x < 0.0;
        }

        if turning_around || (!left_pressed && !right_pressed) {
            if self.base().is_on_floor() {
                velocity.x *= 1.0 - self.ground_friction_factor;
            } else {
                velocity.x *= 1.0 - self.air_friction_factor;
            }
        }

        if velocity.x < 5.0 && velocity.x > -5.0 {
            velocity.x = 0.0;
        }
        if velocity.x > self.max_horizontal_speed {
            velocity.x = self.max_horizontal_speed;
        } else if velocity.x < -self.max_horizontal_speed {
            velocity.x = -self.max_horizontal_speed;
        }

        if self.base().is_on_floor() {
            self.available_air_jumps = self.max_air_jumps;
        }

        if input.is_action_just_pressed("ui_jump") {
            if self.base().is_on_floor() {
                velocity.y = -self.jump_speed;
            } else if self.available_air_jumps > 0 {
                self.available_air_jumps -= 1;
                velocity.y = -self.jump_speed;
            }
        }

        // Update values
        let mut base = self.base_mut();
        base.set_velocity(velocity);
        base.move_and_slide();
    }
}
