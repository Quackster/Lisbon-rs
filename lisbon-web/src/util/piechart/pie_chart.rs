//! Mirrors `org.alexdev.http.util.piechart.PieChart`.

use std::f64::consts::PI;

use image::{GenericImageView, ImageEncoder};

use crate::util::bitmap_font;
use crate::util::piechart::slice::{Color, Slice};

/// Mirrors `org.alexdev.http.util.piechart.PieChart`.
pub struct PieChart {
    slices: Vec<Slice>,
    image: Vec<u8>,
}

impl PieChart {
    /// Mirrors the constructor `PieChart(BufferedImage, List<Slice>)`.
    ///
    /// The Java constructor draws the pie (fillArc) and the legend
    /// (drawString) onto the passed image; here the passed bytes provide
    /// the canvas size (a 500x250 canvas when they are not a decodable
    /// image) and the pie + legend are rendered into a fresh PNG.
    pub fn new(image: Vec<u8>, slices: Vec<Slice>) -> Self {
        let mut slices = slices;
        slices.sort_by(|a, b| {
            a.get_value()
                .partial_cmp(&b.get_value())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let (width, height) = Self::canvas_size(&image);

        let mut canvas = image::RgbaImage::from_pixel(
            width,
            height,
            image::Rgba([255, 255, 255, 255]),
        );

        Self::draw_pie(&mut canvas, width / 2, 0, width / 2, height, &slices);
        Self::draw_legend(&mut canvas, &slices);

        let image = Self::encode_png(&canvas);
        Self { slices, image }
    }

    /// The rendered pie + legend, PNG encoded.
    pub fn get_image(&self) -> &[u8] {
        &self.image
    }

    /// The slices, sorted by value (as in the Java constructor).
    pub fn get_slices(&self) -> &[Slice] {
        &self.slices
    }

    fn canvas_size(image: &[u8]) -> (u32, u32) {
        if image.is_empty() {
            return (500, 250);
        }

        match image::io::Reader::new(std::io::Cursor::new(image))
            .with_guessed_format()
            .map_err(|error| error.to_string())
            .and_then(|reader| reader.decode().map_err(|error| error.to_string()))
        {
            Ok(decoded) => decoded.dimensions(),
            Err(_) => (500, 250),
        }
    }

    /// Mirrors `drawPie(Graphics2D, Rectangle)`.
    fn draw_pie(
        canvas: &mut image::RgbaImage,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        slices: &[Slice],
    ) {
        let total: f64 = slices.iter().map(|slice| slice.get_value()).sum();

        if total <= 0.0 || width == 0 || height == 0 {
            return;
        }

        let mut cur_value = 0.0;

        for slice in slices {
            let start_angle = (cur_value * 361.0 / total) as i32;
            let arc_angle = (slice.get_value() * 361.0 / total) as i32;

            Self::fill_arc(canvas, x, y, width, height, start_angle, arc_angle, slice.get_color());

            cur_value += slice.get_value();
        }
    }

    /// The AWT `fillArc` equivalent: the angles are in degrees, `0` is the
    /// 3 o'clock position and positive angles are counter-clockwise.
    fn fill_arc(
        canvas: &mut image::RgbaImage,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        start: i32,
        arc: i32,
        color: &Color,
    ) {
        if arc <= 0 {
            return;
        }

        let rgba = image::Rgba([
            color.red,
            color.green,
            color.blue,
            color.alpha,
        ]);
        let half_width = width as f64 / 2.0;
        let half_height = height as f64 / 2.0;
        let covered = (arc as f64).min(360.0);

        for py in y..(y + height).min(canvas.height()) {
            for px in x..(x + width).min(canvas.width()) {
                let u = (2.0 * ((px as f64 + 0.5) - x as f64 - half_width)) / width as f64;
                let v = (2.0 * ((py as f64 + 0.5) - y as f64 - half_height)) / height as f64;

                if u * u + v * v > 1.0 {
                    continue;
                }

                let angle = -(v.atan2(u) * 180.0 / PI);
                let delta = (angle - start as f64).rem_euclid(360.0);

                if delta < covered {
                    canvas.put_pixel(px, py, rgba);
                }
            }
        }
    }

    /// Mirrors `drawLegend()`.
    fn draw_legend(canvas: &mut image::RgbaImage, slices: &[Slice]) {
        let mut y = 40;
        let x = (canvas.width() as i32) / 2 + 10;

        for slice in slices {
            let color = slice.get_color();
            Self::fill_rect(canvas, x, y, 5, 5, image::Rgba([
                color.red,
                color.green,
                color.blue,
                color.alpha,
            ]));

            let mut label_x = x + 10;
            for character in slice.get_label().chars() {
                if let Some(glyph) = bitmap_font::glyph(character) {
                    Self::draw_glyph(canvas, label_x, y + 6, glyph, 2, image::Rgba([0, 0, 0, 255]));
                    label_x += 12;
                }
            }

            y += 20;
        }
    }

    fn draw_glyph(
        canvas: &mut image::RgbaImage,
        x: i32,
        y: i32,
        glyph: bitmap_font::Glyph,
        block: i32,
        color: image::Rgba<u8>,
    ) {
        for (column, row) in bitmap_font::set_pixels(glyph) {
            Self::fill_rect(
                canvas,
                x + (column as i32) * block,
                y + (row as i32) * block,
                block,
                block,
                color,
            );
        }
    }

    fn fill_rect(
        canvas: &mut image::RgbaImage,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: image::Rgba<u8>,
    ) {
        for dy in 0..height {
            for dx in 0..width {
                let px = x + dx;
                let py = y + dy;

                if px >= 0 && px < canvas.width() as i32 && py >= 0 && py < canvas.height() as i32 {
                    canvas.put_pixel(px as u32, py as u32, color);
                }
            }
        }
    }

    fn encode_png(canvas: &image::RgbaImage) -> Vec<u8> {
        let mut bytes = Vec::new();
        image::codecs::png::PngEncoder::new(&mut bytes)
            .write_image(
                canvas.as_raw(),
                canvas.width(),
                canvas.height(),
                image::ColorType::Rgba8,
            )
            .expect("pie chart PNG encoding failed");
        bytes
    }
}
