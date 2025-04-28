use comrak::{markdown_to_html, ComrakOptions};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

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

//takes the first few lines from the markdown and converts it on the side and returns it so I can
//preview it
fn extract_preview(content: &str, max_length: usize) -> String {
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

fn process_markdown_file(file_path: &str, template: &str) -> io::Result<(String, String)> {
    let file = fs::read_to_string(file_path).expect("Failed to read markdown file");
    let preview = extract_preview(&file, 200);
    let html_body = markdown_to_html(&file, &ComrakOptions::default());
    let full_html = template.replace("{body}", &html_body);
    Ok((full_html, preview))
}

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

    let template = fs::read_to_string("templates/template.html")?;
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

fn main() -> io::Result<()> {
    make_folders()?;

    if Path::new("templates/style.css").exists() {
        fs::copy("templates/style.css", "output/css/style.css")?;
    }

    let template = fs::read_to_string("templates/template.html")?;
    let mut posts = Vec::new();
    let posts_dir = Path::new("posts");

    if posts_dir.exists() {
        for entry in fs::read_dir(posts_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
                let (html, preview) = process_markdown_file(path.to_str().unwrap(), &template)?;

                let mut outfile = fs::File::create(format!("output/posts/{}.html", file_name))?;
                outfile.write_all(html.as_bytes())?;

                posts.push((file_name.clone(), file_name, preview));
            }
        }
    }

    let home_html = generate_home_page(&posts)?;
    let mut home_file = fs::File::create("output/home.html")?;
    home_file.write_all(home_html.as_bytes())?;

    Ok(())
}
