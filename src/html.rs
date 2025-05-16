use std::{
    fs,
    io::{self},
};
pub fn generate_home_page(posts: &[(String, String, String)]) -> io::Result<String> {
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

    let template = fs::read_to_string("templates/template.html").map_err(|e| {
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
