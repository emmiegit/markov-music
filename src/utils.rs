use std::env;
use std::path::{Path, PathBuf};

lazy_static! {
    pub static ref HOME_DIR_PATH: PathBuf = env::home_dir().expect("Unable to get home directory");
    pub static ref HOME_DIR: &'static str = HOME_DIR_PATH.as_path()
        .to_str().expect("Home directory not UTF-8");
}

pub fn compress_path<P: AsRef<Path>>(path: P) -> String {
    let path = path.as_ref();
    let mut pathbuf;
    match path.strip_prefix(HOME_DIR_PATH.as_path()) {
        Ok(p) => {
            pathbuf = PathBuf::from("~");
            pathbuf.push(p);

            &pathbuf
        }
        Err(_) => path,
    }.to_str()
        .expect("Path not valid UTF-8")
        .to_string()
}
