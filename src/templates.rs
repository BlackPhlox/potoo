use bevy_codegen::model::{
    BevyModel, BevyType, Component, CrateDependency, Custom, CustomCode, Feature, Import, Meta,
    Plugin, System, Used,
};

pub fn default_game_template() -> BevyModel {
    let mut bevy_model = BevyModel {
        meta: Meta {
            name: "bevy_test".to_string(),
            bevy_type: BevyType::App,
            ..Default::default()
        },
        ..Default::default()
    };

    bevy_model.components.push(Component {
        name: "Player".to_string(),
        content: vec![
            ("velocity".to_string(), "Vec3".to_string()),
            ("rotation_speed".to_string(), "f32".to_string()),
            ("shooting_timer".to_string(), "Option<f32>".to_string()),
        ],
    });

    //For asset_server
    bevy_model.plugins.push(Plugin {
        name: "DefaultPlugins.set(AssetPlugin{watch_for_changes: true, ..default()})".to_string(),
        is_group: true,
        dependencies: vec![],
    });

    let setup_entities = System {
        name: "setup".to_string(),
        param: vec![
            ("mut commands".to_string(), "Commands".to_string()),
            ("asset_server".to_string(), "Res<AssetServer>".to_string()),
        ],
        content: r#"

commands.spawn(Camera2dBundle::default());

// player
let ship_handle = asset_server.load("ship_C.png");
commands
.spawn(SpriteBundle {
    texture: ship_handle,
    ..default()
})
.insert(Player {
    velocity: Vec3::ZERO,
    rotation_speed: f32::to_radians(180.0),
    shooting_timer: None,
});

"#
        .to_string(),
        attributes: vec![],
        ..Default::default()
    };

    bevy_model.startup_systems.push(setup_entities);

    bevy_model.systems.push(System {
        name: "player_movement_system".to_string(),
        param: vec![
            (
                "keyboard_input".to_string(),
                "Res<Input<KeyCode>>".to_string(),
            ),
            (
                "mut query".to_string(),
                "Query<(&Player, &mut Transform)>".to_string(),
            ),
            ("time".to_string(), "Res<Time>".to_string()),
        ],
        content: r#"const SPEED: f32 = 500.0;

let (ship, mut transform) = query.single_mut();

let mut rotation_factor = 0.0;
let mut movement_factor = 0.0;

if keyboard_input.pressed(KeyCode::Left) {
    rotation_factor += 1.0;
}

if keyboard_input.pressed(KeyCode::Right) {
    rotation_factor -= 4.0;
}

if keyboard_input.pressed(KeyCode::Up) {
    movement_factor += 1.0;
}

// update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_seconds());

// get the ship's forward vector by applying the current rotation to the ships initial facing vector
let movement_direction = transform.rotation * Vec3::Y;
// get the distance the ship will move based on direction, the ship's movement speed and delta time
let movement_distance = movement_factor * SPEED * time.delta_seconds();
// create the change in translation using the new movement direction and distance
let translation_delta = movement_direction * movement_distance;
// update the ship translation with our new translation delta
transform.translation += translation_delta;

// bound the ship within the invisible level bounds
let extents = Vec3::from((BOUNDS / 2.0, 0.0));
transform.translation = transform.translation.min(extents).max(-extents);"#
            .to_string(),
        ..Default::default()
    });

    bevy_model.systems.push(System {
        name: "player_shooting_system".to_string(),
        param: vec![
            ("mut commands".to_string(), "Commands".to_string()),
            (
                "keyboard_input".to_string(),
                "Res<Input<KeyCode>>".to_string(),
            ),
            (
                "query".to_string(),
                "Query<&Transform, With<Player>>".to_string(),
            ),
        ],
        content: r#"const SIZE: f32 = 10.0;

if keyboard_input.just_pressed(KeyCode::Space) {
    if let Ok(tfm) = query.get_single() {
        commands
            .spawn(SpriteBundle {
                transform: *tfm,
                sprite: Sprite {
                    color: Color::rgb(0.9, 0.8, 0.0),
                    custom_size: Some(Vec2::new(SIZE, SIZE)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Bullet);
    }
        }"#
        .to_string(),
        ..Default::default()
    });

    bevy_model.systems.push(System {
        name: "bullet_movement_system".to_string(),
        param: vec![
            ("mut commands".to_string(), "Commands".to_string()),
            (
                "mut query".to_string(),
                "Query<(Entity, &mut Transform), With<Bullet>>".to_string(),
            ),
            ("cam".to_string(), "Query<&Camera>".to_string()),
            ("time".to_string(), "Res<Time>".to_string()),
        ],
        content: r#"let screen_size = cam.single().logical_viewport_size().unwrap() * 0.5;
let speed = 500.0;
for (entity, mut tfm) in &mut query {
    let x = tfm
        .rotation
        .mul_vec3(Vec3::new(0.0, speed * time.delta_seconds(), 0.0));
    tfm.translation += x;

    if utilities::is_outside_bounds(
        tfm.translation.truncate(),
        (
            (-screen_size.x),
            screen_size.y,
            screen_size.x,
            (-screen_size.y),
        ),
    ) {
        log::info!("pufff");
        commands.entity(entity).despawn();
    }
}"#
        .to_string(),
        ..Default::default()
    });

    bevy_model.systems.push(System {
        name: "bullet_hit_system".to_string(),
        param: vec![
            ("mut commands".to_string(), "Commands".to_string()),
            (
                "bullet_query".to_string(),
                "Query<&Transform, With<Bullet>>".to_string(),
            ),
            (
                "ship_query".to_string(),
                "Query<(Entity, &Transform), With<OtherShip>>".to_string(),
            ),
        ],
        content: r#"for bullet_tfm in bullet_query.iter() {
for (entity, ship_tfm) in ship_query.iter() {
    if collide_aabb::collide(
        bullet_tfm.translation,
        Vec2::new(10.0, 10.0),
        ship_tfm.translation,
        Vec2::new(30.0, 30.0),
    )
    .is_some()
        {
            log::info!("BUUMMMM");
            commands.entity(entity).despawn();
        }
    }
}
"#
        .to_string(),
        ..Default::default()
    });
    bevy_model.systems.push(System {
        name: "spawn_other_ships".to_string(),
        param: vec![
            ("mut commands".to_string(), "Commands".to_string()),
            ("asset_server".to_string(), "Res<AssetServer>".to_string()),
            (
                "others".to_string(),
                "Query<(Entity, &Transform), With<OtherShip>>".to_string(),
            ),
            ("cam".to_string(), "Query<&Camera>".to_string()),
        ],
        content: r#"const MARGIN: f32 = 30.0;
const MIN_SHIP_COUNT: usize = 10;

let screen_size = cam.single().logical_viewport_size().unwrap() * 0.5;
let mut other_ships_count = 0;

for (entity, tfm) in others.iter() {
    if utilities::is_outside_bounds(
        tfm.translation.truncate(),
        (
            (-screen_size.x) - MARGIN,
            screen_size.y + MARGIN,
            screen_size.x + MARGIN,
            (-screen_size.y) - MARGIN,
        ),
    ) {
        commands.entity(entity).despawn();
    } else {
        other_ships_count += 1;
    }
}

if other_ships_count < MIN_SHIP_COUNT {
    let x = if thread_rng().gen::<bool>() {
        thread_rng().gen_range(((-screen_size.x) - MARGIN)..(-screen_size.x))
    } else {
        thread_rng().gen_range(screen_size.x..(screen_size.x + MARGIN))
    };
    let y = if thread_rng().gen::<bool>() {
        thread_rng().gen_range(((-screen_size.y) - MARGIN)..(-screen_size.y))
    } else {
        thread_rng().gen_range(screen_size.y..(screen_size.y + MARGIN))
    };
    let dir = thread_rng().gen_range(0.0f32..360.0f32);
    let mut transform = Transform::from_xyz(x, y, 0.0);
    transform.rotate_z(dir.to_radians());

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("enemy_A.png"),
            transform,
            ..default()
        })
        .insert(OtherShip);
}"#
        .to_string(),
        ..Default::default()
    });
    bevy_model.systems.push(System {
        name: "move_other_ships".to_string(),
        param: vec![
            ("time".to_string(), "Res<Time>".to_string()),
            (
                "mut query".to_string(),
                "Query<&mut Transform, With<OtherShip>>".to_string(),
            ),
        ],
        content: r#"const SPEED: f32 = 100.0;
for mut tfm in &mut query {
    let x = tfm
        .rotation
        .mul_vec3(Vec3::new(0.0, SPEED * time.delta_seconds(), 0.0));

    tfm.translation += x;
}"#
        .to_string(),
        ..Default::default()
    });

    bevy_model.custom.push(Custom::System(CustomCode {
        name: "utilities.rs".to_string(),
        content: r#"use bevy::prelude::*;

pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

pub(crate) fn is_outside_bounds(point: Vec2, bounds: (f32, f32, f32, f32)) -> bool {
    let (left, top, right, bottom) = bounds;
    point.x < left || point.x > right || point.y < bottom || point.y > top
}"#
        .to_string(),
    }));

    bevy_model.imports.push(Import {
        used: Used::Systems,
        dependency: CrateDependency {
            crate_name: "rand".to_string(),
            crate_version: "0.8".to_string(),
            crate_paths: vec!["thread_rng".to_string(), "Rng".to_string()],
        },
    });

    bevy_model.imports.push(Import {
        used: Used::Systems,
        dependency: CrateDependency {
            crate_name: "bevy".to_string(),
            crate_version: "0.9".to_string(),
            crate_paths: vec!["sprite::collide_aabb".to_string()],
        },
    });

    bevy_model.imports.push(Import {
        used: Used::Systems,
        dependency: CrateDependency {
            crate_name: "crate".to_string(),
            crate_version: "0.0".to_string(),
            crate_paths: vec!["utilities::BOUNDS".to_string()],
        },
    });

    //Dynamic Lib for fast reload
    bevy_model.bevy_settings.features.push(Feature::Dynamic);

    bevy_model
}
