use bevy::prelude::*;

use crate::structs::{plugins::UiPlugin, ui::FPSDisplay};

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init)
            .add_systems(Update, update);
    }
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/bigblue.ttf"),
        font_size: 40.0,
        color: Color::srgb(1.0, 1.0, 1.0),
    };
    commands.spawn((
        TextBundle::from_sections(
            [
            TextSection::new("FPS: ", text_style.clone()),
            TextSection::new("", text_style.clone()),
            ]
        ).with_text_justify(JustifyText::Left),
        FPSDisplay,
    ),
);
}

fn update(mut fps_display: Query<&mut Text, With<FPSDisplay>>, time: Res<Time>, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/bigblue.ttf"),
        font_size: 40.0,
        color: Color::srgb(1.0, 1.0, 1.0),
    };
    fps_display.single_mut().sections[1] = TextSection::new((1.0 / time.delta_seconds()).round().to_string(), text_style);
}