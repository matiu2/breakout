use bevy::{
    ecs::system::{Commands, Query, Res},
    text::{Text, TextSection, TextStyle},
    ui::{node_bundles::TextBundle, PositionType, Style},
};

use crate::resources::Scoreboard;

pub fn setup(mut commands: Commands) {
    commands.spawn((TextBundle::from_sections([
        TextSection::new(
            "Score: ",
            TextStyle {
                font_size: Scoreboard::FONT_SIZE,
                color: Scoreboard::TEXT_COLOUR,
                ..Default::default()
            },
        ),
        TextSection::from_style(TextStyle {
            font_size: Scoreboard::FONT_SIZE,
            color: Scoreboard::SCORE_COLOUR,
            ..Default::default()
        }),
    ])
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: Scoreboard::TEXT_PADDING,
        left: Scoreboard::TEXT_PADDING,
        ..Default::default()
    }),));
}

pub fn update(score: Res<Scoreboard>, mut query: Query<&mut Text>) {
    for mut text in &mut query {
        text.sections[1].value = score.score.to_string();
    }
}
