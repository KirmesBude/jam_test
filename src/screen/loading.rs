//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;

use super::Screen;
use crate::{
    game::{
        assets::{ImageAssets, SfxAssets, SoundtrackAssets, UiAssets},
        spawn::ui::HealthBarUiMaterial,
    },
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), enter_loading);
    app.add_systems(Update, check_all_loaded.run_if(in_state(Screen::Loading)));
}

fn enter_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut health_bar_ui_materials: ResMut<Assets<HealthBarUiMaterial>>,
) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });

    // Preload assets so the game runs smoothly.
    commands.insert_resource(ImageAssets::new(&asset_server));
    commands.insert_resource(SfxAssets::new(&asset_server));
    commands.insert_resource(SoundtrackAssets::new(&asset_server));
    commands.insert_resource(UiAssets::new(&asset_server, &mut health_bar_ui_materials));
}

fn check_all_loaded(
    image_assets: Res<Assets<Image>>,
    audio_assets: Res<Assets<AudioSource>>,
    health_bar_ui_materials: Res<Assets<HealthBarUiMaterial>>,
    images: Res<ImageAssets>,
    sfxs: Res<SfxAssets>,
    soundtracks: Res<SoundtrackAssets>,
    ui_assets: Res<UiAssets>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    let all_loaded = images.all_loaded(&image_assets)
        && sfxs.all_loaded(&audio_assets)
        && soundtracks.all_loaded(&audio_assets)
        && ui_assets.all_loaded(&health_bar_ui_materials);
    if all_loaded {
        next_screen.set(Screen::Title);
    }
}
