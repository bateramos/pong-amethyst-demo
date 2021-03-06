use std::time::Duration;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    core::transform::TransformBundle,
    core::frame_limiter::FrameRateLimitStrategy,
    input::{InputBundle, StringBindings},
    ui::{RenderUi, UiBundle},
    audio::{AudioBundle, DjSystemDesc},
};

mod pong;
mod audio;
mod systems;

use crate::pong::Pong;
use crate::audio::Music;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let asset_dir = app_root.join("assets");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderUi::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(DjSystemDesc::new(|music: &mut Music| music.music.next()), "dj_system", &[])
        .with_system_desc(systems::AudioSystemDesc::default(), "custom_audio_system", &[])
        .with(systems::InputSystem, "custom_input_system", &["input_system"])
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "move_ball_system", &[])
        .with(systems::BounceSystem::new(), "bounce_system", &["paddle_system", "move_ball_system"])
        .with(systems::WinnerSystem, "winner_system", &["move_ball_system"])
        .with_system_desc(systems::VelocitySystemDesc::default(), "velocity_system", &["bounce_system", "move_ball_system"])
        .with_system_desc(systems::BallSystemDesc::default(), "ball_system", &["winner_system"]);

    let mut game = Application::build(asset_dir, Pong::default())?
        .with_frame_limit(FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)), 144)
        .build(game_data)?;

    game.run();

    Ok(())
}
