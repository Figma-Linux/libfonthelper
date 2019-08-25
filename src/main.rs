extern crate libfonthelper;

fn main() {
    let dirs = vec![
        String::from("/usr/share/fonts"),
    ];

    match libfonthelper::get_fonts(&dirs) {
        Err(err) => println!("{}", err),
        Ok(fonts) => {
            for (path, face) in fonts {
                println!("{} | {} | {}", path, face.get(0).unwrap().italic, face.len());
            }
        }
    };
}
