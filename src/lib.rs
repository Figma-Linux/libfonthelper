extern crate finder;
extern crate freetype;
extern crate log;
extern crate regex;

pub mod font;
pub mod types;
mod utils;

use finder::{Finder, IntoIter};
use font::Font;
use freetype::{error::Error, Face, Library};
use log::{info, warn};
use types::FontEntry;

pub struct Fonts {
  lib: Library,
  finder: IntoIter,
}

impl Fonts {
  pub fn new(dirs: &Vec<String>) -> Result<Self, Error> {
    match Library::init() {
      Err(err) => return Err(err),
      Ok(lib) => {
        info!("Freetype2 library initialized successful");
        return Ok(Fonts {
          lib,
          finder: Finder::new(&dirs.join(":"))
            .filter(&utils::filter_files)
            .into_iter(),
        });
      }
    };
  }

  pub fn to_json(self) -> String {
    let mut json = "{".to_owned();
    for font in self {
      let string = font.to_json();
      json.push_str(&string[1..string.len() - 1]);
      json.push_str(",");
    }

    json = json[0..json.len() - 1].to_string();
    json.push_str("}");
    json
  }
}

impl Iterator for Fonts {
  type Item = Font;

  fn next(&mut self) -> Option<Font> {
    match self.finder.next() {
      None => return None,
      Some(file) => {
        let font_path = String::from(file.path().to_str().unwrap());
        let mut entry = Font {
          path: "".to_owned(),
          entries: vec![],
        };

        match self.lib.new_face(&font_path, 0) {
          Err(err) => {
            warn!("Cannot open font {}, ERROR: {}", &font_path, err);
            return Some(entry);
          }
          Ok(font) => {
            let font_index = font.num_faces();

            if (!utils::is_correct_font(&font)) {
              warn!("Font '{}' is incorrect! Skip it", &font_path);
              return Some(entry);
            }

            if font_index == 1 {
              let mut values: Vec<FontEntry> = Vec::new();
              values.push(make_fonts(&font));

              entry.path = font_path;
              entry.entries = values;
            } else if font_index > 1 {
              let mut values: Vec<FontEntry> = Vec::new();

              for index in 1..font_index {
                values.push(make_fonts(
                  &self.lib.new_face(&font_path, isize::from(index)).unwrap(),
                ));
              }

              entry.path = font_path;
              entry.entries = values;
            }
          }
        };

        Some(entry)
      }
    }
  }
}

fn make_fonts(face: &Face) -> FontEntry {
  let mut is_italic = false;
  let flags = face.style_flags().bits();

  if flags == 1 || flags == 3 {
    is_italic = true;
  }

  FontEntry {
    postscript: String::from(
      face
        .postscript_name()
        .expect("BUG: Cannot get postscript_name"),
    ),
    family: String::from(face.family_name().expect("BUG: Cannot get family_name")),
    id: String::from(face.family_name().expect("BUG: Cannot get family_name")),
    style: String::from(face.style_name().expect("BUG: Cannot get style_name")),
    weight: 400,
    stretch: 5,
    italic: is_italic,
  }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_get_fonts() {
//         assert_eq!(fonthelper::get_fonts(), "Hello cargo!");
//     }
// }
