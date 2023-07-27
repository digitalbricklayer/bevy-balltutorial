pub mod events;
mod systems;
mod game;
mod main_menu;

use systems::*;

use bevy::{
    diagnostic::{
        LogDiagnosticsPlugin,
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
    },
    prelude::*,
};
use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Ball Game!".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EntityCountDiagnosticsPlugin::default())
        .add_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update,transition_to_main_menu_state)
        .add_systems(Update,exit_game)
        .add_systems(Update,handle_game_over)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
