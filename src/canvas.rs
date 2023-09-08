use crate::color::Color;

#[derive(Debug, PartialEq)]
pub struct Canvas {
    width: u16,
    height: u16,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![Color::new_black(); width as usize]; height as usize],
        }
    }

    pub fn write_pixel(&mut self, x: u16, y: u16, color: Color) {
        if let Some(row) = self.pixels.get_mut(y as usize) {
            if let Some(pixel) = row.get_mut(x as usize) {
                *pixel = color;
            } else {
                panic!("Column index is out of bounds when writing pixel to canvas");
            }
        } else {
            panic!("Row index is out of bounds when writing pixel to canvas");
        }
    }

    pub fn pixel_at(&mut self, x: u16, y: u16) -> &Color {
        if let Some(row) = self.pixels.get(y as usize) {
            if let Some(pixel) = row.get(x as usize) {
                pixel
            } else {
                panic!("Column index is out of bounds when getting pixel from canvas");
            }
        } else {
            panic!("Row index is out of bounds when getting pixel from canvas");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn canvas() -> Canvas {
        Canvas::new(20, 10)
    }

    #[rstest]
    fn can_create_canvas(canvas: Canvas) {
        assert_eq!(canvas.width, 20);
        assert_eq!(canvas.height, 10);

        // Every pixel is initialized to black color
        for row in &canvas.pixels {
            for &color in row {
                assert_eq!(color, Color::new_color_clamped(0.0, 0.0, 0.0));
            }
        }
    }

    #[rstest]
    fn can_write_pixel_to_canvas(mut canvas: Canvas) {
        let red = Color::new_color_clamped(1.0, 0.0, 0.0);
        canvas.write_pixel(10, 5, red);

        let row = canvas.pixels.get(5).expect("should get rows");
        let pixel = row.get(10).expect("should get pixel in row");
        assert_eq!(pixel, &red);
    }

    #[rstest]
    fn can_write_and_get_pixel_at_coordinates(mut canvas: Canvas) {
        let blue = Color::new_color_clamped(0.0, 0.0, 1.0);
        canvas.write_pixel(10, 5, blue);
        assert_eq!(canvas.pixel_at(10, 5), &blue);
    }
}
