#![allow(non_snake_case)]
extern crate raster;
fn main(){
    let image = raster::open("test.png").unwrap();
    let mut resim = raster::Image::blank(image.width, image.height);
    for w in 0..image.width{
        for h in 0..image.height{
            let pixel = image.get_pixel(w,h).unwrap();
            let avg = pixel.r/3 + pixel.b/3 + pixel.g/3;
            let color = raster::Color {r:avg,g:avg,b:avg,a:255};
            resim.set_pixel(w, h, color).unwrap();
        }
    }
    raster::save(&resim, "out.png").unwrap();
}