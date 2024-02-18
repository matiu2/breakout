use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::Vec2,
    render::color::Color,
    sprite::SpriteBundle,
};

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

#[derive(Bundle)]
pub struct Wall {
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
}

impl Wall {
    pub const LEFT: f32 = -450.0;
    pub const RIGHT: f32 = 450.0;
    pub const BOTTOM: f32 = -300.0;
    pub const TOP: f32 = 300.0;

    pub const THICKNESS: f32 = 10.0;
    pub const INNER_SPACE: Vec2 = Vec2::new(Self::RIGHT - Self::LEFT, Self::TOP - Self::BOTTOM);
    // pub const BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
    // pub const BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
    pub const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
}
