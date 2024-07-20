//! Spawn the player.

use bevy::{
    color::palettes::css::{BLUE, RED},
    prelude::*,
};

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{ImageAsset, ImageAssets},
        movement::{Movement, MovementController, WrapWithinWindow},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn color(&self) -> Color {
        match self {
            Player::Player1 => BLUE.into(),
            Player::Player2 => RED.into(),
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            Player::Player1 => "Player 1",
            Player::Player2 => "Player 2",
        }
    }
}

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    commands.spawn((
        Name::new("Player"),
        Player::Player1,
        SpriteBundle {
            texture: images[&ImageAsset::Ducky].clone_weak(),
            transform: Transform::from_scale(Vec2::splat(8.0).extend(1.0)),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        MovementController::default(),
        Movement { speed: 420.0 },
        WrapWithinWindow,
        player_animation,
        StateScoped(Screen::Playing),
    ));
}
