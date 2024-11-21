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

    let mut code = String::new();

    // Generate individual constants
    for (const_name, filename) in &entries {
        code.push_str(&format!(
            "/// Asset file: {}\npub const {}_FILE: &str = \"{}\";\n\n",
            filename, const_name, filename
        ));
    }

    // Generate the array of all filenames
    code.push_str("/// Array containing all asset filenames\n");
    code.push_str("pub const ALL_ASSET_FILES: &[&str] = &[\n");
    for (const_name, _) in &entries {
        code.push_str(&format!("    {}_FILE,\n", const_name));
    }
    code.push_str("];\n");

    fs::write(Path::new(out_dir).join("filename_consts.rs"), code).unwrap();
}

fn main() {
    let out_dir = &std::env::var("OUT_DIR").unwrap();
    generate_filename_consts(out_dir);
    println!("cargo:rerun-if-changed={}", ASSETS_DIR);
}
