use error::Error;

pub mod mpv_player;
pub mod player;

pub trait MediaPlayer {
    // Player control
    fn set_pause(&mut self, pause: bool);
    fn get_pause(&self) -> bool;
    fn toggle_pause(&mut self) {
        let pause = self.get_pause();
        self.set_pause(!pause);
    }

    // Playlist
    fn play(&mut self, song: &str) -> Result<(), Error>;
    fn enqueue(&mut self, song: &str) -> Result<(), Error>;
    fn clear(&mut self);
    fn stop(&mut self);
    fn next(&mut self);
    fn prev(&mut self);
}
