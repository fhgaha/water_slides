//! This example illustrates the various features of Bevy UI.

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    color::palettes::basic::LIME,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    winit::WinitSettings,
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_scroll);

    #[cfg(feature = "bevy_dev_tools")]
    {
        app.add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
            .add_systems(Update, toggle_overlay);
    }

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));

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
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::srgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(5.)),
                                row_gap: Val::Px(5.),
                                ..default()
                            },
                            background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                TextBundle::from_section(
                                    "Text Example",
                                    TextStyle {
                                        // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        ..default()
                                    },
                                ),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));

                            #[cfg(feature = "bevy_dev_tools")]
                            // Debug overlay text
                            parent.spawn((
                                TextBundle::from_section(
                                    "Press Space to enable debug outlines.",
                                    TextStyle {
                                        // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        ..default()
                                    },
                                ),
                                Label,
                            ));

                            #[cfg(not(feature = "bevy_dev_tools"))]
                            parent.spawn((
                                TextBundle::from_section(
                                    "Try enabling feature \"bevy_dev_tools\".",
                                    TextStyle {
                                        // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        ..default()
                                    },
                                ),
                                Label,
                            ));
                        });
                });
            // right vertical fill
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.),
                        ..default()
                    },
                    background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        TextBundle::from_section(
                            "Scrolling list",
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.,
                                ..default()
                            },
                        ),
                        Label,
                    ));
                    // List with hidden overflow
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                height: Val::Percent(50.),
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            background_color: Color::srgb(0.10, 0.10, 0.10).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    ScrollingList::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .with_children(|parent| {
                                    // List items
                                    for i in 0..30 {
                                        parent.spawn((
                                            TextBundle::from_section(
                                                format!("Item {i}"),
                                                TextStyle {
                                                    // font: asset_server
                                                    //     .load("fonts/FiraSans-Bold.ttf"),
                                                    ..default()
                                                },
                                            ),
                                            Label,
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                        ));
                                    }
                                });
                        });
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(210.),
                        bottom: Val::Px(10.),
                        border: UiRect::all(Val::Px(20.)),
                        ..default()
                    },
                    border_color: LIME.into(),
                    background_color: Color::srgb(0.4, 0.4, 1.).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: Color::srgb(0.8, 0.8, 1.).into(),
                        ..default()
                    });
                });
            // render order test: reddest in the back, whitest in the front (flex center)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(100.0),
                                height: Val::Px(100.0),
                                ..default()
                            },
                            background_color: Color::srgb(1.0, 0.0, 0.).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(NodeBundle {
                                style: Style {
                                    // Take the size of the parent node.
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(20.),
                                    bottom: Val::Px(20.),
                                    ..default()
                                },
                                background_color: Color::srgb(1.0, 0.3, 0.3).into(),
                                ..default()
                            });
                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(40.),
                                    bottom: Val::Px(40.),
                                    ..default()
                                },
                                background_color: Color::srgb(1.0, 0.5, 0.5).into(),
                                ..default()
                            });
                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(60.),
                                    bottom: Val::Px(60.),
                                    ..default()
                                },
                                background_color: Color::srgb(1.0, 0.7, 0.7).into(),
                                ..default()
                            });
                            // alpha test
                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(80.),
                                    bottom: Val::Px(80.),
                                    ..default()
                                },
                                background_color: Color::srgba(1.0, 0.9, 0.9, 0.4).into(),
                                ..default()
                            });
                        });
                });
            // bevy logo (flex center)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // bevy logo (image)
                    // A `NodeBundle` is used to display the logo the image as an `ImageBundle` can't automatically
                    // size itself with a child node present.
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(500.0),
                                    height: Val::Px(125.0),
                                    margin: UiRect::top(Val::VMin(5.)),
                                    ..default()
                                },
                                ..default()
                            },
                            // UiImage::new(asset_server.load("branding/bevy_logo_dark_big.png")),
							UiImage::default()
                        ))
                        .with_children(|parent| {
                            // alt text
                            // This UI node takes up no space in the layout and the `Text` component is used by the accessibility module
                            // and is not rendered.
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        display: Display::None,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                Text::from_section("Bevy logo", TextStyle::default()),
                            ));
                        });
                });
        });
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}

#[cfg(feature = "bevy_dev_tools")]
// The system that will enable/disable the debug outlines around the nodes
fn toggle_overlay(
    input: Res<ButtonInput<KeyCode>>,
    mut options: ResMut<bevy::dev_tools::ui_debug_overlay::UiDebugOptions>,
) {
    info_once!("The debug outlines are enabled, press Space to turn them on/off");
    if input.just_pressed(KeyCode::Space) {
        // The toggle method will enable the debug_overlay if disabled and disable if enabled
        options.toggle();
    }
}
