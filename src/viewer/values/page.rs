use std::{
    fs::{self, DirEntry},
    path::Path,
};

pub struct Page(pub DirEntry);

impl Page {
    pub fn new(entry: DirEntry) -> Self {
        Page(entry)
    }

    pub fn path(&self) -> String {
        self.0.path().to_str().unwrap().to_string()
    }

    pub fn stem(&self) -> String {
        self.0
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}

pub struct Pages(pub Vec<Page>);

impl Pages {
    pub fn new(root: &str) -> Self {
        Pages(
            fs::read_dir(Path::new(root))
                .unwrap()
                .map(|x| Page::new(x.unwrap()))
                .filter(|x| x.0.file_type().unwrap().is_file())
                .collect(),
        )
    }
}
