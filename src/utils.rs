extern crate regex;

use std::fs::DirEntry;

use freetype::Face;
use regex::Regex;

pub fn filter_files(entry: &DirEntry) -> bool {
  let regex = Regex::new(".+\\.(tt[fc]|otf)").unwrap();
  if regex.is_match(entry.file_name().to_str().unwrap()) {
    return true;
  }

  false
}

pub fn is_correct_font(face: &Face) -> bool {
  if let Some(_) = face.postscript_name() {
    true
  } else {
    false
  }
}
