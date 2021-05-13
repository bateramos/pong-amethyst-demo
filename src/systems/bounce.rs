use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage, Write},
    shrev::EventChannel,
};

use crate::pong::{Ball, Paddle, AREA_HEIGHT};
use crate::audio::SoundEvent;

pub struct BounceSystem;

impl <'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        Write<'s, EventChannel<SoundEvent>>,
    );

    fn run(&mut self, (mut balls, paddles, transforms, mut event_channel): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;
            let ball_y_velocity = ball.velocity[1];

            if (ball_y <= ball.radius && ball_y_velocity < 0.0) || (ball_y >= AREA_HEIGHT - ball.radius && ball_y_velocity > 0.0) {
                ball.velocity[1] = -ball.velocity[1];
            }

            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x - (paddle.width * 0.5);
                let paddle_y = paddle_transform.translation().y - (paddle.height * 0.5);

                if point_in_rect(
                    ball_x, ball_y,
                    paddle_x - ball.radius, paddle_y - ball.radius,
                    paddle_x + paddle.width + ball.radius,
                    paddle_y + paddle.height + ball.radius
                ) {
                    ball.velocity[0] = -ball.velocity[0];
                    event_channel.single_write(SoundEvent::Bounce);
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
