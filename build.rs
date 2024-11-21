use std::fs;
use std::path::Path;

const ASSETS_DIR: &str = "assets";

fn generate_filename_consts(out_dir: &str) {
    let entries: Vec<_> = fs::read_dir(ASSETS_DIR)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                let stem = path.file_stem()?.to_str()?.to_uppercase();
                let filename = path.file_name()?.to_str()?.to_string();
                Some((stem, filename))
            } else {
                None
            }
        })
        .collect();

    let const_code = entries
        .iter()
        .map(|(const_name, filename)| {
            format!(
                "/// Asset file: {}\npub const {}_FILE: &str = \"{}\";",
                filename, const_name, filename
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(Path::new(out_dir).join("filename_consts.rs"), const_code).unwrap();
}

fn main() {
    let out_dir = &std::env::var("OUT_DIR").unwrap();
    generate_filename_consts(out_dir);
    println!("cargo:rerun-if-changed={}", ASSETS_DIR);
}
