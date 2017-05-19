use config::Config;
use error::Error;
use player::mpv_player::MpvPlayer;
use player::player::Player;

pub struct Selector {
    player: Player,
}

impl Selector {
    pub fn new(cfg: &Config) -> Result<Self, Error> {
        let mut player = match cfg.get_player() {
            "mpv" => Player::Mpv(MpvPlayer::new()),
            _ => {
                return Err(Error::new("Unknown player type"));
            }
        };

        Ok(Selector { player: player })
    }
}
