use bevy::prelude::*;

pub mod camera;



mod systems;
pub(crate) mod ui;

use crate::events::GameOver;
use crate::AppState;
use camera::CameraPlugin;



use systems::*;

use crate::game::ui::GameUIPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // State
            .init_state::<SimulationState>()
            // Events
            .add_event::<GameOver>()
            // On Enter systems
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            // Plugins
            .add_plugins((CameraPlugin, GameUIPlugin))
            // systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            // the toggle simulation above will only run if we are in the AppState::Game state.
            // On Enter System
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default] // The default SimulationState is Running.
    Running,
    Paused,
}
