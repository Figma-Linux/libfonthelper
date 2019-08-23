extern crate freetype;
extern crate finder;
extern crate regex;

mod utils;
mod types;

use freetype::{Library, Error, Face};
use std::collections::HashMap;
use finder::Finder;
use types::{FontEntry, FontMap};

pub fn get_fonts(directories: &[String]) -> Result<FontMap, Error> {
    match Library::init() {
        Err(err) => return Err(err),
        Ok(lib) => return handle(&lib, directories),
    };
}

fn handle(lib: &Library, directories: &[String]) -> Result<FontMap, Error> {
    let mut font_map: FontMap = HashMap::new();
    let finder = Finder::new(directories.join(":")).filter(&utils::filter_files);

    for file in finder.into_iter() {
        let font_path = String::from(file.path().to_str().unwrap());

        match lib.new_face(&font_path, 0) {
            Ok(font) => font_map.insert(font_path, handle_font(&font)),
            Err(err) => {
                println!("Cannot open font {}, ERROR: {}", &font_path, err);
                continue;
            },
        };
    }

    Ok(font_map)
}

fn handle_font(face: &Face) -> FontEntry {
    FontEntry {
        postscript: String::from(face.postscript_name().expect("BUG: Cannot get postscript_name")),
        family: String::from(face.family_name().expect("BUG: Cannot get family_name")),
        id: String::from(face.family_name().expect("BUG: Cannot get family_name")),
        style: String::from(face.style_name().expect("BUG: Cannot get style_name")),
        weight: 400,
        stretch: 4,
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
