pub const GLOBAL_SCALE: f32 = 0.3;
pub const GLOBAL_SCALE_VEC: Vec3 = Vec3::new(GLOBAL_SCALE, GLOBAL_SCALE, GLOBAL_SCALE);

////////////////////////////////////////////////////////////////

use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum SceneAssetType {
    StructureHouse,
    StructureWoodHut,
    TreePine,
    TreeRound,
    TreeDead,
    Villager,
    ResourceWood,
}

#[derive(Resource)]
pub struct SceneAssets {
    pub handles: HashMap<SceneAssetType, Handle<Scene>>,
}

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut asset_handles = HashMap::new();

    asset_handles.insert(
        SceneAssetType::StructureHouse,
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("house.glb")),
    );

    asset_handles.insert(
        SceneAssetType::TreePine,
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("tree.glb")),
    );
    asset_handles.insert(
        SceneAssetType::TreeRound,
        asset_server.load(GltfAssetLabel::Scene(1).from_asset("tree.glb")),
    );
    asset_handles.insert(
        SceneAssetType::TreeDead,
        asset_server.load(GltfAssetLabel::Scene(2).from_asset("tree.glb")),
    );

    asset_handles.insert(
        SceneAssetType::Villager,
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("villager_man.glb")),
    );

    asset_handles.insert(
        SceneAssetType::ResourceWood,
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("wood.glb")),
    );

    asset_handles.insert(
        SceneAssetType::StructureWoodHut,
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("wood_hut.glb")),
    );

    commands.insert_resource(SceneAssets {
        handles: asset_handles,
    });
}
