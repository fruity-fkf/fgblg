use std::{
    fs,
    io::{self},
};
pub fn extract_preview(content: &str, max_length: usize) -> String {
    let clean_content = content
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect::<Vec<&str>>()
        .join(" ");
    let preview = clean_content.chars().take(max_length).collect::<String>();
    if clean_content.len() > max_length {
        format!("{}...", preview)
    } else {
        preview
    }
}
pub fn generate_home_page(posts: &[(String, String, String)],  html_file: &str) -> io::Result<String> {
    let mut posts_html = String::new();

    for (title, path, preview) in posts {
        posts_html.push_str(&format!(
            r#"<article class="blog-post">
                <h2><a href="/posts/{}.html">{}</a></h2>
                <p class="post-preview">{}</p>
                <a href="/posts/{}.html" class="read-more">Read more →</a>
            </article>"#,
            path, title, preview, path
        ));
    }

    let template = fs::read_to_string(html_file).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to read template: {}", e),
        )
    })?;
    let home_html = template.replace(
        "{body}",
        &format!(
            r#"<h1>Blog Posts</h1>
        <div class="blog-posts">
            {}
        </div>"#,
            posts_html
        ),
    );

    Ok(home_html)
}
