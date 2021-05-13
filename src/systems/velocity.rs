use amethyst::{
    ecs::{Join, WriteStorage, Read, System, SystemData, World},
    shrev::{EventChannel, ReaderId},
    core::SystemDesc,
};

use crate::pong::{PongEvent, Ball};

#[derive(Default)]
pub struct VelocitySystemDesc;

impl <'s, 'f> SystemDesc<'s, 'f, VelocitySystem> for VelocitySystemDesc {
    fn build(self, world: &mut World) -> VelocitySystem {
        <VelocitySystem as System>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<PongEvent>>().register_reader();
        VelocitySystem { reader_id }
    }
}

pub struct VelocitySystem {
    pub reader_id: ReaderId<PongEvent>,
}

impl <'s> System<'s> for VelocitySystem {
    type SystemData = (
        Read<'s, EventChannel<PongEvent>>,
        WriteStorage<'s, Ball>,
    );

    fn run(&mut self, (event_channel, mut balls): Self::SystemData) {
        for event in event_channel.read(&mut self.reader_id) {
            match event {
                PongEvent::Bounce(ball_id) => {
                    for ball in (&mut balls).join() {
                        if &ball.id == ball_id {
                            let (x_direction, y_direction) = get_ball_directions(ball);

                            ball.velocity[0] = ball.velocity[0] + (5. * x_direction);
                            ball.velocity[1] = ball.velocity[1] + (5. * y_direction);
                        }
                    }
                },
                PongEvent::Score => {
                    for ball in (&mut balls).join() {
                        let (x_direction, y_direction) = get_ball_directions(ball);

                        ball.velocity[0] = ball.original_velocity[0] * x_direction;
                        ball.velocity[1] = ball.original_velocity[1] * y_direction;
                    }
                },
            }
        }
    }
}

fn get_ball_directions(ball: &Ball) -> (f32, f32) {
    let x_direction = if ball.velocity[0] > 0. { 1. } else { -1. };
    let y_direction = if ball.velocity[1] > 0. { 1. } else { -1. };

    (x_direction, y_direction)
}
