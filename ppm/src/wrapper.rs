mod lib;
use std::path::Path;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::str;

// Makes average between 2 Pixels
#[no_mangle]
pub extern "C" fn pixel_avg(p1: lib::Pixel, p2: lib::Pixel) -> lib::Pixel {
    lib::Pixel::new((p1.red() + p2.red()) / 2,
                (p1.green() + p2.green()) / 2,
                (p1.blue() + p2.blue()) / 2)
}

// invert_Pixel for lib, inplace
#[no_mangle]
pub extern "C" fn pixel_inv(p_ptr: *mut lib::Pixel) -> lib::Pixel{
    let p = unsafe {
        assert!(!p_ptr.is_null());
        &mut *p_ptr
    };
    p.invert();
    lib::Pixel::new(p.red(), p.green(), p.blue())
}

// invert_Pixel for lib
#[no_mangle]
pub extern "C" fn pixel_to_gray(p_ptr: *mut lib::Pixel) -> lib::Pixel{
    let p = unsafe {
        assert!(!p_ptr.is_null());
        &mut *p_ptr
    };
    p.basic_gray_scale();
    lib::Pixel::new(p.red(), p.green(), p.blue())
}


// open_image for lib
#[no_mangle]
pub extern "C" fn open_image(path_c: *const c_char) -> lib::Image{
    
    //println!("{}", path_c);
    let c_str = unsafe { CStr::from_ptr(path_c) };
    let str_slice: &str = c_str.to_str().unwrap();
    println!("{}", str_slice);
    let path = Path::new(&str_slice);
    let img = lib::Image::new_with_file(path);
    return img;
}

// Display the image for lib
#[no_mangle]
pub extern "C" fn display_image_in_terminal(img: lib::Image){
    img.display_image_in_terminal();
}
