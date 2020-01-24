use std::fmt;

#[derive(Clone, Copy)]
struct Pixel{
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel {
            r: r,
            g: g,
            b: b,
        }
    }

    fn red(&self) -> u8 {
        self.r
    }

    fn green(&self) -> u8 {
        self.g
    }

    fn blue(&self) -> u8 {
        self.b
    }

    fn invert(&mut self)
    {
        self.r = 0xFF - self.r;
        self.g = 0xFF - self.g;
        self.b = 0xFF - self.b;
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "r = {}, g = {}, b = {}", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::Pixel;

    #[test]
    fn test_create_pixel() {
        let p = Pixel{r:0, g:0, b:0};

        assert_eq!(p.red(), 0);
        assert_eq!(p.green(), 0);
        assert_eq!(p.blue(), 0);
    }

    #[test]
    fn test_invert_pixel() {
        let mut p = Pixel{r:0, g:0, b:0};

        p.invert();
        assert_eq!(p.red(), 255);
        assert_eq!(p.green(), 255);
        assert_eq!(p.blue(), 255);
    
    }


}