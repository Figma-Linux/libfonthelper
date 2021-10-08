#[macro_use]
extern crate log;
extern crate finder;
extern crate font_reader;
extern crate pretty_env_logger;
extern crate regex;

mod utils;

use finder::{Finder, IntoIter};
use font_reader::{read, types::Fonts};

pub struct FontsHelper {
  finder: IntoIter,
}

pub struct FontEntry {
  pub path: String,
  pub entries: Fonts,
}

impl FontsHelper {
  pub fn new(dirs: &[String]) -> Self {
    pretty_env_logger::init();

    FontsHelper {
      finder: Finder::new(&dirs.join(":"))
        .filter(&utils::filter_files)
        .into_iter(),
    }
  }
}

impl Iterator for FontsHelper {
  type Item = FontEntry;

  fn next(&mut self) -> Option<FontEntry> {
    match self.finder.next() {
      None => None,
      Some(file) => {
        let font_path = String::from(file.path().to_str().unwrap());
        let mut entry = FontEntry {
          path: "".to_owned(),
          entries: vec![],
        };

        match read(&font_path) {
          Err(err) => {
            warn!("Cannot open font {}, ERROR: {}", &font_path, err);
          }
          Ok(fonts) => {
            entry.path = font_path;
            entry.entries = fonts;
          }
        };

        Some(entry)
      }
    }
  }
}
