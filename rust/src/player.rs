use godot::prelude::*;
use godot::engine::RigidBody2D;
use godot::engine::IRigidBody2D;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Player {
	speed: f32,
	angular_speed: f32,

	base: Base<RigidBody2D>,
}

#[godot_api]
impl IRigidBody2D for Player {
	fn init(base: Base<RigidBody2D>) -> Self {
		godot_print!("Hello, World!");

		Self {
			speed: 400.0,
			angular_speed: std::f64::consts::PI as f32,
			base,
		}
	}

	fn physics_process(&mut self, delta: f64) {
		let up_force = Vector2::new(0.0, -self.speed);
		let down_force = Vector2::new(0.0, self.speed);
		let left_force = Vector2::new(-self.speed, 0.0);
		let right_force = Vector2::new(self.speed, 0.0);

		let input = Input::singleton();
		if input.is_action_pressed("move_right".into()) {
			self.base_mut().apply_force(right_force);
		}
		if input.is_action_pressed("move_left".into()) {
			self.base_mut().apply_force(left_force);
		}
		if input.is_action_pressed("move_up".into()) {
			self.base_mut().apply_force(up_force);
		}
		if input.is_action_pressed("move_down".into()) {
			self.base_mut().apply_force(down_force);
		}

	}
}