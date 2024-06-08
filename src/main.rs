mod slot;

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitSettings};
use slot::{Dir, SelectSlot, Ship, ShipType};

pub const WIDTH: u8 = 21;
pub const HEIGHT: u8 = 8;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    Placing,
    Playing,
}

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, drag_ship)
        .run();
}

#[allow(clippy::needless_pass_by_value, clippy::cast_lossless)]
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(80.),
                        height: Val::Percent(50.),
                        top: Val::Percent(40.),
                        left: Val::Percent(10.),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for y in 0..HEIGHT {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100. / HEIGHT as f32),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                for x in 0..WIDTH {
                                    parent.spawn((
                                        ButtonBundle {
                                            style: Style {
                                                width: Val::Percent(90. / WIDTH as f32),
                                                height: Val::Percent(90.),
                                                margin: UiRect {
                                                    left: Val::Auto,
                                                    right: Val::Auto,
                                                    top: Val::Auto,
                                                    bottom: Val::Auto,
                                                },
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::Rgba {
                                                red: 40.,
                                                green: 40.,
                                                blue: 40.,
                                                alpha: 1.,
                                            }),
                                            ..default()
                                        },
                                        SelectSlot { x, y },
                                    ));
                                }
                            });
                    }
                });
            spawn_ship(parent, ShipType::PatrolBoat, &asset_server);
            spawn_ship(parent, ShipType::Submarine, &asset_server);
            spawn_ship(parent, ShipType::Destroyer, &asset_server);
            spawn_ship(parent, ShipType::Battleship, &asset_server);
            spawn_ship(parent, ShipType::Carrier, &asset_server);
        });
}

#[allow(clippy::cast_lossless)]
fn spawn_ship(parent: &mut ChildBuilder, ship: ShipType, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(80. / WIDTH as f32),
                    height: Val::Percent(50. / HEIGHT as f32 * ship.len() as f32),
                    left: Val::Percent(80. / WIDTH as f32 * ship.id() as f32),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.)),
                ..default()
            },
            Ship {
                placed: false,
                x: 0,
                y: 0,
                dir: Dir::Vertical,
                type_: ship,
            },
        ))
        .with_children(|parent| {
            parent.spawn(ship.ressource(asset_server));
        });
}

static mut SELECTED: Option<u8> = None;

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
fn drag_ship(
    mut ships: Query<(&Interaction, &mut Style, &mut Ship), (With<Button>, With<Ship>)>,
    slots: Query<(&Interaction, &SelectSlot), (With<Button>, With<SelectSlot>)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    for (interaction, _, mut ship) in &mut ships {
        let nid = ship.type_.id();
        if *interaction == Interaction::Pressed {
            unsafe {
                SELECTED = Some(nid);
            }
        } else if let Some(id) = unsafe { SELECTED } {
            if id == nid {
                unsafe {
                    SELECTED = None;
                }
                ship.placed = false;
                for (interaction, slot) in &slots {
                    if *interaction == Interaction::Hovered {
                        (ship.x, ship.y, ship.placed) = (slot.x, slot.y, true);
                        break;
                    }
                }
            }
        }
    }

    if let Some(id) = unsafe { SELECTED } {
        for (_, mut style, ship) in &mut ships {
            if id == ship.type_.id() {
                let Some(Vec2 { x, y }) = q_windows.single().cursor_position() else {
                    return;
                };
                let Val::Percent(height) = style.height else {
                    return;
                };
                let Val::Percent(width) = style.width else {
                    return;
                };

                style.top = Val::Px(y - height * q_windows.single().height() / 200.);
                style.left = Val::Px(x - width * q_windows.single().width() / 200.);
            }
        }
    }
}
