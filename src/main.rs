use std::{
    fs,
    io::{self, Write},
    path::Path,
};
mod markdown;
mod server;

fn make_folders() -> io::Result<()> {
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

// the main function that actually generates the previews for the homepage
fn generate_home_page(posts: &[(String, String, String)]) -> io::Result<String> {
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

#[tokio::main]
async fn main() -> io::Result<()> {
    // making the folder/file tree and copying shit there
    make_folders()?;
    if Path::new("templates/style.css").exists() {
        fs::copy("templates/style.css", "output/css/style.css").map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to copy CSS: {}", e))
        })?;
    }

    // read the tenplate
    let template = fs::read_to_string("templates/template.html").map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to read template: {}", e),
        )
    })?;

    // read posts
    let mut posts = Vec::new();
    let posts_dir = Path::new("posts");

    //read posts recursively
    if posts_dir.exists() {
        for entry in fs::read_dir(posts_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let file_name = path
                    .file_stem()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid file name"))?
                    .to_string_lossy()
                    .to_string();

                // generate the html of the same name

                let (html, preview) = markdown::process_markdown_file(
                    path.to_str()
                        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid file path"))?,
                    &template,
                )?;

                let mut outfile = fs::File::create(format!("output/posts/{}.html", file_name))
                    .map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            format!("Failed to create post file: {}", e),
                        )
                    })?;
                outfile.write_all(html.as_bytes()).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to write post file: {}", e),
                    )
                })?;

                posts.push((file_name.clone(), file_name, preview));
            }
        }
    }
    let home_html = generate_home_page(&posts)?;
    let mut home_file = fs::File::create("output/index.html").map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create index file: {}", e),
        )
    })?;
    home_file.write_all(home_html.as_bytes()).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to write index file: {}", e),
        )
    })?;

    println!("Files have been generated! :3");
    println!("fkf likes boys");

    server::serve_web("output", 3000).await?;

    Ok(())
}
