use godot::classes::ISprite2D;
use godot::classes::Sprite2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        #[allow(clippy::cast_possible_truncation)]
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
    }
}
