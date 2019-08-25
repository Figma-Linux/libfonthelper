extern crate freetype;
extern crate finder;
extern crate regex;

mod utils;
pub mod types;

use freetype::{Library, Face};
use std::collections::HashMap;
use finder::Finder;
use types::{FontEntry, FontMap};

pub fn get_fonts(directories: &[String]) -> Option<FontMap> {
    match Library::init() {
        Err(err) => {
            println!("Cannot init freetype library, error: {}", err);
            return None;
        },
        Ok(lib) => return Some(handle(&lib, directories)),
    };
}

fn handle(lib: &Library, directories: &[String]) -> FontMap {
    let mut font_map: FontMap = HashMap::new();
    let finder = Finder::new(directories.join(":")).filter(&utils::filter_files);

    for file in finder.into_iter() {
        let font_path = String::from(file.path().to_str().unwrap());

        match lib.new_face(&font_path, 0) {
            Err(err) => {
                println!("Cannot open font {}, ERROR: {}", &font_path, err);
                continue;
            },
            Ok(font) => {
                let font_index = font.num_faces();

                if font_index == 1 {
                    let mut values: Vec<FontEntry> = Vec::new();
                    values.push(make_fonts(&font));

                    font_map.insert(font_path, values);
                } else if font_index > 1 {
                    let mut values: Vec<FontEntry> = Vec::new();

                    for index in 1..font_index {
                        values.push(make_fonts(&lib.new_face(&font_path, isize::from(index)).unwrap()));
                    }

                    font_map.insert(String::from(&font_path), values);
                }
            },
        };
    }

    font_map
}

fn make_fonts(face: &Face) -> FontEntry {
    FontEntry {
        postscript: String::from(face.postscript_name().expect("BUG: Cannot get postscript_name")),
        family: String::from(face.family_name().expect("BUG: Cannot get family_name")),
        id: String::from(face.family_name().expect("BUG: Cannot get family_name")),
        style: String::from(face.style_name().expect("BUG: Cannot get style_name")),
        weight: 400,
        stretch: face.num_faces() as i64,
        italic: false,
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
