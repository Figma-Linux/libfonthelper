extern crate libfonthelper;

use libfonthelper::FontsHelper;

fn main() {
  let dirs = vec![
    String::from("/usr/share/fonts"),
    String::from("/home/ruut/.local/share/fonts"),
    String::from("/home/ruut/.local/share/bad_fonts"),
  ];

  let mut fonts = FontsHelper::new(&dirs);

  let mut font = fonts.next().unwrap();
  println!("Font entry: {}, count: {}", font.path, font.entries.len());

  font = fonts.next().unwrap();
  println!("Font entry: {}, count: {}", font.path, font.entries.len());

  font = fonts.next().unwrap();
  println!("Font entry: {}, count: {}", font.path, font.entries.len());

  font = fonts.next().unwrap();
  println!("Font entry: {}, count: {}", font.path, font.entries.len());

  font = fonts.next().unwrap();
  println!("Font entry: {}, count: {}", font.path, font.entries.len());

  font = fonts.next().unwrap();
  println!("Font entry: {}, count: {}", font.path, font.entries.len());
}
