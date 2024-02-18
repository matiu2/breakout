use bevy::prelude::*;
use breakout::resources::Scoreboard;
use breakout::systems::ball::{ball_collision, ball_move};
use breakout::systems::paddle::move_paddle;
use breakout::systems::{ball, brick, camera, paddle, score, wall};
// https://www.youtube.com/watch?v=E9SzRc9HkOg

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Scoreboard::default())
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, ball::setup)
        .add_systems(Startup, camera::setup)
        .add_systems(Startup, paddle::setup)
        .add_systems(Startup, wall::setup)
        .add_systems(Startup, brick::setup)
        .add_systems(Startup, score::setup)
        .add_systems(FixedUpdate, move_paddle)
        .add_systems(FixedUpdate, ball_collision)
        .add_systems(FixedUpdate, ball_move)
        .add_systems(FixedUpdate, score::update)
        .run();
}
