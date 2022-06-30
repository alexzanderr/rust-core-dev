use core_dev::audio::nonblocking::MusicPlayer;

fn main() -> std::result::Result<(), soloud::SoloudError> {
    let path = "";
    let mut player = MusicPlayer::new()?;
    player.load_file(path)?;
    player.play_music(false, None);

    std::thread::sleep(std::time::Duration::from_millis(1000));

    // player.pause_playing();
    player.play_music(true, Some(100));
    while player.is_playing() {
        // calls to play are non-blocking, so we put the thread to sleep
        // player.pause_playing();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        // player.continue_playing();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    Ok(())
}
