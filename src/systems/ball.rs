use amethyst::{
    ecs::{Join, Entities, Read, WriteStorage, ReadExpect, System, SystemData, World},
    assets::{Handle},
    shrev::{EventChannel, ReaderId},
    core::{SystemDesc, transform::Transform, timing::Time},
    renderer::{SpriteRender, SpriteSheet},
};

use crate::pong::{PongEvent, Ball, SpriteSheetHandler, AREA_WIDTH, AREA_HEIGHT};

#[derive(Default)]
pub struct BallSystemDesc;

impl <'s, 'f> SystemDesc<'s, 'f, BallSystem> for BallSystemDesc {
    fn build(self, world: &mut World) -> BallSystem {
        <BallSystem as System>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<PongEvent>>().register_reader();

        BallSystem { reader_id, ball_spawn_timer: Some(1.0) }
    }
}

pub struct BallSystem {
    pub reader_id : ReaderId<PongEvent>,
    ball_spawn_timer: Option<f32>,
}

impl <'s> System<'s> for BallSystem {
    type SystemData = (
        Read<'s, EventChannel<PongEvent>>,
        Entities<'s>,
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, SpriteSheetHandler>,
        ReadExpect<'s, Time>,
    );

    fn run(&mut self, (event_channel, entities, mut balls, mut transform, mut sprites, sprite_sheet_handler, time): Self::SystemData) {
        for event in event_channel.read(&mut self.reader_id) {
            match event {
                PongEvent::Score => {
                    for (entity, _ball) in (&*entities, &mut balls).join() {
                        match entities.delete(entity) {
                            Ok(_) => {},
                            Err(error) => panic!("Something when wrong {}", error),
                        };
                        self.ball_spawn_timer.replace(1.0);
                    }
                },
                _ => {},
            };
        }

        if let Some(mut timer) = self.ball_spawn_timer.take() {
            {
                timer -= time.delta_seconds();
            }

            if timer <= 0.0 {
                initialise_ball(&entities, &mut balls, &mut transform, &mut sprites, sprite_sheet_handler.sprite_sheet_handler.clone());
                self.ball_spawn_timer.take();
            } else {
                self.ball_spawn_timer.replace(timer);
            }
        }
    }
}

fn initialise_ball<'s>(entities: &Entities, balls: &mut WriteStorage<'s, Ball>, locals: &mut WriteStorage<'s, Transform>, sprites: &mut WriteStorage<'s, SpriteRender>, sprite_sheet_handler: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(AREA_WIDTH / 2.0, AREA_HEIGHT / 2.0, 0.0);

    let sprite_render = SpriteRender::new(sprite_sheet_handler, 1);

    entities
        .build_entity()
        .with(sprite_render, sprites)
        .with(local_transform, locals)
        .with(Ball::default(), balls)
        .build();
}
