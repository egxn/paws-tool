extern crate clap;
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about = "Create icons from emojis", version = "0.1.0", author = "@egxn on GitHub")]
struct Icon {
   #[clap(short = 'e', long = "emoji", required = true, help = "Emoji to icon")]
   emoji: char,
   #[clap(short = 's', long = "sizes" , default_value = "64", help = "Icon sizes 64,128,...")]
   sizes: String,
   #[clap(short = 'b', long = "background", default_value = "transparent", help = "Background icon color")]
   background: String,
}

fn main() -> std::io::Result<()> {
    let body = "
    <html lang='en'>
    <head><meta charset='UTF-8'><meta name='viewport' content='width=device-width, initial-scale=1.0'><title>Paws</title></head>
    <body><div>{canvasHtml}</div></body>
    <script>
        function draw(size, emoji, background) {
          const ctx = document.getElementById(`canvas_${size}`).getContext('2d');
          ctx.font = `${size-4}px serif`;
          ctx.textAlign = 'center';
          ctx.textBaseline = 'middle';
          ctx.fillText(emoji, (size / 2), (size / 2) + ((size/2)*0.08) );
        }
        function download(id){
          const link = document.createElement('a');
          link.download = `icon_${id}.png`;
          link.href = document.getElementById(`canvas_${id}`).toDataURL();
          link.click();
        }
        {canvasJs}
    </script>
    </html>";

    let icon = Icon::parse();
    let emoji = icon.emoji;
    let sizes = icon.sizes;
    let background = icon.background;
    let mut canvas_html = "".to_owned();
    let mut canvas_js = "".to_owned();

    for size in sizes.split(",") {
        canvas_html.push_str(&format!("<canvas width='{}' height='{}' id='canvas_{}'></canvas>", size, size, size));
        canvas_js.push_str(&format!("draw({}, '{}', '{}'); download({});", size, emoji, background, size));
    }

    let content = body.replace("{canvasHtml}", canvas_html.as_str())
                      .replace("{canvasJs}", canvas_js.as_str());
    let mut file = File::create("index.html")?;
    file.write_all(content.as_bytes())?;
    Ok(())
}