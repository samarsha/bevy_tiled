use bevy::{asset::AssetServerSettings, prelude::*, render::render_graph::RenderGraph};

mod loader;
mod map;
pub use map::*;
mod pipeline;
pub use pipeline::*;
mod tile_map;
pub use tile_map::*;

/// Adds support for GLTF file loading to Apps
#[derive(Default)]
pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let asset_folder = app
            .resources_mut()
            .get_or_insert_with(AssetServerSettings::default)
            .asset_folder
            .clone();

        app.add_asset::<map::Map>()
            .add_asset_loader(loader::TiledMapLoader { asset_folder })
            .add_system(process_loaded_tile_maps.system());

        let resources = app.resources();
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
        render_graph.add_tile_map_graph(resources);
    }
}
