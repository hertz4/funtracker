extern crate sdl2;
use std::time::Duration;
use std::thread;
use self::sdl2::audio::{AudioSpecDesired, AudioCallback};
use self::sdl2::{Sdl, AudioSubsystem};

use player::song::{Song, Track};

pub struct Audio {
    subsystem: AudioSubsystem,
    srate: i32,
}

impl Audio {
    pub fn new(context: Sdl, srate: i32) -> Audio {
        Audio {
            subsystem: context.audio()
                .expect("failed to init sdl2 audio"),
            srate: srate,
        }
    }

    pub fn play_track(&mut self, track: Track) {
        let desired_spec = AudioSpecDesired {
            freq: Some(self.srate),
            channels: Some(1),
            samples: None,
        };

        struct ACallback<'a> {
            song: Song<'a>,
        };

        impl<'a> AudioCallback for ACallback<'a> {
            type Channel = f32;
            fn callback(&mut self, out: &mut [f32]) {
                for point in out.iter_mut() {
                    *point = self.song.get_point();
                }
            }
        }

        let device = self.subsystem.open_playback(
            None, &desired_spec, |_|
            ACallback {
                song: Song::new(&track, track.seq[1].len()),
            }
        )
            .expect("could not open playback");

        device.resume();
        thread::sleep(Duration::from_millis(40000));
    }
}
