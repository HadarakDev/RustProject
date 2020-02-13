#![crate_type = "dylib"]
use std::fmt;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use std::io::BufWriter;
use std::io::BufReader;


/// Representation of a Pixel: RGB
#[derive(Clone, Copy)]
pub struct Pixel{
    r: u8,
    g: u8,
    b: u8,
}

#[no_mangle]
pub extern fn create_pixel(r: u8, g: u8, b: u8) -> Pixel {
    Pixel::new(r, g, b)
}

#[no_mangle]
pub extern fn invert_pixel(p: &mut Pixel) {
    p.invert()
}

impl Pixel {
    /// Create a new Pixel
    #[no_mangle]
    pub extern fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel {
            r: r,
            g: g,
            b: b,
        }
    }

    /// Get red Value of Pixel 
    pub fn red(&self) -> u8 {
        self.r
    }

    /// Get green Value of Pixel 
    pub fn green(&self) -> u8 {
        self.g
    }

    /// Get blue Value of Pixel 
    pub fn blue(&self) -> u8 {
        self.b
    }
    /// Invert Pixel color
    pub fn invert(&mut self) {
        self.r = 0xFF - self.r;
        self.g = 0xFF - self.g;
        self.b = 0xFF - self.b;
    }

    /// Convert a pixel into Gray Scale using an improved method (30 % of red, 59% of blue, 11% of green)
    pub fn true_gray_scale(&mut self) {
        let gray = (self.r as f32 * 0.3 + self.g as f32 * 0.59 + self.b as f32 * 0.11) as u8;

        self.r = gray;
        self.g = gray;
        self.b = gray;
    }

    /// Convert a pixel into Gray Scale using basic ratio
    pub fn basic_gray_scale(&mut self){
        let gray = self.r / 3 + self.g / 3 + self.b / 3;
        
        self.r = gray;
        self.g = gray;
        self.b = gray;
    }


}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "r = {}, g = {}, b = {}", self.r, self.g, self.b)
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.red() && self.g == other.green() && self.b == other.blue()
    }
}

/// Representation of a Image
pub struct Image {
    pixels: Vec<Pixel>,
    height: usize,
    width: usize,
}



impl Image {

    /// Get height of Image
    pub fn height(&self) -> usize {
        self.height
    }
    /// Get width of Image
    pub fn width(&self) -> usize {
        self.width
    }
    /// Load Image with ppm file
    pub fn new_with_file(filename: &Path) -> Image {
        let f = File::open(filename).expect("Unable to open");
        let reader = BufReader::new(f);
        let mut i = 0;
        let mut j = 0;
        let mut w = 0;
        let mut h = 0;
        let mut buffer = vec![Pixel::new(0, 0, 0); 0 as usize];
        for line in reader.lines() {
            if let Ok(l) = line {
                if i == 1
                {
                    let positions:Vec<&str> = l.split_whitespace().collect();
                    h = positions[1].parse::<usize>().unwrap();
                    w = positions[0].parse::<usize>().unwrap();
                    let size = h * w;
                    buffer = vec![Pixel::new(0, 0, 0); size as usize];
                }
                if i > 2
                {
                    let pixels:Vec<&str> = l.split_whitespace().collect();
                    
                    for x in (0..pixels.len()).step_by(3) {
                        let r = pixels[x].parse::<u8>().unwrap();
                        let g = pixels[x + 1].parse::<u8>().unwrap();
                        let b = pixels[x + 2].parse::<u8>().unwrap();
                        mem::replace(&mut buffer[j], Pixel::new(r, g, b));
                        j += 1;
                    }
                }
                i += 1;
            }
        }

        Image { pixels: buffer, height: h, width: w}


    }

    /// Save Image to  ppm format with given filename
    pub fn save_to_ppm(&self, filename: &Path){
        let file = File::create(filename).unwrap();
        let mut writer = BufWriter::new(&file);

        write!(&mut writer, "P3\n").expect("Error");
        write!(&mut writer, "{} {}\n", self.width(), self.height()).expect("Error");
        write!(&mut writer, "255\n").expect("Error");
        let mut i = 0;
        for y in 0..self.height() - 1
        {
            for x in 0..self.width() - 1
            {
                if x != self.width() - 1
                {
                    write!(&mut writer, "{} {} {} ", self.pixels[i].red(), self.pixels[i].green(), self.pixels[i].blue()).expect("Error"); 
                }
                else
                {
                    write!(&mut writer, "{} {} {}", self.pixels[i].red(), self.pixels[i].green(), self.pixels[i].blue()).expect("Error");
                }
                i += 1;
            }
            if y != self.height() - 1
            {
                write!(&mut writer, "\n").expect("Error");
            }
        }
    }
    
    /// Convert Image into Gray Scale in Memory ( 9 to use basic method, 1 to use improved one)
    pub fn convert_image_to_gray(&mut self, basic_gray: u8){
        let size = self.height() * self.width();
        for i in 0..size - 1
        {
            if basic_gray == 1
            {
                Pixel::basic_gray_scale(&mut self.pixels[i]);
            }
            else
            {
                Pixel::true_gray_scale(&mut self.pixels[i]);
            }
        }
    }

    /// Invert Image Color in Memory
    pub fn invert(&mut self){
        let size = self.height() * self.width();
        for i in 0..size - 1
        {
            Pixel::invert(&mut self.pixels[i]);
        }
    }


}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut comma_separated = String::new();

        for num in &self.pixels {
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str(" ");
        }

        write!(f, "{}", comma_separated)
    }
}


