# FGBLG
So uhhh.... As you can see, my site currently looks a bit different. 
And that's because I FUCKIN WROTE A STATIC SITE GENERATOR. IN RUST. FROM SCRATCH.

So basically, I was unsatisfied with Hugo for multiple reasons. Mainly the fact that the themes were hard to configure/mod and the fact that the tool itself is a bit hard to configure. Then one day my friend wrote their own static site generator called FAG blog.  
[This tool]( https://airkoala.lol/blog/1-fagblog-stack)  was honestly pretty cool but I sadly have an overinflated ego. So I decided to make something....better. Something more....faggoted. SO I wrote my own static site generator called FaggotBlog in rust for the extra gay vibes :3 .


## How I made it
--- 
So I decided on using markdown (And sadly betraying my bestie org-mode :c) because it was the most widely available/used format for text. So I assumed that it would have more libraries and documentations available. And I was right. So I googled "Rust markdown to html" and clicked the first github link I could find. Which led me to [markdown-rs](https://github.com/wooorm/markdown-rs) which is still in beta. But it was a pretty easy library.
In fact I only neede two damn lines to convert the markdown
```rust
use markdown::to_html;
//and this one:
let html_output = to_html(markdown_input);
```

But it had some glaring issues. Firstly it was in beta. Secondly, it was missing some features. And it, for some reason refused to compile on my friends Arch computer.(For context I use Nix unlike most filthy peasants. So I never had such issues). So I moved over my project to Comrak (Which my friend suggested against due to the fact that it used a bunch of dependencies) since it had more out-of-the-box features. Plus more extensions too. 

So I moved over to it. Surprisingly it was only two lines of code(initially) too!.
```rust
use comrak::{ComrakOptions, markdown_to_html};
let html = markdown_to_html(markdown, &ComrakOptions::default());
```

So after that I needed a way to turn it into a proper html document with CSS and stuff.
For that I just made an html file and put ``{body}`` as the placeholder text inside it to be replaced. Afterwards I just replaced {body} with my code with some basic rust string substitution by just doing replace().

Then I needed CSS to make it look not-ugly. But then I realized something. I suck at CSS and making things look good. So the next part might make you mad. I used....chatgpt to generate the CSS. (everything else was made live during youtube livestreams by myself). Then just injected the CSS by editing the substitution html file.

Then I just wrote some functions for making the output folder structure and copying the output over to the output folder.

Then I needed a homepage. So I just wrote a  function that I stole from someplace partially to take the first few lines and render them seperately.
```rust
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
```


Then I thought I was done for now. But after sending that version to a friend he complained about how he had to use python to host a webserver to view the contents. Then I remembered hugo had a server too. So I wrote a simple quick webserver using Axum.
I contemplated using Rocket or something else which is easy to use but the dumped that idea since I had decided I wanted a way to  improve features and turn FGBLG into a proper tech stack. So that's why I went with Axum ~~Plus  it also had more docs and stuff~~ .
So uhh
that's how I did it

BAI BAIII

# :3
