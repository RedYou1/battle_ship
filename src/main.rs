mod slot;

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitSettings};
use slot::ShipType;

pub const WIDTH: usize = 21;
pub const HEIGHT: usize = 6;

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

#[allow(clippy::needless_pass_by_value)]
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
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            });
            for ship in [
                ShipType::PatrolBoat,
                ShipType::Submarine,
                ShipType::Destroyer,
                ShipType::Battleship,
                ShipType::Carrier,
            ] {
                spawn_ship(parent, ship, &asset_server);
            }
        });
}

#[allow(clippy::cast_lossless)]
fn spawn_ship(parent: &mut ChildBuilder, ship: ShipType, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(60.0),
                    height: Val::Px(ship.len() as f32 * 50.),
                    left: Val::Px(ship.id() as f32 * 100.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.)),
                ..default()
            },
            ship,
        ))
        .with_children(|parent| {
            parent.spawn(ship.ressource(asset_server));
        });
}

// fn spawn_board(parent: &mut ChildBuilder) {
//     for i in 0..HEIGHT {
//         parent.spawn((ButtonBundle {
//             style: Style {
//                 flex_direction: FlexDirection::Row,
//                 align_items: AlignItems::Center,
//                 ..default()
//             },
//             ..default()
//         }, Slot));
//     }
// }

static mut SELECTED: Option<u8> = None;

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
fn drag_ship(
    mut ships: Query<(&Interaction, &mut Style, &ShipType), (With<Button>, With<ShipType>)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    for (interaction, _, ship) in &mut ships {
        let nid = ship.id();
        if *interaction == Interaction::Pressed {
            unsafe {
                SELECTED = Some(nid);
            }
        } else if let Some(id) = unsafe { SELECTED } {
            if id == nid {
                unsafe {
                    SELECTED = None;
                }
            }
        }
    }

    if let Some(id) = unsafe { SELECTED } {
        for (_, mut style, ship) in &mut ships {
            if id == ship.id() {
                let Some(position) = q_windows.single().cursor_position() else {
                    return;
                };
                let Val::Px(height) = style.height else {
                    return;
                };
                let Val::Px(width) = style.width else {
                    return;
                };
                style.top = Val::Px(position.y - height / 2.);
                style.left = Val::Px(position.x - width / 2.);
            }
        }
    }
}
