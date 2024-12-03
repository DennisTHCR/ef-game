use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::structs::{
    player::AvailableResources, plugins::UiPlugin, ui::MaterialsDisplay, world::Material,
};

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init).add_systems(Update, update);
    }
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/bigblue.ttf"),
        font_size: 32.0,
        color: Color::srgb(1.0, 1.0, 1.0),
    };
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(256.0),
                            border: UiRect::all(Val::Px(10.)),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.9).into(),
                        border_color: Color::srgba(0.0, 0.0, 0.0, 0.9).into(),
                        ..default()
                    },
                    MaterialsDisplay,
                ))
                .with_children(|parent| {
                    Material::iter().for_each(|material| {
                        if material != Material::Grass && material != Material::Spawner {
                            parent.spawn((
                                TextBundle::from_sections([
                                    TextSection::new(format!("{material}: "), text_style.clone()),
                                    TextSection::new("0", text_style.clone()),
                                ]),
                                material,
                            ));
                        }
                    });
                });
        });
}

fn update(
    children: Query<&Children, With<MaterialsDisplay>>,
    mut text_boxes: Query<(&mut Text, &Material)>,
    asset_server: Res<AssetServer>,
    available_resources: Res<AvailableResources>,
) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/bigblue.ttf"),
        font_size: 32.0,
        color: Color::srgb(1.0, 1.0, 1.0),
    };
    children.single().iter().for_each(|&child| {
        let (mut text, material) = text_boxes.get_mut(child).unwrap();
        let amount = available_resources
            .resource_map
            .get(material)
            .unwrap();
        text.sections[1] = TextSection::new(
            amount.to_string(),
            text_style.clone(),
        );
        let color;
        if *amount == 0 {
            color = Color::srgb(0.3, 0.3, 0.3);
        } else {
            color = Color::srgb(1.0, 1.0, 1.0);
        }
        for section in &mut text.sections {
            section.style.color = color;
        }
    });
}
