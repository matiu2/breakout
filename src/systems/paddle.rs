use bevy::{
    ecs::{
        query::With,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, Input},
    sprite::{Sprite, SpriteBundle},
    time::Time,
    transform::components::Transform,
};

use crate::components::{
    paddle::Paddle,
    wall::{Collider, Wall},
};

pub fn setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Paddle::COLOUR,
                custom_size: Some(Paddle::SIZE),
                ..Default::default()
            },
            transform: Transform {
                translation: bevy::math::vec3(0., Paddle::START_Y, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle,
        Collider { size: Paddle::SIZE },
    ));
}

/// Moves the paddle if the user is pressing the right keys
pub fn move_paddle(
    input: Res<Input<KeyCode>>,
    time_step: Res<Time>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();

    let mut direction = 0.0;
    if input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }
    if input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let new_x =
        paddle_transform.translation.x + direction * Paddle::SPEED * time_step.delta_seconds();
    let new_x = new_x.min(Wall::RIGHT - (Wall::THICKNESS + Paddle::SIZE.x) * 0.5);
    let new_x = new_x.max(Wall::LEFT + (Wall::THICKNESS + Paddle::SIZE.x) * 0.5);

    paddle_transform.translation.x = new_x;
}
