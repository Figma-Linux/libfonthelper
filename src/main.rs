extern crate fonthelper;

fn main() {
    let dirs = vec![
        String::from("/usr/share/fonts"),
    ];

    match fonthelper::get_fonts(&dirs) {
        Err(err) => println!("ERROR: {}", err),
        Ok(fonts) => {
            for (path, face) in fonts {
                println!("{} | {}", path, face.postscript);
            }
        }
    }
}
