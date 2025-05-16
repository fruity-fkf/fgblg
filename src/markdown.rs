use crate::html;
use comrak::{ComrakOptions, markdown_to_html};
use std::{fs, io};

pub fn process_markdown_file(file_path: &str, template: &str) -> io::Result<(String, String)> {
    let file = fs::read_to_string(file_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to read markdown file: {}", e),
        )
    })?;
    // for now using 200 as the preview size
    let preview = html::extract_preview(&file, 200);

    // adding extensions to comrak for stuff
    let mut options = ComrakOptions::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.parse.smart = true;
    options.render.unsafe_ = true; // needed for raw HTML 

    let html_body = markdown_to_html(&file, &options);
    let full_html = template.replace("{body}", &html_body);

    Ok((full_html, preview))
}
