use soloud::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::path::Path;

// TODO this is broken if more than 1 instance of this struct is alive

#[derive(Debug)]
pub struct MusicPlayerError {
    message: String
}

impl MusicPlayerError {
    pub fn new<M: AsRef<str>>(message: M) -> Self {
        let message = String::from(message.as_ref());

        Self {
            message
        }
    }
}

impl std::fmt::Display for MusicPlayerError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MusicPlayerError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<SoloudError> for MusicPlayerError {
    fn from(soloud_error: SoloudError) -> Self {
        MusicPlayerError::new(&soloud_error.to_string())
    }
}

lazy_static::lazy_static! {
    static ref INITIALIZED_MUSIC_PLAYER: AtomicBool = AtomicBool::new(false);
}

pub type MusicPlayerResult<T> = std::result::Result<T, MusicPlayerError>;

#[derive(Debug)]
// https://github.com/MoAlyousef/soloud-rs/issues/24
pub struct MusicPlayer {
    pub player: Soloud,
    wav:        Wav,
    playing:    bool,
    // songs: HashMap<&'a str, >
    handle:     Option<Handle>,
    timeout:    Option<u64>
}

impl MusicPlayer {
    pub fn new() -> MusicPlayerResult<Self> {
        if INITIALIZED_MUSIC_PLAYER.load(Ordering::Relaxed) {
            Err(MusicPlayerError::new("MusicPlayer already initialized!"))
        } else {
            INITIALIZED_MUSIC_PLAYER.store(true, Ordering::Relaxed);
            let mut _soloud = Soloud::default()?;
            _soloud.set_global_volume(2.0);

            let mut wav = Wav::default();

            Ok(MusicPlayer {
                player: _soloud,
                wav,
                playing: false,
                handle: None,
                timeout: Some(10)
            })
        }
    }

    // pub fn new() -> Result<Self, SoloudError> {
    //     let mut sl = Soloud::default()?;
    //     // sl.set_global_volume(2.0);

    //     let mut wav = Wav::default();
    //     Ok(Self {
    //         player: sl,
    //         wav,
    //         playing: false,
    //         handle: None,
    //     })
    // }

    /// load music to play from file
    /// file extension can be mp3, wav, any audio format
    pub fn load_file<P: AsRef<Path>>(
        &mut self,
        path: P
    ) -> MusicPlayerResult<()> {
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

    pub fn play_music(
        &mut self,
        wait: bool,
        timeout: Option<u64>
    ) {
        self.playing = true;
        let handle = self.player.play(&self.wav);
        self.handle = Some(handle);
        if wait {
            // calls to play are non-blocking, so we put the thread to sleep
            while self.is_playing() {
                if let Some(timeout) = timeout {
                    std::thread::sleep(std::time::Duration::from_millis(
                        timeout
                    ));
                }
            }
        }
    }

    pub fn play_music_while_doing<F>(
        &mut self,
        mut program_logic: F
    ) where
        F: FnMut(&mut Self) -> () {
        self.playing = true;
        let handle = self.player.play(&self.wav);
        self.handle = Some(handle);
        // calls to play are non-blocking, so we put the thread to sleep
        while self.is_playing() {
            program_logic(self);
            if let Some(timeout) = self.timeout {
                std::thread::sleep(std::time::Duration::from_millis(
                    timeout
                ));
            }
        }
    }
}
