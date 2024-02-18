use bevy::{ecs::component::Component, math::Vec2, render::color::Color};

#[derive(Component)]
pub struct Brick;

impl Brick {
    pub const SIZE: Vec2 = Vec2::new(100., 30.);
    pub const COLOUR: Color = Color::rgb(0.5, 0.5, 1.0);
    /// Get the gap between a brick and some other entity.
    /// Used for spawning
    pub const fn gap(between: Between) -> f32 {
        match between {
            Between::Paddle => 270.0,
            Between::Bricks => 5.0,
            Between::Ceiling => 20.0,
            Between::Sides => 20.0,
        }
    }
}

/// The Gap between a brick and some other entitiy
pub enum Between {
    Paddle,
    Bricks,
    Ceiling,
    Sides,
}
