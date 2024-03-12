mod utils;

use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_tween::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DefaultTweenPlugins,
            ResourceInspectorPlugin::<Config>::new(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (utils::main_cursor_world_coord_system, jeb_follows_cursor),
        )
        .init_resource::<Config>()
        .init_resource::<utils::MainCursorWorldCoord>()
        .register_type::<Config>()
        .run();
}

// Let us change the the tween ease and duration at runtime
#[derive(Resource, Reflect)]
struct Config {
    tween_duration: Duration,
    tween_ease: EaseFunction,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            tween_duration: Duration::from_millis(500),
            tween_ease: EaseFunction::ExponentialOut,
        }
    }
}

/// Marker component for the square that will be following the cursor
#[derive(Component)]
struct Jeb;

/// Marker component for the tween entity we will be modifying to make the follow
/// effect
#[derive(Component)]
struct JebTranslationTween;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            ..Default::default()
        },
        utils::MainCamera,
    ));

    // Spawning the square
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("square_filled.png"),
                ..Default::default()
            },
            Jeb,
        ))
        .with_children(|c| {
            // Spawning the marker for a tween player that will be responsible
            // for the follow effect
            c.spawn(JebTranslationTween);

            // Spawning a tween player that's responsible for a rotating effect
            c.spawn((
                SpanTweenPlayerBundle::new(Duration::from_secs(2))
                    .with_repeat(Repeat::Infinitely)
                    .with_repeat_style(RepeatStyle::PingPong),
                SpanTweenBundle::new(..Duration::from_secs(2)),
                EaseFunction::CubicInOut,
                ComponentTweenDyn::player_parent_dyn(interpolate::closure(
                    |transform: &mut Transform, value| {
                        let start = 0.;
                        let end = TAU;
                        let angle = (end - start).mul_add(value, start);
                        transform.rotation = Quat::from_rotation_z(angle);
                    },
                )),
            ));

            // Spawning a TweenPlayer that's responsible for scaling effect
            // when you launch up the demo.
            c.spawn((
                SpanTweenPlayerBundle::new(Duration::from_secs(1)),
                SpanTweenBundle::new(..Duration::from_secs(1)),
                EaseFunction::QuinticIn,
                ComponentTween::player_parent(interpolate::Scale {
                    start: Vec3::ZERO,
                    end: Vec3::ONE,
                }),
            ));
        });
}

fn jeb_follows_cursor(
    mut commands: Commands,
    coord: Res<utils::MainCursorWorldCoord>,
    config: Res<Config>,
    q_jeb: Query<&Transform, With<Jeb>>,
    q_jeb_translation_tween: Query<Entity, With<JebTranslationTween>>,
    mut cursor_moved: EventReader<CursorMoved>,
) {
    let jeb_transform = q_jeb.single();
    let jeb_tween = q_jeb_translation_tween.single();
    let Some(coord) = coord.0 else {
        return;
    };
    if cursor_moved.read().next().is_some() {
        // inserting a new TweenPlayer everytime the cursor moved
        commands.entity(jeb_tween).insert((
            SpanTweenPlayerBundle::new(config.tween_duration),
            SpanTweenBundle::new(..config.tween_duration),
            config.tween_ease, // don't forget the ease
            // You can have multiple tween in the same Entity as long as their
            // type is differernt.
            //
            // This one for translation
            ComponentTween::player_parent(interpolate::Translation {
                start: jeb_transform.translation,
                end: Vec3::new(coord.x, coord.y, 0.),
            }),
            // This one for color
            ComponentTween::player_parent(interpolate::SpriteColor {
                start: Color::PINK,
                end: Color::WHITE,
            }),
        ));
    }
}
