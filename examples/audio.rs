use core_dev::audio::{
    MusicPlayer,
    MusicPlayerResult
};

fn main() -> MusicPlayerResult<()> {
    let mut mp = MusicPlayer::new()?;
    mp.load_file("examples_data/audio/allsafe.wav")?;
    mp.play_music(true, 10.into());
    // mp.play_music_while_doing(|player| {
    //     println!("{}", player.is_done_playing());
    // });

    Ok(())
}
