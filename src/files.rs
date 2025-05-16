use std::{fs, io, path::Path};
pub fn make_folders() -> io::Result<()> {
    if !Path::new("output").exists() {
        fs::create_dir("output")?;
    }
    if !Path::new("output/about").exists() {
        fs::create_dir("output/about")?;
    }
    if !Path::new("output/posts").exists() {
        fs::create_dir("output/posts")?;
    }
    if !Path::new("output/css").exists() {
        fs::create_dir("output/css")?;
    }
    Ok(())
}

pub fn detect_filetype(path: &Path) -> Option<&'static str> {
    path.extension()
        .and_then(|s| s.to_str())
        .and_then(|ext| match ext {
            "md" | "markdown" => Some("markdown"),
            "org" => Some("org"),
            _ => None,
        })
}
