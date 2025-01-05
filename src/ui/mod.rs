use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::structs::{
    input::ParsedInput,
    player::AvailableResources,
    plugins::UiPlugin,
    ui::{HealthDisplay, MaterialsDisplay, StatusDisplay},
    world::Material,
};

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init).add_systems(
            Update,
            (
                update_material_display,
                toggle_inventory,
            ),
        );
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
                        visibility: Visibility::Hidden,
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
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(150.0),
                            border: UiRect::all(Val::Px(10.)),
                            flex_direction: FlexDirection::Column,
                            align_self: AlignSelf::End,
                            ..default()
                        },
                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.9).into(),
                        border_color: Color::srgba(0.0, 0.0, 0.0, 0.9).into(),
                        ..default()
                    },
                    StatusDisplay,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_sections([
                            TextSection::new("HP: ", text_style.clone()),
                            TextSection::new("", text_style.clone()),
                        ]),
                        HealthDisplay,
                    ));
                });
        });
}

fn toggle_inventory(
    keyboard_input: Res<ParsedInput>,
    mut material_display: Query<&mut Visibility, With<MaterialsDisplay>>,
) {
    if keyboard_input.toggle_inventory {
        *material_display.single_mut() = match *material_display.single() {
            Visibility::Hidden => Visibility::Inherited,
            Visibility::Inherited => Visibility::Hidden,
            Visibility::Visible => Visibility::Hidden,
        }
    }
}

fn update_material_display(
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
        let amount = available_resources.resource_map.get(material).unwrap();
        text.sections[1] = TextSection::new(amount.to_string(), text_style.clone());
        let color;
        if *amount == 0 {
            color = Color::srgba(0.0, 0.0, 0.0, 0.0);
        } else {
            color = Color::srgb(1.0, 1.0, 1.0);
        }
        for section in &mut text.sections {
            section.style.color = color;
        }
    });
}
