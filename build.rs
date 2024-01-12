use std::convert::TryInto;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use unicode_normalization::UnicodeNormalization;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let mut map =
        std::io::BufWriter::new(fs::File::create(Path::new(&out_dir).join("map.bin")).unwrap());

    let mut chars = String::with_capacity(9);

    (0..=(char::MAX as u32)).for_each(|index| {
        chars.clear();

        let mut buf = [0u8; 10];

        if let Some(c) = char::from_u32(index) {
            c.to_lowercase()
                .nfd()
                .filter(|c| c.is_ascii() || c.is_alphanumeric())
                .for_each(|c| chars.push(c));
        }

        buf[0] = chars.len().try_into().expect("u8");
        buf[1..(chars.len() + 1)].copy_from_slice(chars.as_bytes());

        map.write_all(&buf).unwrap();
    });

    println!("cargo:rerun-if-changed=build.rs");
}
