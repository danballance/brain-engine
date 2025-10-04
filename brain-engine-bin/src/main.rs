use bevy::prelude::*;
use brain_engine_core::{Map, TileGeneratorDefault};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum PlayerAnimationState {
    Idle,
    WalkNorth,
    WalkEast,
    WalkSouth,
    WalkWest,
}

#[derive(Component)]
struct AnimationTimer(Timer);

const TILE_SIZE: f32 = 64.0;
const GRID_SIZE: usize = 5;
const PLAYER_SPEED: f32 = 100.0;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("{}x{} Grid", GRID_SIZE, GRID_SIZE).into(),
                resolution: (GRID_SIZE as f32 * TILE_SIZE, GRID_SIZE as f32 * TILE_SIZE).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_map, setup_player))
        .add_systems(Update, (handle_input, move_player, animate_sprite))
        .run()
}

fn setup_map(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(Camera2d);

    let tile_generator = TileGeneratorDefault {
        tile_exit_probability: 0.5,
    };
    let map = Map::new(GRID_SIZE, tile_generator);
    let center_offset = (map.size - 1) as f32 / 2.0 * TILE_SIZE;
    for (position, texture_file_name) in map.iter_tiles() {
        let tile_texture = asset_server.load(texture_file_name);
        commands.spawn((
            Sprite::from_image(tile_texture.clone()),
            Transform::from_translation(Vec3::new(
                position.x as f32 * TILE_SIZE - center_offset, // Center the grid
                position.y as f32 * TILE_SIZE - center_offset,
                0.0,
            )),
        ));
    }
    commands.insert_resource(map);
}

fn setup_player(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut asset_texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player_texture = "16x16-Player-Sheet.png";
    let player_texture_handle = asset_server.load(player_texture);
    let player_texture_atlas_layout =
        TextureAtlasLayout::from_grid(UVec2::new(16, 16), 20, 1, None, None);
    let player_texture_atlas_layout_handle =
        asset_texture_atlas_layout.add(player_texture_atlas_layout);

    commands.spawn((
        Sprite::from_atlas_image(
            player_texture_handle,
            TextureAtlas {
                layout: player_texture_atlas_layout_handle,
                index: 0,
            },
        ),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        PlayerAnimationState::Idle,
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
    ));
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PlayerAnimationState>,
) {
    for mut state in query.iter_mut() {
        let new_state = if keyboard_input.pressed(KeyCode::ArrowLeft) {
            PlayerAnimationState::WalkWest
        } else if keyboard_input.pressed(KeyCode::ArrowUp) {
            PlayerAnimationState::WalkNorth
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            PlayerAnimationState::WalkEast
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            PlayerAnimationState::WalkSouth
        } else {
            PlayerAnimationState::Idle
        };

        *state = new_state;
    }
}

fn move_player(time: Res<Time>, mut query: Query<(&mut Transform, &PlayerAnimationState)>) {
    for (mut transform, state) in query.iter_mut() {
        let movement = match state {
            PlayerAnimationState::Idle => Vec3::ZERO,
            PlayerAnimationState::WalkNorth => {
                Vec3::new(0.0, PLAYER_SPEED * time.delta_secs(), 0.0)
            }
            PlayerAnimationState::WalkEast => Vec3::new(PLAYER_SPEED * time.delta_secs(), 0.0, 0.0),
            PlayerAnimationState::WalkSouth => {
                Vec3::new(0.0, -PLAYER_SPEED * time.delta_secs(), 0.0)
            }
            PlayerAnimationState::WalkWest => {
                Vec3::new(-PLAYER_SPEED * time.delta_secs(), 0.0, 0.0)
            }
        };
        transform.translation += movement;
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut AnimationTimer, &PlayerAnimationState)>,
) {
    for (mut sprite, mut timer, state) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                let step = (atlas.index + 1) % 4; // step will be 1-4
                let animation_start_frame = match state {
                    PlayerAnimationState::Idle => 0,
                    PlayerAnimationState::WalkSouth => 4,
                    PlayerAnimationState::WalkWest => 8,
                    PlayerAnimationState::WalkNorth => 12,
                    PlayerAnimationState::WalkEast => 16,
                };
                atlas.index = animation_start_frame + step;
            }
        }
    }
}
