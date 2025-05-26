use std::{fs, io, path::Path, process};

mod files;
mod html;
mod markdown;
mod org;
mod server;

#[tokio::main]
async fn main() -> io::Result<()> {
    // making the folder/file tree and copying shit there
    files::make_folders()?;
    //copy over the css
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

            /*
            useing match here to check if the file ends with .md or .org and then just
            converting them with whichever function is appropriate
            */
            let (html, preview) = match path.extension().and_then(|s| s.to_str()) {
                Some("md") => markdown::process_markdown_file(
                    path.to_str()
                        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid file path"))?,
                    &template,
                )?,
                Some("org") => match org::process_org_file(
                    path.to_str()
                        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid file path"))?,
                    &template,
                ) {
                    Ok(result) => result,
                    Err(e) => {
                        eprintln!("Error processing org file {}: {}", path.display(), e);
                        continue;
                    }
                },
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

    let home_html = html::generate_home_page(&posts)?;
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
