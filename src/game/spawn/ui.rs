use bevy::color::palettes::css::{GREEN, ORANGE, WHITE};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::ui::Val::*;

use crate::game::assets::UiAssets;
use crate::{screen::Screen, ui::widgets::Containers};

use super::player::Player;

/// This example uses a shader source file from the assets subdirectory
const HEALTH_BAR_UI_SHADER_PATH: &str = "shaders/health_bar_ui.wgsl";

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(UiMaterialPlugin::<HealthBarUiMaterial>::default());
    app.observe(spawn_game_ui);
    app.register_type::<GameUi>();
}

#[derive(Event, Debug)]
pub struct SpawnGameUi;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct GameUi;

fn spawn_game_ui(_trigger: Trigger<SpawnGameUi>, mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Playing))
        .with_children(|parent| {
            top_ui_root(parent, &ui_assets.health_bar);
        });
}

fn top_ui_root(parent: &mut ChildBuilder, material: &Handle<HealthBarUiMaterial>) {
    parent
        .spawn((
            Name::new("Top Game UI"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(15.0),
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    position_type: PositionType::Absolute,
                    top: Percent(0.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            player_ui(parent, Player::Player1, material.clone_weak());
            player_ui(parent, Player::Player2, material.clone_weak());
        });
}

fn player_ui(parent: &mut ChildBuilder, player: Player, material: Handle<HealthBarUiMaterial>) {
    parent
        .spawn((
            Name::new(player.id()),
            NodeBundle {
                style: Style {
                    width: Percent(40.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(player.color()),
                ..default()
            },
        ))
        .with_children(|parent| {
            name_score_ui(parent, player);
            lives_health_ui(parent, material, player);
        });
}

fn name_score_ui(parent: &mut ChildBuilder, player: Player) {
    parent
        .spawn((
            Name::new("Name Score UI"),
            NodeBundle {
                style: Style {
                    width: Percent(95.0),
                    height: Percent(40.0),
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    top: Percent(0.0),
                    ..default()
                },
                background_color: BackgroundColor(GREEN.into()),
                ..default()
            },
        ))
        .with_children(|parent| {
            name_ui(parent, player);
            score_ui(parent, player);
        });
}

#[derive(Component)]
pub struct NameUi;

fn name_ui(parent: &mut ChildBuilder, player: Player) {
    parent
        .spawn((
            Name::new("Name UI"),
            NodeBundle {
                style: Style {
                    width: Percent(65.0),
                    height: Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Name UI Text"),
                TextBundle::from_section(
                    player.id(),
                    TextStyle {
                        font_size: 30.0,
                        color: WHITE.into(),
                        ..default()
                    },
                ),
                player,
                NameUi,
            ));
        });
}

#[derive(Component)]
pub struct ScoreUi;

fn score_ui(parent: &mut ChildBuilder, player: Player) {
    parent
        .spawn((
            Name::new("Score UI"),
            NodeBundle {
                style: Style {
                    width: Percent(30.0),
                    height: Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Score UI Text"),
                TextBundle::from_section(
                    "0",
                    TextStyle {
                        font_size: 30.0,
                        color: WHITE.into(),
                        ..default()
                    },
                ),
                player,
                ScoreUi,
            ));
        });
}

fn lives_health_ui(
    parent: &mut ChildBuilder,
    material: Handle<HealthBarUiMaterial>,
    player: Player,
) {
    parent
        .spawn((
            Name::new("Lives Health UI"),
            NodeBundle {
                style: Style {
                    width: Percent(95.0),
                    height: Percent(40.0),
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    bottom: Percent(0.0),
                    ..default()
                },
                background_color: BackgroundColor(ORANGE.into()),
                ..default()
            },
        ))
        .with_children(|parent| {
            lives_ui(parent, player);
            health_ui(parent, material, player);
        });
}

#[derive(Component)]
pub struct LivesUi;

fn lives_ui(parent: &mut ChildBuilder, player: Player) {
    parent
        .spawn((
            Name::new("Lives UI"),
            NodeBundle {
                style: Style {
                    width: Percent(15.0),
                    height: Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Lives UI Text"),
                TextBundle::from_section(
                    "0",
                    TextStyle {
                        font_size: 30.0,
                        color: WHITE.into(),
                        ..default()
                    },
                ),
                player,
                LivesUi,
            ));
        });
}

#[derive(Component)]
pub struct HealthBarUi;

fn health_ui(parent: &mut ChildBuilder, material: Handle<HealthBarUiMaterial>, player: Player) {
    parent
        .spawn((
            Name::new("Health UI"),
            NodeBundle {
                style: Style {
                    width: Percent(80.0),
                    height: Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Lives UI Image"),
                MaterialNodeBundle {
                    style: Style {
                        width: Percent(100.0),
                        height: Percent(100.0),
                        ..default()
                    },
                    material,
                    ..default()
                },
                player,
                HealthBarUi,
            ));
        });
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct HealthBarUiMaterial {
    /// Represents how much of the image is visible
    /// Goes from 0 to 1
    #[uniform(0)]
    pub slider: f32,
    /// Image used to represent the slider
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,
}

impl UiMaterial for HealthBarUiMaterial {
    fn fragment_shader() -> ShaderRef {
        HEALTH_BAR_UI_SHADER_PATH.into()
    }
}
