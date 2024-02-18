use bevy::{
    ecs::system::Commands,
    math::Vec2,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

use crate::components::{
    brick::{Between, Brick},
    paddle::Paddle,
    wall::{Collider, Wall},
};

pub fn setup(mut commands: Commands) {
    const BRICKS_WIDTH: f32 = Wall::INNER_SPACE.x - 2.0 * Brick::gap(Between::Sides);
    const BRICKS_HEIGHT: f32 =
        (Wall::TOP - Wall::BOTTOM) - Brick::gap(Between::Ceiling) - Brick::gap(Between::Paddle);

    tracing::info!("data: {}, {}, {}", Wall::INNER_SPACE.x, BRICKS_WIDTH, BRICKS_HEIGHT);
    const LEFT_BRICK: f32 = Wall::LEFT + Wall::THICKNESS + Wall::INNER_SPACE.x * 0.5
        - (BRICKS_WIDTH * 0.5)
        + (Brick::SIZE.x * 0.5);
    // const OFFSET_X: f32 = Wall::LEFT + Brick::gap(Between::Sides) + Brick::SIZE.x * 0.5;
    const BOTTOM_BRICK: f32 = Paddle::START_Y + Brick::gap(Between::Paddle) + Brick::SIZE.y * 0.5;

    let rows = (BRICKS_HEIGHT / (Brick::SIZE.y + Brick::gap(Between::Bricks))).floor() as i32;
    let columns = (BRICKS_WIDTH / (Brick::SIZE.x + Brick::gap(Between::Bricks))).floor() as i32;

    for row in 0..rows {
        for col in 0..columns {
            let brick_pos = Vec2::new(
                LEFT_BRICK + col as f32 * (Brick::SIZE.x + Brick::gap(Between::Bricks)),
                BOTTOM_BRICK + row as f32 * (Brick::SIZE.y + Brick::gap(Between::Bricks)),
            );

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: brick_pos.extend(0.0),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: Brick::COLOUR,
                        custom_size: Some(Brick::SIZE),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Brick,
                Collider { size: Brick::SIZE },
            ));
        }
    }
}
