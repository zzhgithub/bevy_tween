use bevy::prelude::*;
use bevy_time_runner::{Repeat, RepeatStyle};
use bevy_tween::interpolate::{basic_color, translation};
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::{AnimationBuilderExt, IntoTarget};
use bevy_tween::tween::AnimationTarget;
use bevy_tween::DefaultTweenPlugins;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Basic Color for Pbr".to_string(),
                    resizable: false,
                    resolution: Vec2::new(1100., 250.).into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            DefaultTweenPlugins,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 创建 一个相机的 一个方块
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 5.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    let entity = AnimationTarget.into_target();

    commands
        .spawn((
            AnimationTarget,
            Transform::default(),
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                ..Default::default()
            })),
        ))
        .animation()
        .repeat(Repeat::Infinitely)
        .repeat_style(RepeatStyle::PingPong)
        .insert_tween_here(
            Duration::from_secs_f32(2.),
            EaseKind::ExponentialInOut,
            entity.with(basic_color(Color::WHITE, Color::BLACK)),
        );
}
