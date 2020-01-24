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
}