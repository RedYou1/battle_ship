use bevy::{
    asset::AssetServer,
    ecs::{component::Component, system::Res},
    ui::{node_bundles::ImageBundle, UiImage},
    utils::default,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Component)]
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

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Slot {
    #[default]
    None,
    NoneShot,
    Ship(ShipType),
    Hit(ShipType),
}
