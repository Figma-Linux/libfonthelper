extern crate libfonthelper;

use libfonthelper::Fonts;

fn main() {
  let dirs = vec![
    String::from("/usr/share/fonts"),
    String::from("/home/ruut/.local/share/fonts"),
  ];

  let fonts = Fonts::new(&dirs).unwrap();
  println!("{}", fonts.to_json());
}
