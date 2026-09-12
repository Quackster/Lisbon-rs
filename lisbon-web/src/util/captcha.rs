//! Mirrors `org.alexdev.http.util.Captcha`.

use std::f32::consts::PI;

use rand::Rng;

use image::ImageEncoder;
use lisbon_server::util::string_util::StringUtil;

use crate::duckhttpd::web_connection::WebConnection;
use crate::util::bitmap_font;

const DATA: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

const WIDTH: i32 = 200;
const HEIGHT: i32 = 50;
const BLACK: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);
const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);
const GLYPH_BASE_SIZE: f32 = 5.0;

/// Mirrors `org.alexdev.http.util.Captcha`.
#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
pub struct Captcha;

impl Captcha {
    /// Mirrors the no-arg constructor.
    pub fn new() -> Self {
        Self
    }

    /// Mirrors `createHash()`.
    pub fn create_hash(&self) -> Option<String> {
        let text = Self::generate_text(32);
        Some(StringUtil::md5(&text))
    }

    /// Mirrors `generateText(int)`.
    pub fn generate_text(length: i32) -> String {
        let mut rng = rand::thread_rng();
        let mut out = String::with_capacity(length.max(0) as usize);
        for _ in 0..(length - 1) {
            out.push(DATA[rng.gen_range(0..DATA.len())]);
        }
        out.to_lowercase()
    }

    /// Mirrors `generateImage(String)`.
    ///
    /// The Java AWT rendering (white canvas, 8 random black lines, the text
    /// drawn rotated / scaled per character) is reproduced with a 5x7 bitmap
    /// font, since the `image` crate has no text engine.
    pub fn generate_image(text: &str) -> Vec<u8> {
        if text.is_empty() {
            panic!("No captcha text given");
        }

        let mut image = image::RgbaImage::from_pixel(WIDTH as u32, HEIGHT as u32, WHITE);

        let mut rng = rand::thread_rng();
        for _ in 0..8 {
            let old_x = rng.gen_range(0..WIDTH);
            let old_y = rng.gen_range(0..HEIGHT);
            let new_x = rng.gen_range(0..HEIGHT);
            let new_y = rng.gen_range(0..HEIGHT);
            Self::draw_line(&mut image, old_x, old_y, new_x, new_y);
        }

        let mut x_pos = 20;
        let char_count = text.chars().count().max(1) as i32;

        for character in text.chars() {
            let char_max_width = WIDTH / char_count - rng.gen_range(0..20);
            Self::draw_character(&mut image, character, x_pos, char_max_width);
            x_pos += char_max_width;
        }

        let mut bytes = Vec::new();
        image::codecs::png::PngEncoder::new(&mut bytes)
            .write_image(
                image.as_raw(),
                image.width(),
                image.height(),
                image::ColorType::Rgba8,
            )
            .expect("captcha PNG encoding failed");

        bytes
    }

    fn draw_character(
        image: &mut image::RgbaImage,
        character: char,
        x: i32,
        box_width: i32,
    ) {
        let glyph = match bitmap_font::glyph(character) {
            Some(glyph) => glyph,
            None => return,
        };

        let mut rng = rand::thread_rng();
        let degree = rng.gen_range(-15.0..15.0);
        let scale = 1.0 - rng.gen_range(0.0..0.15);
        let bold = rng.gen_bool(0.5);

        let theta = degree * PI / 180.0;
        let cos = theta.cos();
        let sin = theta.sin();
        let block = GLYPH_BASE_SIZE * scale;
        let block_size = block.ceil() as i32;
        let thickness = if bold { 2 } else { 1 };

        for (column, row) in bitmap_font::set_pixels(glyph) {
            for extra in 0..thickness {
                let cx = column as f32 + 0.5 - 2.5;
                let cy = row as f32 + 0.5 - 3.5;
                let sx = cx * block + (extra as f32) * (block / thickness as f32);
                let sy = cy * block;

                let rx = sx * cos - sy * sin;
                let ry = sx * sin + sy * cos;

                let dst_x = (x as f32 + box_width as f32 / 2.0 + rx).round() as i32;
                let dst_y = ((HEIGHT / 2) as f32 + ry).round() as i32;

                Self::fill_block(image, dst_x, dst_y, block_size);
            }
        }
    }

    fn fill_block(image: &mut image::RgbaImage, x: i32, y: i32, size: i32) {
        for dy in 0..size {
            for dx in 0..size {
                let px = x + dx;
                let py = y + dy;

                if px >= 0 && px < image.width() as i32 && py >= 0 && py < image.height() as i32 {
                    image.put_pixel(px as u32, py as u32, BLACK);
                }
            }
        }
    }

    /// Mirrors `matches(WebConnection, String)`.
    pub fn matches(web_connection: &WebConnection, captcha: &str) -> bool {
        match web_connection.session().get_string("captcha-text") {
            Some(generated) if !generated.trim().is_empty() => generated == captcha,
            _ => false,
        }
    }

    fn draw_line(image: &mut image::RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        let mut x = x0;
        let mut y = y0;

        loop {
            Self::fill_block(image, x, y, 1);

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
}
