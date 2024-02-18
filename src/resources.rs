use bevy::{
    asset::Handle,
    audio::AudioSource,
    ecs::system::Resource,
    prelude::{Deref, DerefMut},
    render::color::Color,
    ui::Val,
};

#[derive(Resource, Clone, Copy, Default)]
pub struct Scoreboard {
    pub score: usize,
}

impl Scoreboard {
    pub const FONT_SIZE: f32 = 40.0;
    pub const TEXT_PADDING: Val = Val::Px(5.0);
    pub const TEXT_COLOUR: Color = Color::rgb(0.5, 0.5, 1.0);
    pub const SCORE_COLOUR: Color = Color::rgb(1.0, 0.5, 0.5);
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CollisionSound(pub Handle<AudioSource>);
