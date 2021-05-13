use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, System, SystemData, WriteStorage, Write, ReadExpect},
    ui::UiText,
    shrev::EventChannel,
};

use crate::audio::SoundEvent;
use crate::pong::{PongEvent, Ball, ScoreBoard, ScoreText, AREA_WIDTH};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Write<'s, EventChannel<SoundEvent>>,
        Write<'s, EventChannel<PongEvent>>,
    );

    fn run(&mut self, (mut balls, mut locals, mut ui_text, mut scores, score_text, mut event_channel, mut pong_event_channel): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                scores.score_right = (scores.score_right + 1.).min(999.);

                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else if ball_x >= AREA_WIDTH - ball.radius {
                scores.score_left = (scores.score_left + 1.).min(999.);

                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_left.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                event_channel.single_write(SoundEvent::Score);
                pong_event_channel.single_write(PongEvent::Score);
            }
        }
    }
}
