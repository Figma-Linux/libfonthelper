extern crate libfonthelper;

use libfonthelper::FontsHelper;

fn main() {
  let dirs = vec![
    String::from("/usr/share/fonts"),
    String::from("/home/ruut/.local/share/fonts"),
    String::from("/home/ruut/.local/share/bad_fonts"),
  ];

  let fonts = FontsHelper::new(&dirs);

  for font in fonts {
    println!("Font entry: {}, count: {}", font.path, font.entries.len());
  }
}
