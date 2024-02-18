use bevy::{
    asset::AssetServer,
    audio::{AudioBundle, PlaybackSettings},
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    sprite::{
        collide_aabb::{collide, Collision},
        Sprite, SpriteBundle,
    },
    time::Time,
    transform::components::Transform,
};

use crate::{
    components::{
        ball::{Ball, Velocity},
        brick::Brick,
        wall::Collider,
    },
    resources::{CollisionSound, Scoreboard},
};

pub fn ball_move(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time>) {
    let dt = time_step.delta_seconds();
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_tex = asset_server.load("textures/ball.png");
    let collision_sound = asset_server.load("sounds/collision.ogg");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Ball::STARTING_POSITION,
                ..Default::default()
            },
            sprite: Sprite {
                color: Ball::COLOUR,
                custom_size: Some(Ball::SIZE),
                ..Default::default()
            },
            texture: ball_tex,
            ..Default::default()
        },
        Ball::default(),
        Velocity(Ball::SPEED * Ball::INITIAL_DIRECTION),
    ));
    commands.insert_resource(CollisionSound(collision_sound));
}

pub fn ball_collision(
    mut ball: Query<(&mut Velocity, &Transform, &Ball)>,
    mut score: ResMut<Scoreboard>,
    collision_sound: Res<CollisionSound>,
    collider: Query<(Entity, &Transform, &Collider, Option<&Brick>)>,
    mut commands: Commands,
) {
    // ball velocity, ball transform, ball
    for (mut bv, bt, b) in &mut ball {
        // entity, transform, other, optionally brick (the thing the ball is colliding with)
        for (e, t, o, brick) in &collider {
            if let Some(collision) = collide(bt.translation, b.size, t.translation, o.size) {
                match collision {
                    Collision::Left if bv.x > 0.0 => bv.x *= -1.0,
                    Collision::Right if bv.x < 0.0 => bv.x *= -1.0,
                    Collision::Top if bv.y < 0.0 => bv.y *= -1.0,
                    Collision::Bottom if bv.y > 0.0 => bv.y *= -1.0,
                    _ => (),
                };
                if brick.is_some() {
                    score.score += 1;
                    println!("Score: {}", score.score);
                    commands.entity(e).despawn();
                    commands.spawn(AudioBundle {
                        source: collision_sound.clone(),
                        settings: PlaybackSettings::DESPAWN,
                    });
                }
            };
        }
    }
}
