use crate::html;
use orgize::Org;
use std::{fs, io};

/*
For Org mode I'm just usign Orgize. this is pretty much a 1:1 recreation of the markdown.rs file.
All I did differently is just the replaced the mm handler function with the org mode handler function


*/

pub fn process_org_file(file_path: &str, template: &str) -> io::Result<(String, String)> {
    let file = fs::read_to_string(file_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to read org file: {}", e),
        )
    })?;

    let preview = html::extract_preview(&file, 200);

    // Parse the org file and convert to HTML
    let html_body = Org::parse(&file).to

    let full_html = template.replace("{body}", &html_body);

    Ok((full_html, preview))
}
