use error::Error;
use player::MediaPlayer;
use player::mpv_player::MpvPlayer;

pub enum Player {
    Mpv(MpvPlayer),
}

impl Player {
    fn get_inst<'a>(&'a self) -> &'a MediaPlayer {
        match self {
            &Player::Mpv(ref x) => x,
        }
    }

    fn get_mut_inst<'a>(&'a mut self) -> &'a mut MediaPlayer {
        match self {
            &mut Player::Mpv(ref mut x) => x,
        }
    }
}

impl MediaPlayer for Player {
    fn set_pause(&mut self, pause: bool) {
        self.get_mut_inst().set_pause(pause);
    }

    fn get_pause(&self) -> bool {
        self.get_inst().get_pause()
    }

    fn play(&mut self, song: &str) -> Result<(), Error> {
        self.get_mut_inst().play(song)
    }

    fn enqueue(&mut self, song: &str) -> Result<(), Error> {
        self.get_mut_inst().enqueue(song)
    }

    fn clear(&mut self) {
        self.get_mut_inst().clear();
    }

    fn stop(&mut self) {
        self.get_mut_inst().stop();
    }

    fn next(&mut self) {
        self.get_mut_inst().next();
    }

    fn prev(&mut self) {
        self.get_mut_inst().prev();
    }
}
