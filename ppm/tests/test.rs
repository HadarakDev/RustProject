// use ppm;

extern crate ppm;

#[test]
fn test_create_pixel() {
    let p = ppm::Pixel::new(0, 0, 0);

    assert_eq!(p.red(), 0);
    assert_eq!(p.green(), 0);
    assert_eq!(p.blue(), 0);
}

#[test]
fn test_invert_pixel() {
    let mut p = ppm::Pixel::new(0, 0, 0);

    p.invert();
    assert_eq!(p.red(), 255);
    assert_eq!(p.green(), 255);
    assert_eq!(p.blue(), 255);  
}

#[test]
fn test_display_pixel() {
    let p = ppm::Pixel::new(0, 0, 0);

    assert_eq!(format!("{}", p), "r = 0, g = 0, b = 0"); 
}

#[test]
fn test_eq_pixel() {
    let p1 = ppm::Pixel::new(0, 0, 0);
    let p2 = ppm::Pixel::new(0, 0, 0);

    assert!(p1 == p2);
}


#[test]
fn test_gray_true() {
    let mut p1 = ppm::Pixel::new(255, 0, 167);
    
    p1.true_gray_scale();
    assert_eq!(p1.red(), 94);
    assert_eq!(p1.green(), 94);
    assert_eq!(p1.blue(), 94);
}

#[test]
fn test_gray_basic() {
    let mut p1 = ppm::Pixel::new(255, 0, 167);
    
    p1.basic_gray_scale();
    assert_eq!(p1.red(), 140);
    assert_eq!(p1.green(), 140);
    assert_eq!(p1.blue(), 140);
}
