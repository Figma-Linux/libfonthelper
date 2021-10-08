extern crate regex;

use std::fs::DirEntry;

use regex::Regex;

pub fn filter_files(entry: &DirEntry) -> bool {
  let regex = Regex::new(".+\\.(tt[fc]|otf)").unwrap();
  if regex.is_match(entry.file_name().to_str().unwrap()) {
    return true;
  }

  false
}
