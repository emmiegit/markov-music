use error::Error;

mod mpv_player;

pub trait Player {
    // Player control
    fn set_pause(&mut self, state: bool);
    fn get_pause(&self) -> bool;
    fn toggle_pause(&mut self) {
        self.set_pause(!self.get_pause());
    }

    // Playlist
    fn play(&mut self, song: &str) -> Result<(), Error>;
    fn enqueue(&mut self, song: &str) -> Result<(), Error>;
    fn clear(&mut self);
    fn stop(&mut self);
    fn next(&mut self);
    fn prev(&mut self);
}
