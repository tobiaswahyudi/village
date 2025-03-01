pub mod components;
pub mod transitions;
pub mod states;

use bevy::prelude::*;
use states::*;

pub use components::FSMIdle;

pub struct FSMPlugin;

impl Plugin for FSMPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            fsm_update_idle,
            fsm_update_walking_to,
            fsm_update_walking_to_harvest,
            fsm_update_harvesting,
            fsm_update_picking_up,
            fsm_update_bringing_to,
        ));
    }
}