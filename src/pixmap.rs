use color::Color;

use std::io::BufWriter;
use std::io;
use std::io::prelude::*;
use std::fs::File;

#[allow(dead_code)]
#[derive(Clone,PartialEq,Debug)]
pub struct Pixel {
    color: Color
}

#[allow(dead_code)]
#[derive(Clone,PartialEq,Debug)]
pub struct Pixmap {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Pixel>
}

impl Pixmap {
    /// Creates a new pixmap with specific dimensions.
    #[allow(dead_code)]
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let mut pixels = Vec::with_capacity(size);
        for _ in 0..size {
            let pixel = Pixel {
                color: Color { r: 0, g: 0, b: 0 }
            };
            pixels.push(pixel);
        }

        Pixmap {
            width: width,
            height: height,
            pixels: pixels
        }
    }

    /// Maps screen coordinates onto an index than can be used to access
    /// the pixel value in the internal pixels vector.
    #[allow(dead_code)]
    fn coords_to_index(&self, x: u32, y: u32) -> u32 {
        if x > self.width - 1 || y > self.height - 1 {
            panic!("Coordinates are not within the pixmaps's dimensions.");
        }
        x * self.height + y
    }

    /// Loads a pixmap from a PPM file
    #[allow(dead_code)]
    pub fn try_load_from_ppm(path: String) -> Result<Self, io::Error> {
        panic!("Not implemented yet.");
    }

    /// Saves the pixmap's contents as a PPM file
    #[allow(dead_code)]
    pub fn save_as_ppm(self, path: String) -> Result<(), io::Error> {
        let f = File::create(path)?;
        let mut f = BufWriter::new(f);

        // Generate PPM header.
        f.write("P3\n".as_bytes())?;
        f.write_fmt(format_args!("{} {}\n", self.width, self.height))?;
        f.write("255\n".as_bytes())?;

        // Write pixmap contents.
        // TODO: y iteration direction has been inversed to flip image upside-down
        // but I'm unsure if this was the right way to fix the issue.
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let index = self.coords_to_index(x, y) as usize;
                let ref pixel = &self.pixels[index];
                f.write_fmt(format_args!("{} {} {} ", pixel.color.r, pixel.color.g, pixel.color.b))?;
            }

            f.write("\n".as_bytes())?;
        }

        Ok(())
    }

    /// Colorizes a single pixel on the pixmap
    #[allow(dead_code)]
    pub fn draw(&mut self, x: u32, y: u32, color: Color) {
        let index = self.coords_to_index(x, y) as usize;
        self.pixels[index].color = color;
    }

    /// Get color of a single pixel
    /// TODO: Return reference with life time instead of copy?
    #[allow(dead_code)]
    pub fn get_color(&self, x: u32, y: u32) -> Color {
        let index = self.coords_to_index(x, y) as usize;
        return self.pixels[index].color.clone();
    }
}

#[test]
fn test_good_coords_to_index() {
    let pixmap = Pixmap::new(4, 2);
    assert!(pixmap.coords_to_index(0, 0) == 0);
    assert!(pixmap.coords_to_index(0, 1) == 1);
    assert!(pixmap.coords_to_index(1, 0) == 2);
    assert!(pixmap.coords_to_index(3, 1) == 7);
}

#[test]
#[should_panic]
fn test_bad_coords_to_index() {
    let pixmap = Pixmap::new(4, 4);
    pixmap.coords_to_index(0, 6);
}

#[test]
fn test_draw() {
    let mut pixmap = Pixmap::new(4, 4);
    let color = Color { r: 255, g: 0, b: 0 };
    pixmap.draw(1, 1, color.clone());
    assert!(pixmap.get_color(1, 1) == color);
}

#[test]
#[ignore]
fn test_save_load_ppm() {
    let mut pixmap = Pixmap::new(4, 4);
    pixmap.pixels[0].color.r = 5;
    let result = pixmap.save_as_ppm("./testdata/output/pixmap_save_test.ppm".to_string());
    assert!(result.is_ok());
    Pixmap::try_load_from_ppm("./testdata/output/pixmap_save_test.ppm".to_string()).unwrap();
}

