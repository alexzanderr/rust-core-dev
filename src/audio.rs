use soloud::SoloudError;
use soloud::audio::Wav;
use soloud::Soloud;
use soloud::Handle;


pub mod nonblocking {

    // use super::SoloudError;
    // use super::Wav;
    // use super::Soloud;
    // use super::Handle;
    use soloud::*;

    // TODO this is broken if more than 1 instance of this struct is alive
    #[derive(Debug)]
    pub struct MusicPlayer {
        pub player:  Soloud,
        wav:     Wav,
        playing: bool,
        // songs: HashMap<&'a str, >
        handle:  Option<Handle>,
    }

    impl MusicPlayer {
        pub fn new() -> Result<Self, SoloudError> {
            let mut sl = Soloud::default()?;
            // sl.set_global_volume(2.0);

            let mut wav = Wav::default();
            Ok(Self {
                player: sl,
                wav,
                playing: false,
                handle: None,
            })
        }

        pub fn load_file<P: AsRef<std::path::Path>>(
            &mut self,
            path: P,
        ) -> Result<(), SoloudError> {
            self.wav.load(path)?;
            Ok(())
        }

        pub fn pause_playing(&mut self) {
            if let Some(handle) = self.handle {
                self.playing = false;

                self.player.set_pause(handle, true);
            }
        }

        pub fn continue_playing(&mut self) {
            if let Some(handle) = self.handle {
                self.playing = true;
                self.player.set_pause(handle, false);
            }
        }

        pub fn is_done_playing(&self) -> bool {
            self.player.voice_count() == 0
        }

        pub fn is_playing(&self) -> bool {
            self.player.voice_count() > 0
        }

        pub fn play_music(&mut self, wait: bool, timeout: Option<u64>) {
            self.playing = true;
            let handle = self.player.play(&self.wav);
            self.handle = Some(handle);
            if wait {
                // calls to play are non-blocking, so we put the thread to sleep
                while self.is_playing() {
                    if let Some(timeout) = timeout {
                        std::thread::sleep(std::time::Duration::from_millis(
                            timeout,
                        ));
                    }
                }
            }
        }
    }
}


// // mod blocking {
// //     pub fn playsound()

// // }
