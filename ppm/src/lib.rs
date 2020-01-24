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

    fn invert(&mut self) {
        self.r = 0xFF - self.r;
        self.g = 0xFF - self.g;
        self.b = 0xFF - self.b;
    }

    fn true_gray_scale(&mut self) {
        let gray = (self.r as f32 * 0.3 + self.g as f32 * 0.59 + self.b as f32 * 0.11) as u8;

        self.r = gray;
        self.g = gray;
        self.b = gray;
    }

    fn basic_gray_scale(&mut self){
        let gray = ((self.r as f32 + self.g as f32 + self.b as f32) / 3.0) as u8 ;
        
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
    
    #[test]
    fn test_display_pixel() {
        let mut p = Pixel{r:0, g:0, b:0};

        assert_eq!(format!("{}", p), "r = 0, g = 0, b = 0"); 
    }

    #[test]
    fn test_eq_pixel() {
        let mut p1 = Pixel{r:0, g:0, b:0};
        let mut p2 = Pixel{r:0, g:0, b:0};

        assert!(p1 == p2);
    }

    
    #[test]
    fn test_gray_true() {
        let mut p1 = Pixel{r:255, g:0, b:167};
        
        p1.true_gray_scale();
        assert_eq!(p1.red(), 94);
        assert_eq!(p1.green(), 94);
        assert_eq!(p1.blue(), 94);
    }

    #[test]
    fn test_gray_basic() {
        let mut p1 = Pixel{r:255, g:0, b:167};
        
        p1.basic_gray_scale();
        assert_eq!(p1.red(), 140);
        assert_eq!(p1.green(), 140);
        assert_eq!(p1.blue(), 140);
    }
}