mod slot;

use std::f32::consts::PI;

use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    window::PrimaryWindow,
    winit::WinitSettings,
};
use slot::{Dir, Overlay, Ship, ShipType};

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
                width: Val::Percent(100.),
                height: Val::Percent(100.),
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
                    for _ in 0..HEIGHT {
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
                                for _ in 0..WIDTH {
                                    parent.spawn(NodeBundle {
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
                                    });
                                }
                            });
                    }
                });
            for i in 0..5 {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(72. / WIDTH as f32),
                            height: Val::Percent(45. / HEIGHT as f32),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::Rgba {
                            red: 0.,
                            green: 40.,
                            blue: 0.,
                            alpha: 0.5,
                        }),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    Overlay { id: i },
                ));
            }
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
                background_color: BackgroundColor(Color::rgba(1., 0., 0., 0.)),
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

#[allow(
    clippy::needless_pass_by_value,
    clippy::type_complexity,
    clippy::cast_lossless
)]
fn drag_ship(
    mut transforms: Query<(&mut Transform, &mut Style), (Without<Overlay>, Without<Ship>)>,
    mut evr_click: EventReader<MouseButtonInput>,
    mut ships: Query<
        (&mut Children, &Interaction, &mut Style, &mut Ship),
        (With<Button>, Without<Overlay>, With<Ship>),
    >,
    mut overlay: Query<
        (&mut Visibility, &mut Style, &Overlay),
        (With<Node>, Without<Ship>, With<Overlay>),
    >,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let right = evr_click.read().last().map_or(false, |button| {
        button.state == ButtonState::Released && button.button == MouseButton::Right
    });
    for (childs, interaction, mut style, mut ship) in &mut ships {
        let nid = ship.type_.id();
        if right && *interaction == Interaction::Hovered {
            let (mut image, mut image_style) = transforms
                .get_mut(*childs.first().expect("have image"))
                .expect("have image2");
            image.rotate_local_z(PI / 2.);
            image.scale *= match ship.dir {
                Dir::Vertical => ship.type_.len() as f32,
                Dir::Horizontal => 1. / ship.type_.len() as f32,
            };
            let ratio = q_windows.single().width() / q_windows.single().height();
            (
                ship.placed,
                ship.dir,
                image_style.left,
                style.width,
                style.height,
            ) = match ship.dir {
                Dir::Vertical => (
                    ship.placed && ship.x + ship.type_.len() < WIDTH,
                    Dir::Horizontal,
                    Val::Percent(50.),
                    Val::Percent(50. / HEIGHT as f32 * ship.type_.len() as f32) * (1. / ratio),
                    Val::Percent(80. / WIDTH as f32) * ratio,
                ),
                Dir::Horizontal => (
                    ship.placed && ship.y + ship.type_.len() < HEIGHT,
                    Dir::Vertical,
                    Val::Percent(0.),
                    Val::Percent(80. / WIDTH as f32),
                    Val::Percent(50. / HEIGHT as f32 * ship.type_.len() as f32),
                ),
            };
            continue;
        }
        if *interaction == Interaction::Pressed {
            ship.placed = false;
            unsafe {
                SELECTED = Some(nid);
            }

            for (mut vis, _, _) in &mut overlay {
                *vis = Visibility::Hidden;
            }
            let Some(Vec2 { x, y }) = q_windows.single().cursor_position() else {
                return;
            };
            let Val::Percent(height) = style.height else {
                return;
            };
            let Val::Percent(width) = style.width else {
                return;
            };

            let mut top = (y - height * q_windows.single().height() / 200.)
                / q_windows.single().height()
                * 100.;
            let mut left =
                (x - width * q_windows.single().width() / 200.) / q_windows.single().width() * 100.;

            style.top = Val::Percent(top);
            style.left = Val::Percent(left);

            top += height / 2.;
            left += width / 2.;

            if !(40. ..=90.).contains(&top) || !(10. ..=90.).contains(&left) {
                return;
            }

            let Some((top, left)) = update_ship_coord(left, top, &mut ship) else {
                return;
            };

            for (mut vis, mut style, ov) in &mut overlay {
                if ov.id < ship.type_.len() {
                    *vis = Visibility::Visible;
                    match ship.dir {
                        Dir::Horizontal => {
                            style.top = Val::Percent(top);
                            style.left =
                                Val::Percent(left + ov.id as f32 * width / ship.type_.len() as f32);
                        }
                        Dir::Vertical => {
                            style.top =
                                Val::Percent(top + ov.id as f32 * height / ship.type_.len() as f32);
                            style.left = Val::Percent(left);
                        }
                    }
                }
            }

            return;
        } else if let Some(id) = unsafe { SELECTED } {
            if id == nid {
                unsafe {
                    SELECTED = None;
                }
                let Val::Percent(mut top) = style.top else {
                    return;
                };
                let Val::Percent(mut left) = style.left else {
                    return;
                };
                let Val::Percent(width) = style.width else {
                    return;
                };
                let Val::Percent(height) = style.height else {
                    break;
                };
                left += width / 2.;
                top += height / 2.;

                if !(40. ..=90.).contains(&top) || !(10. ..=90.).contains(&left) {
                    return;
                }
                let Some((top, left)) = update_ship_coord(left, top, &mut ship) else {
                    return;
                };

                ship.placed = true;
                for (mut vis, _, _) in &mut overlay {
                    *vis = Visibility::Hidden;
                }
                style.top = Val::Percent(top);
                style.left = Val::Percent(left);

                return;
            }
        }
    }
}

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_lossless,
    clippy::cast_possible_wrap
)]
fn update_ship_coord(left: f32, top: f32, ship: &mut Ship) -> Option<(f32, f32)> {
    ship.x = ((left - 10.) / 80. * (WIDTH + 1) as f32 - 0.5).floor() as u8;
    ship.y = ((top - 40.) / 50. * (HEIGHT + 1) as f32 - 0.5).floor() as u8;

    match ship.dir {
        Dir::Horizontal => {
            if ship.x
                < match ship.type_ {
                    ShipType::PatrolBoat => 0,
                    ShipType::Submarine | ShipType::Destroyer => 1,
                    ShipType::Battleship | ShipType::Carrier => 2,
                }
            {
                return None;
            }
            if ship.x
                > WIDTH
                    - match ship.type_ {
                        ShipType::PatrolBoat | ShipType::Submarine | ShipType::Destroyer => 2,
                        ShipType::Battleship | ShipType::Carrier => 3,
                    }
            {
                return None;
            }
        }
        Dir::Vertical => {
            if ship.y
                < match ship.type_ {
                    ShipType::PatrolBoat => 0,
                    ShipType::Submarine | ShipType::Destroyer => 1,
                    ShipType::Battleship | ShipType::Carrier => 2,
                }
            {
                return None;
            }
            if ship.y
                > HEIGHT
                    - match ship.type_ {
                        ShipType::PatrolBoat | ShipType::Submarine | ShipType::Destroyer => 2,
                        ShipType::Battleship | ShipType::Carrier => 3,
                    }
            {
                return None;
            }
        }
    }
    let left = (ship.x as i8
        - match (ship.dir, ship.type_) {
            (Dir::Vertical, _) | (Dir::Horizontal, ShipType::PatrolBoat) => 0,
            (Dir::Horizontal, ShipType::Submarine | ShipType::Destroyer) => 1,
            (Dir::Horizontal, ShipType::Battleship) if ship.x > 2 => 1,
            (Dir::Horizontal, ShipType::Battleship | ShipType::Carrier) => 2,
        }) as f32
        / WIDTH as f32
        * 80.
        + 10.;
    let top = (ship.y as i8
        - match (ship.dir, ship.type_) {
            (Dir::Horizontal, _) | (Dir::Vertical, ShipType::PatrolBoat) => 0,
            (Dir::Vertical, ShipType::Submarine | ShipType::Destroyer) => 1,
            (Dir::Vertical, ShipType::Battleship) if ship.y > 2 => 1,
            (Dir::Vertical, ShipType::Battleship | ShipType::Carrier) => 2,
        }) as f32
        / HEIGHT as f32
        * 50.
        + 40.;
    Some((top, left))
}
