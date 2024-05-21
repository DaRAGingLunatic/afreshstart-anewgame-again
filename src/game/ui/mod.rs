mod game_over_menu;

pub(crate) mod pause_menu;

use game_over_menu::GameOverMenuPlugin;

use pause_menu::PauseMenuPlugin;

use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            
            .add_plugins((PauseMenuPlugin, GameOverMenuPlugin));
            
    }
}
