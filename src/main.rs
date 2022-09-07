extern crate clap;
use clap::Parser;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageResult};
use std::fs::{File, read_to_string};
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[clap(
  about = "Create icons for images or emojis",
  version = "0.1.0",
  author = "@egxn on GitHub"
)]
struct Icon {
  #[clap(short = 'e', long = "emoji", help = "Emoji to icon")]
  emoji: Option<char>,
  #[clap(short = 'i', long = "image", help = "Image")]
  image: Option<String>,
  #[clap(short = 's', long = "sizes", default_value = "64", help = "Icon sizes (comma separated)")]
  sizes: String,
  #[clap(short = 'b', long = "background", default_value = "transparent", help = "Background color")]
  background: String,
  #[clap(short = 'o', long = "output", default_value = "icon", help = "Output icon name")]
  output: String,
}

fn main() -> std::io::Result<()> {
  let body: String = read_to_string("./src/template")?.parse().unwrap();

  let icon = Icon::parse();
  let emoji: char = match icon.emoji { Some(emoji) => emoji, None => ' '};
  let sizes: String = icon.sizes;
  let background: String = icon.background;
  let image_path: String = match icon.image { Some(image) => image, None => String::from("") };
  let output: String = icon.output;
  let mut canvas_html = "".to_owned();
  let mut canvas_js = "".to_owned();
  
  let mut img_width = 0;
  let mut img_height = 0;
  let mut data = Vec::new();

  let image_decoded: Option<ImageResult<DynamicImage>> = if !image_path.eq("") {
    Some(ImageReader::open(image_path)?.decode())
  } else {
    None
  };

  match image_decoded {
    Some(img) => {
      let img = img.unwrap();
      img_width = img.width();
      img_height = img.height();
      data = img.into_rgba8().into_raw();
    }
    None => {},
  }
  
  for size in sizes.split(",") {
    canvas_html.push_str(&format!(
      "<canvas width='{}' height='{}' id='canvas_{}'></canvas>",
      size, size, size
    ));

    let draw =  if img_width > 0 && img_height > 0 {
      format!(
        "drawImg({}, {}, {:?}, {}); download({}, '{}');",
        img_width, img_height, data, size, size, output
      )
    } else {
      format!(
        "drawEmoji({}, '{}', '{}'); download({}, '{}');",
        size, emoji, background, size, output
      )
    };

    canvas_js.push_str(&draw);
  }

  let content = body
    .replace("{canvasHtml}", canvas_html.as_str())
    .replace("{canvasJs}", canvas_js.as_str());
  let mut file = File::create("icons.html")?;
  file.write_all(content.as_bytes())?;
  Ok(())
}
