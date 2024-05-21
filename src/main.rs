pub mod events;
pub mod game;
pub mod main_menu;
pub mod systems;



use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;
use bevy::prelude::*;
use bevy::window::{WindowMode};
use systems::*;


fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Want to play a game?".into(),
                mode: WindowMode::SizedFullscreen,
                resolution: (3840., 2160.).into(),
                decorations: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        // My Plugins
        .add_plugins((MainMenuPlugin,GamePlugin))
                // systems
        .add_systems(Update,( 
            transition_to_game_state, 
            transition_to_main_menu_state, 
            exit_game, 
            handle_game_over))
        .run()
}
// in order to get States, we need to have all these other traits derived.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default] // our default game state is set to the MainMenu
    MainMenu,
    Game,
    GameOver,
    Paused,
}
