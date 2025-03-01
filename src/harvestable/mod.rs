use bevy::prelude::*;

pub mod harvestable;
pub mod tree;

pub use harvestable::*;

pub struct HarvestablePlugin;

impl Plugin for HarvestablePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HarvestableDestroyed>();
        app.add_systems(PostUpdate, check_harvestable_destroyed);
    }
}
