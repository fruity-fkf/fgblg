use crate::toml_config::Config;
use clap::{Parser, command};
use std::{fs, io, path::Path, process};
mod files;
mod html;
mod markdown;
mod toml_config;
//TODO: fix org mode later
// mod org;
mod server;

//command line parser

#[derive(Parser)]
#[command(
    name = "Fgblg",                   // Binary name
    about = "FKF's blogging tool. :3", // --help and --about section
    version = "1.0.2",               // --version info
)]
struct Args {}

#[tokio::main]
async fn main() -> io::Result<()> {
    let config: Config = toml_config::read_config("config.toml").expect("Failed to read config.toml");

    // Construct file paths from config
    let css_file = format!("templates/style-{}.css", config.theme);
    let html_file = format!("templates/{}.html", config.template);
    let home_template = format!("templates/{}.html", config.home_template);

    // making the folder/file tree and copying shit there
    files::make_folders()?;
    //copy over the css
    if Path::new(&css_file).exists() {
        fs::copy(css_file, "output/css/style.css").map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to copy CSS: {}", e))
        })?;
    }

    // read the template
    let mut template = fs::read_to_string(&html_file).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to read template: {}", e),
        )
    })?;

    // Add highlight.js theme
    template = template.replace("{highlight_theme}", &config.code_theme);

    //  vector for holding all of the posts.....
    let mut posts = Vec::new();
    // This following codeblock was a battleground for me learning to mess around with match
    // statements

    let posts_dir = Path::new("posts");
    match posts_dir.exists() {
        true => println!("Found the posts directory. Generating the blog......"),
        false => {
            eprintln!("FUCKING MORON. You forgot to make the posts folder");
            process::exit(1);
        }
    }

    //warzone ends here

    //read posts recursively... (note: Change it to a better comment later because this just looks
    //generic af )
    if posts_dir.exists() {
        for entry in fs::read_dir(posts_dir)? {
            let entry = entry?;
            let path = entry.path();

            let file_name = path
                .file_stem()
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid file name"))?
                .to_string_lossy()
                .to_string();

            let (html, preview) = match path.extension().and_then(|s| s.to_str()) {
                Some("md") => markdown::process_markdown_file(
                    path.to_str().ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            "File path invalid error. Shouldn't even be happening. WTF Went wrong?",
                        )
                    })?,
                    &template,
                )?,
                _ => continue,
            };

            fs::write(format!("output/posts/{}.html", file_name), html.as_bytes()).map_err(
                |e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to write post file: {}", e),
                    )
                },
            )?;

            posts.push((file_name.clone(), file_name, preview));
        }
    }

    let home_html = html::generate_home_page(&posts, &home_template)?;
    fs::write("output/index.html", home_html.as_bytes()).map_err(|e| {
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
