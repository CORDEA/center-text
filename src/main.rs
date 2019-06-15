extern crate image;
extern crate imageproc;
extern crate rusttype;

use imageproc::drawing::{draw_text_mut, draw_hollow_rect_mut};
use image::{GenericImageView, Rgba};
use rusttype::{FontCollection, Scale, Rect, point};

fn main() {
    let mut image = image::open("white.png").unwrap();

    let font = Vec::from(include_bytes!("Roboto-Black.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();

    let text = "rustpythonclojureC#";
    let scale = Scale { x: 100.0, y: 100.0 };
    let point = point(0.0, font.v_metrics(scale).ascent);

    let (w, h) = image.dimensions();
    let glyphs: Vec<Rect<i32>> = font.layout(text, scale, point)
        .map(|g| g.pixel_bounding_box().unwrap())
        .collect();

    let first = glyphs.first().unwrap().min;
    let last = glyphs.last().unwrap().max;
    let min_y = glyphs.iter().map(|g| g.min.y).min().unwrap();
    let max_y = glyphs.iter().map(|g| g.max.y).max().unwrap();
    let height = max_y - min_y;
    let width = last.x - first.x;
    let center_x = (w / 2) - (width / 2) as u32 - first.x as u32;
    let center_y = (h / 2) - (height / 2) as u32 - min_y as u32;

    draw_text_mut(&mut image, Rgba([0, 0, 0, 255]), center_x, center_y, scale, &font, text);

    let red = Rgba([255, 0, 0, 255]);
    for glyph in glyphs.iter() {
        let rect = imageproc::rect::Rect::at(glyph.min.x, glyph.min.y)
            .of_size(glyph.width() as u32, glyph.height() as u32);
        draw_hollow_rect_mut(&mut image, rect, red);
    }

    let blue = Rgba([0, 255, 0, 255]);

    let first = glyphs.first().unwrap().min;
    let last = glyphs.last().unwrap().max;
    let min_y = glyphs.iter().map(|g| g.min.y).min().unwrap();
    let max_y = glyphs.iter().map(|g| g.max.y).max().unwrap();

    let rect = imageproc::rect::Rect::at(first.x, min_y)
        .of_size((last.x - first.x) as u32, (max_y - min_y) as u32);
    draw_hollow_rect_mut(&mut image, rect, blue);

    let _ = image.save("hello.png").unwrap();
}
