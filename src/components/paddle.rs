use bevy::{ecs::component::Component, math::Vec2, render::color::Color};

use super::{brick::Brick, wall::Wall};

#[derive(Component)]
pub struct Paddle;

impl Paddle {
    // One brick up from the bottom
    pub const START_Y: f32 = Wall::BOTTOM + Brick::SIZE.y;
    pub const SIZE: Vec2 = Vec2::new(120.0, 20.0);
    pub const COLOUR: Color = Color::rgb(0.3, 0.3, 0.7);
    pub const SPEED: f32 = 500.0;
}
