use crate::color::Color;
use glam::Vec3A;
use image::io::Reader as ImageReader;
use log::{debug, info};
use rayon::prelude::*;
use std::{fs, io::Cursor};

#[derive(Debug, PartialEq)]
pub struct Canvas {
    width: u16,
    height: u16,
    pixels: Vec<Vec<Vec3A>>,
}

impl Canvas {
    pub fn new_black_canvas(width: u16, height: u16) -> Self {
        info!("Creating new black canvas, width: {width}, height: {height}");
        Self {
            width,
            height,
            pixels: vec![vec![Vec3A::new_black(); width as usize]; height as usize],
        }
    }

    pub fn new_with_initial_color(width: u16, height: u16, init_color: Vec3A) -> Self {
        info!("Creating new canvas with provided initial color, width: {width}, height: {height}");
        info!(
            "Initial canvas color as f32 values (0.0 - 1.0) R: {}, G: {}, B: {}",
            init_color.get_red_val(),
            init_color.get_green_val(),
            init_color.get_blue_val()
        );
        info!(
            "Initial canvas color as u8 values (0 - 255) R: {}, G: {}, B: {}",
            init_color.get_red_val_as_u8(),
            init_color.get_green_val_as_u8(),
            init_color.get_blue_val_as_u8()
        );
        Self {
            width,
            height,
            pixels: vec![vec![init_color; width as usize]; height as usize],
        }
    }

    pub fn write_pixel(&mut self, x: u16, y: u16, color: Vec3A) {
        if let Some(row) = self.pixels.get_mut(y as usize) {
            if let Some(pixel) = row.get_mut(x as usize) {
                *pixel = color;
            } else {
                panic!(
                    "Column index is out of bounds when writing pixel to canvas, x: {}",
                    x
                );
            }
        } else {
            panic!(
                "Row index is out of bounds when writing pixel to canvas, y: {}",
                y
            );
        }
    }

    pub fn pixel_at(&mut self, x: u16, y: u16) -> &Vec3A {
        if let Some(row) = self.pixels.get(y as usize) {
            if let Some(pixel) = row.get(x as usize) {
                pixel
            } else {
                panic!(
                    "Column index is out of bounds when getting pixel from canvas, x: {}",
                    x
                );
            }
        } else {
            panic!(
                "Row index is out of bounds when getting pixel from canvas, y: {}",
                y
            );
        }
    }

    fn export_as_ppm(&self) -> String {
        let total_pixels: u64 = self.width as u64 * self.height as u64;
        info!("Exporting canvas as PPM");
        debug!(
            "Exported canvas width: {}, height: {}, total pixels: {}",
            self.width, self.height, total_pixels
        );

        // TODO: Use image create directly instead of creating PPM string for better flexibility
        // FIXME: use .fold() instead of format!() - https://rust-lang.github.io/rust-clippy/master/index.html#/format_collect
        let ppm_data: String = self
            .pixels
            .par_iter()
            .flatten()
            .map(|color| {
                let r = color.get_red_val_as_u8();
                let g = color.get_green_val_as_u8();
                let b = color.get_blue_val_as_u8();
                format!("{r} {g} {b}",)
            })
            .collect::<Vec<String>>()
            .chunks(5) // Line can be at most 70 chars, so we limit to 5 chunks here to be within limit
            .map(|chunk| format!("{}\n", chunk.join(" ")))
            .collect();

        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        let full_ppm_data = format!("{}{}", header, ppm_data);
        full_ppm_data
    }

    pub fn export_as_ppm_and_save_to_file(&self, ppm_file_path: &str) {
        let ppm = self.export_as_ppm();
        info!("Saving PPM export to file: {}", ppm_file_path);
        fs::write(ppm_file_path, ppm).expect("should write exported PPM to file");
    }

    pub fn export_to_desired_format_based_on_extension(&self, image_file_path: &str) {
        let expected_file_format = image_file_path.split('.').last().unwrap().to_uppercase();

        info!(
            "Exporting canvas as PPM and converting to desired image format: {}",
            expected_file_format
        );
        let ppm = self.export_as_ppm();

        debug!("Reading exported PPM data");
        let reader = ImageReader::new(Cursor::new(ppm))
            .with_guessed_format()
            .expect("should read PPM image");

        debug!("Decoding PPM image data");
        let img = reader.decode().expect("should decode PPM image");

        debug!(
            "The converted image format is derived from the file extension: {}",
            expected_file_format
        );
        info!(
            "Saving decoded {} image to file: {}",
            expected_file_format, image_file_path
        );
        img.save(image_file_path)
            .expect("should write exported image to file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn canvas() -> Canvas {
        Canvas::new_black_canvas(800, 600)
    }

    #[rstest]
    fn can_create_canvas(canvas: Canvas) {
        assert_eq!(canvas.width, 800);
        assert_eq!(canvas.height, 600);

        // Every pixel is initialized to black color
        for row in &canvas.pixels {
            for &color in row {
                assert_eq!(color, Vec3A::new_black());
            }
        }
    }

    #[rstest]
    fn can_write_pixel_to_canvas(mut canvas: Canvas) {
        let red = Vec3A::new_red();
        canvas.write_pixel(10, 5, red);

        let row = canvas.pixels.get(5).expect("should get rows");
        let pixel = row.get(10).expect("should get pixel in row");
        assert_eq!(pixel, &red);
    }

    #[rstest]
    fn can_write_and_get_pixel_at_coordinates(mut canvas: Canvas) {
        let blue = Vec3A::new_blue();
        canvas.write_pixel(10, 5, blue);
        assert_eq!(canvas.pixel_at(10, 5), &blue);
    }

    #[test]
    fn can_export_small_canvas_as_ppm() {
        let mut canvas = Canvas::new_black_canvas(5, 3);
        let c1 = Vec3A::new_color(1.5, 0.0, 0.0);
        let c2 = Vec3A::new_color(0.0, 0.5, 0.0);
        let c3 = Vec3A::new_color(-0.5, 0.0, 1.0);
        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);
        let ppm = canvas.export_as_ppm();
        assert_eq!(ppm, "P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n");
    }

    #[test]
    fn can_split_long_lines_when_exporting_as_ppm() {
        let color = Vec3A::new_color(1.0, 0.9, 0.8);
        let canvas = Canvas::new_with_initial_color(10, 2, color);
        let ppm = canvas.export_as_ppm();
        assert_eq!(ppm, "P3\n10 2\n255\n255 230 204 255 230 204 255 230 204 255 230 204 255 230 204\n255 230 204 255 230 204 255 230 204 255 230 204 255 230 204\n255 230 204 255 230 204 255 230 204 255 230 204 255 230 204\n255 230 204 255 230 204 255 230 204 255 230 204 255 230 204\n");
    }

    #[test]
    fn ppm_export_ends_with_a_newline() {
        let color = Vec3A::new_color(1.0, 0.9, 0.8);
        let canvas = Canvas::new_with_initial_color(20, 20, color);
        let ppm = canvas.export_as_ppm();
        let ppm_chars: Vec<char> = ppm.chars().collect();
        assert_eq!(
            ppm_chars
                .last()
                .expect("should get last element of ppm_chars"),
            &'\n'
        );
    }
}
