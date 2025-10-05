use bevy::prelude::*;
use brain_engine_core::{Map, Screen, TileGeneratorDefault};

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

#[derive(Component)]
struct Player;

#[derive(Component)]
struct TilePosition(IVec2);

#[derive(Component)]
struct Move {
    destination: Vec3,
}

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
        .add_systems(Startup, (setup_map, setup_player).chain())
        .add_systems(Update, (start_move, animate_move, animate_sprite))
        .run()
}

fn setup_map(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(Camera2d);

    let tile_generator = TileGeneratorDefault {
        tile_exit_probability: 0.5,
        room_probability: 0.5,
    };
    let map = Map::new(GRID_SIZE, tile_generator);
    let screen = Screen::new(UVec2::new(map.x as u32, map.y as u32), TILE_SIZE);
    for (position, texture_file_name) in map.iterate_tiles() {
        let tile_texture = asset_server.load(texture_file_name);
        commands.spawn((
            Sprite::from_image(tile_texture.clone()),
            Transform::from_translation(screen.pixel_position(position)),
        ));
    }
    commands.insert_resource(map);
    commands.insert_resource(screen);
}

fn setup_player(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut asset_texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    screen: Res<Screen>,
) {
    let player_texture = "16x16-Player-Sheet.png";
    let player_texture_handle = asset_server.load(player_texture);
    let player_texture_atlas_layout =
        TextureAtlasLayout::from_grid(UVec2::new(16, 16), 20, 1, None, None);
    let player_texture_atlas_layout_handle =
        asset_texture_atlas_layout.add(player_texture_atlas_layout);

    // Start player at center tile (2, 2)
    let start_tile = IVec2::new(2, 2);
    let start_position = screen.pixel_position(start_tile);

    commands.spawn((
        Player,
        TilePosition(start_tile),
        Sprite::from_atlas_image(
            player_texture_handle,
            TextureAtlas {
                layout: player_texture_atlas_layout_handle,
                index: 0,
            },
        ),
        Transform::from_translation(start_position),
        PlayerAnimationState::Idle,
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
    ));
}

fn start_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    map: Res<Map<TileGeneratorDefault>>,
    screen: Res<Screen>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut TilePosition, &mut PlayerAnimationState), Without<Move>>,
) {
    for (entity, mut tile_position, mut animation_state) in query.iter_mut() {
        // Only process input when Idle (not currently moving)
        if *animation_state != PlayerAnimationState::Idle {
            continue;
        }

        let dir = if keyboard_input.pressed(KeyCode::ArrowUp) {
            Some((IVec2::new(0, 1), PlayerAnimationState::WalkNorth))
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            Some((IVec2::new(0, -1), PlayerAnimationState::WalkSouth))
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            Some((IVec2::new(-1, 0), PlayerAnimationState::WalkWest))
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            Some((IVec2::new(1, 0), PlayerAnimationState::WalkEast))
        } else {
            None
        };

        if let Some((delta, new_animation_state)) = dir {
            let target = tile_position.0 + delta;

            // Check if movement is valid using Map.can_move
            if map.can_move(tile_position.0, target) {
                // Update tile position
                tile_position.0 = target;

                // Set animation state
                *animation_state = new_animation_state;

                // Calculate destination pixel position
                let destination = screen.pixel_position(target);

                // Add Move component to start animation
                commands.entity(entity).insert(Move { destination });
            }
        }
    }
}

fn animate_move(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut PlayerAnimationState, &Move)>,
) {
    for (entity, mut transform, mut animation_state, move_component) in query.iter_mut() {
        let step = PLAYER_SPEED * time.delta_secs();
        let direction = move_component.destination - transform.translation;
        let distance = direction.length();

        if distance <= step {
            // Snap to destination
            transform.translation = move_component.destination;
            *animation_state = PlayerAnimationState::Idle;
            commands.entity(entity).remove::<Move>();
        } else {
            // Move toward destination
            transform.translation += direction.normalize() * step;
        }
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
