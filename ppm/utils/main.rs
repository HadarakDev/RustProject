extern crate ppm;
use std::path::Path;

fn main() {
    let path = Path::new("../../img.ppm");
    let path2 = Path::new("img2.ppm");
    let _img = ppm::Image::new_with_file(path);
    _img.save_to_ppm(path2);
    println!(" okokoko")
}