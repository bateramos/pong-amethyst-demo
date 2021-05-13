use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    ecs::{Read, ReadExpect, System, SystemData, World},
    shrev::{EventChannel, ReaderId},
    core::SystemDesc,
};

use crate::audio::{SoundEvent, Sounds, play_sound};

#[derive(Default)]
pub struct AudioSystemDesc;

impl <'s, 'f> SystemDesc<'s, 'f, AudioSystem> for AudioSystemDesc {
    fn build(self, world: &mut World) -> AudioSystem {
        <AudioSystem as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<SoundEvent>>().register_reader();
        AudioSystem { reader_id }
    }
}

pub struct AudioSystem {
    reader_id: ReaderId<SoundEvent>
}

impl <'s> System<'s> for AudioSystem {

    type SystemData = (
        Read<'s, EventChannel<SoundEvent>>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (event_channel, asset_storage, sounds, output): Self::SystemData) {
        for event in event_channel.read(&mut self.reader_id) {
            play_sound(&*sounds, event, &asset_storage, output.as_deref());
        }
    }
}
