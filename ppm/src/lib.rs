use std::fmt;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::ptr;
use std::mem;


#[derive(Clone, Copy)]
pub struct Pixel{
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel {
            r: r,
            g: g,
            b: b,
        }
    }

    pub fn red(&self) -> u8 {
        self.r
    }

    pub fn green(&self) -> u8 {
        self.g
    }

    pub fn blue(&self) -> u8 {
        self.b
    }

    pub fn invert(&mut self) {
        self.r = 0xFF - self.r;
        self.g = 0xFF - self.g;
        self.b = 0xFF - self.b;
    }

    pub fn true_gray_scale(&mut self) {
        let gray = (self.r as f32 * 0.3 + self.g as f32 * 0.59 + self.b as f32 * 0.11) as u8;

        self.r = gray;
        self.g = gray;
        self.b = gray;
    }

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

pub struct Image {
    pixels: Vec<Pixel>,
    height: usize,
    width: usize,
}

use std::io::{self, prelude::*, BufReader};

impl Image {
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
                if i == 2
                {
                    let positions:Vec<&str> = l.split(" ").collect();
                    h = positions[1].parse::<usize>().unwrap();
                    w = positions[0].parse::<usize>().unwrap();
                    let size = h * w;
                    buffer = vec![Pixel::new(0, 0, 0); size as usize];
                }
                if i > 3
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

        for pixel in &buffer
        {
            println!("{}", pixel);
        }
        Image { pixels: buffer, height: h, width: w}

    }

}
    
// impl Image {
//     fn new_with_file(filename: &Path) -> Image {
//         let f = File::open("img.ppm");
//         let reader  = BufReader::new(f);

//         let mut w = 0;
//         let mut buffer = vec![0; 0 as usize];
//         for line in reader.lines() {
//             i += 1;


//         }
//         // Image { pixels: buffer, height: h, width: w}
//     }
// }
