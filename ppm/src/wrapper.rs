mod lib;

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

// invert_Pixel for lib, inplace
#[no_mangle]
pub extern "C" fn pixel_to_gray(p_ptr: *mut lib::Pixel) -> lib::Pixel{
    let p = unsafe {
        assert!(!p_ptr.is_null());
        &mut *p_ptr
    };
    p.basic_gray_scale();
    lib::Pixel::new(p.red(), p.green(), p.blue())
}