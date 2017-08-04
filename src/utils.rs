use std::env;
use std::path::{Path, PathBuf};

lazy_static! {
    pub static ref HOME_DIR_PATH: PathBuf = env::home_dir().expect("Unable to get home directory");
    pub static ref HOME_DIR: &'static str = HOME_DIR_PATH.as_path()
        .to_str().expect("Home directory not UTF-8");
}

pub fn compress_path<'a>(path: &'a Path) -> Result<PathBuf, &'a Path> {
    match path.strip_prefix(HOME_DIR_PATH.as_path()) {
        Ok(p) => {
            let mut path = PathBuf::from("~");
            path.push(p);

            Ok(path)
        },
        Err(_) => Err(path),
    }
}
