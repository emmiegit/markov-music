use error::Error;
use mpv::{MpvHandler,MpvHandlerBuilder};
use player::Player;

pub struct Mpv {
    handle: MpvHandler,
}

impl Mpv {
    fn new() -> Mpv {
        let mut mpv_builder = MpvHandlerBuilder::new().expect("Failed to initialize MPV builder");
        mpv_builder.set_option("vo", "null").expect("Unable to disable video output");
        let mpv_handler = mpv_builder.build().expect("Unable to build MPV handler");

        Mpv {
            handle: mpv_handler,
        }
    }
}

impl Player for Mpv {
    // Player control
    fn set_pause(&mut self, pause: bool) {
        self.handle.set_property_async("pause", pause, 0).expect("Unable to set player pause");
    }

    fn get_pause(&self) -> bool {
        self.handle.get_property("pause").expect("Unable to get player pause")
    }

    // Playlist
    fn play(&mut self, song: &str) -> Result<(), Error> {
        match self.handle.command(&["loadfile", song, "replace"]) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn enqueue(&mut self, song: &str) -> Result<(), Error> {
        match self.handle.command(&["loadfile", song, "append"]) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn clear(&mut self) {
        self.handle.command_async(&["playlist-clear"], 0).expect("Unable to clear playlist");
    }

    fn stop(&mut self) {
        self.handle.command_async(&["stop"], 0).expect("Unable to stop player");
    }

    fn next(&mut self) {
        self.handle.command_async(&["playlist-next", "weak"], 0).expect("Unable to move next in the playlist");
    }

    fn prev(&mut self) {
        self.handle.command_async(&["playlist-prev", "weak"], 0).expect("Unable to move next in the playlist");
    }
}
