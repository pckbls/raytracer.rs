use color::Color;

use std::io::{ BufReader, BufWriter };
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
    pub fn try_load_from_ppm(path: String) -> Result<Self, String> {
        let f = File::open(path).map_err(|e| e.to_string())?;
        let f = BufReader::new(f);

        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut pixmap: Pixmap = Pixmap::new(0, 0); // TODO: use Box instead?
        let mut current_y: u32 = 0;

        enum FSM { MagicHeader, Dimensions, Foo, Contents, Accepted };
        let mut current_state = FSM::MagicHeader;

        for line in f.lines() {
            let line = line.unwrap();

            match current_state {
                FSM::MagicHeader => {
                    if line != "P3" {
                        return Err("Header does not equal 'P3'".to_string());
                    }
                    current_state = FSM::Dimensions;
                }
                FSM::Dimensions => {
                    let splits: Vec<&str> = line.split_whitespace().collect();

                    if splits.len() != 2 {
                        return Err("Dimensions should consist of width and height".to_string())
                    }

                    //  TODO: do we have to use map_err ?
                    width = splits[0].parse::<u32>().map_err(|_| "Cannot parse width.".to_string())?;
                    height = splits[1].parse::<u32>().map_err(|_| "Cannot parse height.".to_string())?;

                    pixmap = Pixmap::new(width, height);

                    current_state = FSM::Foo;
                }
                FSM::Foo => {
                    // TODO: implement
                    current_state = FSM::Contents;
                }
                FSM::Contents => {
                    let splits: Vec<&str> = line.split_whitespace().collect();

                    if splits.len() != (width * 3) as usize {
                        return Err("TODO".to_string()); // TODO
                    }

                    // TODO: consider using .map() instead?
                    for x in 0..(width as usize) {
                        let r = splits[3*x+0].parse::<u8>().map_err(|_| "Cannot parse r color".to_string())?;
                        let g = splits[3*x+1].parse::<u8>().map_err(|_| "Cannot parse g color".to_string())?;
                        let b = splits[3*x+2].parse::<u8>().map_err(|_| "Cannot parse b color".to_string())?;

                        // TODO: Remember that we've inverted the y-axis, right?
                        pixmap.draw(x as u32, height-current_y-1, Color { r: r, g: g, b: b });
                    }

                    current_y += 1;
                    if current_y == height {
                        current_state = FSM::Accepted
                    }
                }
                FSM::Accepted => {
                    return Err("TODO_height".to_string());
                }
            }
        }

        Ok(pixmap)
    }

    /// Saves the pixmap's contents as a PPM file
    #[allow(dead_code)]
    pub fn save_as_ppm(&self, path: String) -> Result<(), io::Error> {
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
fn test_save_load_ppm() {
    let mut pixmap = Pixmap::new(4, 4);
    pixmap.pixels[0].color.r = 5;
    pixmap.save_as_ppm("./testdata/output/pixmap_save_test.ppm".to_string()).unwrap();
    let loaded_pixmap = Pixmap::try_load_from_ppm("./testdata/output/pixmap_save_test.ppm".to_string()).unwrap();
    assert_eq!(pixmap, loaded_pixmap);
}

