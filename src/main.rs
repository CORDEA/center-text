extern crate image;
extern crate imageproc;
extern crate rusttype;

use imageproc::drawing::draw_text_mut;
use image::Rgba;
use rusttype::{FontCollection, Scale};

fn main() {
    let mut image = image::open("white.png").unwrap();

    let font = Vec::from(include_bytes!("Roboto-Black.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();

    let scale = Scale { x: 100.0, y: 100.0 };
    draw_text_mut(&mut image, Rgba([0u8, 0u8, 0u8, 1u8]), 0, 0, scale, &font, "hello");

    let _ = image.save("hello.png").unwrap();
}
