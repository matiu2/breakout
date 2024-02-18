use crate::components::wall::{Collider, Wall};
use bevy::{
    ecs::system::Commands,
    math::{vec2, vec3, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

fn wall(size: Vec2, translation: Vec3) -> Wall {
    Wall {
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation,
                ..Default::default()
            },
            sprite: Sprite {
                color: Wall::WALL_COLOR,
                custom_size: Some(size),
                ..Default::default()
            },
            ..Default::default()
        },
        collider: Collider { size },
    }
}

pub fn setup(mut commands: Commands) {
    let vertical_wall_size = vec2(Wall::THICKNESS, Wall::INNER_SPACE.y + Wall::THICKNESS);
    let horizontal_wall_size = vec2(Wall::INNER_SPACE.x + Wall::THICKNESS, Wall::THICKNESS);
    // Left wall
    commands.spawn(wall(vertical_wall_size, vec3(Wall::LEFT, 0.0, 0.0)));
    // Right wall
    commands.spawn(wall(vertical_wall_size, vec3(Wall::RIGHT, 0.0, 0.0)));
    // Bottom wall
    commands.spawn(wall(horizontal_wall_size, vec3(0.0, Wall::BOTTOM, 0.0)));
    // Top wall
    commands.spawn(wall(horizontal_wall_size, vec3(0.0, Wall::TOP, 0.0)));
}
