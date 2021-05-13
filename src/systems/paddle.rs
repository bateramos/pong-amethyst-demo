use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::pong::{Paddle, Side, AREA_HEIGHT, PADDLE_HEIGHT};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl <'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transform, paddle, input): Self::SystemData) {
        for (paddle, transform) in (&paddle, &mut transform).join() {
            let movement = match paddle.side {
                Side::Right => input.axis_value("right_paddle"),
                Side::Left => input.axis_value("left_paddle"),
            };
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scale_amount = 1.2 * mv_amount as f32;
                    transform.set_translation_y(
                        (transform.translation().y + scale_amount)
                            .min(AREA_HEIGHT - PADDLE_HEIGHT * 0.5)
                            .max(PADDLE_HEIGHT * 0.5)
                    );
                }
            }
        }
    }
}
