use std::fs::DirEntry;
use std::ffi::OsString;
use iced::{button, scrollable};
use std::fs;
use std::path::Path;
use std::slice::Iter;


pub struct Frame {
    pub root: String,
    pub pages: Pages,
    pub page: OsString,
    pub prev_button: button::State,
    pub next_button: button::State,
    pub image_files: scrollable::State,
    pub file_name_buttons: Vec<button::State>,
}

impl Frame {
    pub fn new(root: &str) -> Self {
        let root = String::from(root);
        let pages = Pages::new(&root);
        let page = pages.first().unwrap().file_name();
        let file_name_buttons = pages.iter().map(|_| {button::State::default()}).collect();
        Frame {
            root,
            pages,
            page,
            prev_button: button::State::default(),
            next_button: button::State::default(),
            image_files: scrollable::State::default(),
            file_name_buttons,
        }
    }

    pub fn update_pages(&mut self) {
        self.pages = Pages::new(&self.root);
    }

    pub fn move_prev_page(&mut self) {
        let f = || {
            let mut iter = self.pages.iter();
            let mut prev = iter.next().unwrap().file_name();
            for item in iter {
                if item.file_name().eq(&self.page) {
                    return prev
                }
                prev = item.file_name();
            }
            self.pages.last().unwrap().file_name()
        };
        self.page = f();
    }

    pub fn move_next_page(&mut self) {
        let f = || {
            let mut iter = self.pages.iter();
            let mut prev = iter.next().unwrap().file_name();
            for item in iter {
                if prev.eq(&self.page) {
                    return item.file_name()
                }
                prev = item.file_name();
            }
            self.pages.first().unwrap().file_name()
        };
        self.page = f();
    }

    pub fn move_pressed_page(&mut self, page_num: usize) {
        self.page = self.pages.get_at(page_num).unwrap().file_name();
    }
}


pub struct Pages {
    pages: Vec<DirEntry>,
}

impl Pages {
    pub fn new(root: &str) -> Self {
        Pages {
            pages: fs::read_dir(Path::new(root)).unwrap()
                .map(|x| x.unwrap())
                .filter(|x| {
                    x.file_type().unwrap().is_file()
                }).collect()
        }
    }

    pub fn iter(&self) -> Iter<DirEntry> {
        self.pages.iter()
    }

    pub fn first(&self) -> Option<&DirEntry> {
        self.pages.first()
    }

    pub fn last(&self) -> Option<&DirEntry> {
        self.pages.last()
    }

    pub fn get_at(&self, index: usize) -> Option<&DirEntry> {
        self.pages.get(index)
    }
}
