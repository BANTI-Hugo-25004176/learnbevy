use bevy::{prelude::*, window::{MonitorSelection, Window, WindowMode, WindowPlugin},};
use bevy_procedural_tilemaps::prelude::{Cartesian3D, ProcGenSimplePlugin};

use crate::map::generate::setup_generator;
use crate::camera::CameraPlugin;

mod map;
mod characters;
mod state;
mod collision;
mod config;
mod inventory;
mod camera;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins.set(AssetPlugin {
        file_path : "src/assets".into(),
        ..Default::default()
    }).set(WindowPlugin {
            primary_window: Some(Window {
            title: "Bevy Game".into(),
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current), // Add this line
            ..default()
        }),
        ..Default::default()
    }).set(ImagePlugin::default_nearest()),)
    .add_plugins(ProcGenSimplePlugin::<Cartesian3D, Sprite>::default())
    .add_plugins(state::StatePlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(characters::CharactersPlugin)
    .add_plugins(inventory::InventoryPlugin)
    .add_plugins(collision::CollisionPlugin)
    .add_systems(Startup, setup_generator)
    .run();
}
