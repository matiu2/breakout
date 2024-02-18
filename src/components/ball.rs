use bevy::{
    ecs::component::Component,
    math::{Vec2, Vec3},
    prelude::{Deref, DerefMut},
    render::color::Color,
};

use super::{brick::Brick, paddle::Paddle, wall::Wall};

#[derive(Component, Deref, DerefMut)]
pub struct Ball {
    pub size: Vec2,
}

impl Default for Ball {
    fn default() -> Self {
        Self { size: Self::SIZE }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

impl Ball {
    pub const COLOUR: Color = Color::rgb(1.0, 0.5, 0.5);
    pub const STARTING_POSITION: Vec3 = Vec3::new(
        (Wall::RIGHT - Wall::LEFT) * 0.25,
        Paddle::START_Y + Brick::SIZE.y,
        0.0,
    );
    pub const SIZE: Vec2 = Vec2::new(30.0, 30.0);
    pub const SPEED: f32 = 400.0;
    pub const INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
}
