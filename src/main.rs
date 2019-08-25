extern crate libfonthelper;

fn main() {
    let dirs = vec![
        String::from("/usr/share/fonts"),
    ];

    if let Some(fonts) = libfonthelper::get_fonts(&dirs) {
        for (path, face) in fonts {
            println!("{} | {} | {}", path, face.get(0).unwrap().italic, face.len());
        }
    }
}
