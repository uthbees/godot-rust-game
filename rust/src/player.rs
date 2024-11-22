use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::prelude::*;

const GRAVITY: real = 1000.0;
const WALK_SPEED: real = 1000.0;
const JUMP_SPEED: real = 1000.0;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self { base }
    }

    fn process(&mut self, delta: f64) {
        // Update velocity
        let mut velocity = self.base().get_velocity();
        velocity.y += delta as real * GRAVITY;

        let input = Input::singleton();
        if input.is_action_pressed("ui_left") {
            velocity.x = -WALK_SPEED;
        } else if input.is_action_pressed("ui_right") {
            velocity.x = WALK_SPEED;
        } else {
            velocity.x = 0.0;
        }

        if input.is_action_pressed("ui_up") && self.base().is_on_floor() {
            velocity.y = -JUMP_SPEED;
        }

        self.base_mut().set_velocity(velocity);

        // Update position
        self.base_mut().move_and_slide();
    }
}
