use bevy::{
    asset::AssetServer,
    ecs::{component::Component, system::Res},
    ui::{node_bundles::ImageBundle, UiImage},
    utils::default,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShipType {
    PatrolBoat,
    Submarine,
    Destroyer,
    Battleship,
    Carrier,
}

impl ShipType {
    pub const fn id(self) -> u8 {
        match self {
            Self::PatrolBoat => 1,
            Self::Submarine => 2,
            Self::Destroyer => 3,
            Self::Battleship => 4,
            Self::Carrier => 5,
        }
    }
    pub const fn len(self) -> u8 {
        match self {
            Self::PatrolBoat => 2,
            Self::Submarine | Self::Destroyer => 3,
            Self::Battleship => 4,
            Self::Carrier => 5,
        }
    }
    pub fn ressource(self, asset_server: &Res<AssetServer>) -> ImageBundle {
        ImageBundle {
            image: UiImage {
                texture: asset_server.load(match self {
                    Self::PatrolBoat => "PatrolBoat.png",
                    Self::Submarine => "Submarine.png",
                    Self::Destroyer => "Destroyer.png",
                    Self::Battleship => "Battleship.png",
                    Self::Carrier => "Carrier.png",
                }),
                ..default()
            },
            ..default()
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Dir {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, PartialEq, Component)]
pub struct Ship {
    pub placed: bool,
    pub x: u8,
    pub y: u8,
    pub dir: Dir,
    pub type_: ShipType,
}

#[derive(Clone, Copy, PartialEq, Component)]
pub struct Overlay {
    pub id: u8,
}
