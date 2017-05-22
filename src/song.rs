use std::path::Path;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Song {
    path: Box<Path>,
}

impl Song {
    pub fn new(pathstr: &str) -> Self {
        let path = Path::new(pathstr);

        assert!(path.is_relative);

        Song {
            path: Box::new(path),
        }
    }
}
