use std::{iter::Cycle, vec::IntoIter};
use amethyst::{
    assets::{Loader, AssetStorage},
    audio::{output::Output, AudioSink, Source, OggFormat, SourceHandle},
    ecs::{World, WorldExt},
};

const BOUNCE_SOUND: &str = "audio/bounce.ogg";
const SCORE_SOUND: &str = "audio/score.ogg";
const MUSIC_TRACKS: &[&str; 2] = &[
    "audio/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
    "audio/Computer_Music_All-Stars_-_Albatross_v2.ogg",
];

#[derive(Debug)]
pub enum SoundEvent {
    Score, Bounce
}

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle,
}

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub fn initialise_audio(world: &mut World) {
    let (sound, music) = {
        let loader = world.read_resource::<Loader>();
        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.3);

        let music = MUSIC_TRACKS
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };

        let sound = Sounds {
            bounce_sfx: load_audio_track(&loader, &world, BOUNCE_SOUND),
            score_sfx: load_audio_track(&loader, &world, SCORE_SOUND),
        };

        (sound, music)
    };

    world.insert(sound);
    world.insert(music);
}

pub fn play_sound(sounds: &Sounds, sound_type: &SoundEvent, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        let handle = match sound_type {
            SoundEvent::Score => &sounds.score_sfx,
            SoundEvent::Bounce => &sounds.bounce_sfx,
        };

        if let Some(sound) = storage.get(handle) {
            output.play_once(sound, 1.0);
        }
    }
}
