use config::Config;
use error::Error;
use markov::Chain;
use player::MpvPlayer;
use player::Player;
use song::Song;

pub struct Selector {
    player: Player,
    chain: Chain<Song>,
}

impl Selector {
    pub fn new(cfg: &Config) -> Result<Self, Error> {
        let mut player = match cfg.get_player() {
            "mpv" => Player::Mpv(MpvPlayer::new()),
            _ => {
                return Err(Error::new("Unknown player type"));
            }
        };

        Ok(Selector {
               player: player,
               chain: Chain::new(),
           })
    }
}
