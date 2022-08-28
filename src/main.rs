#![allow(non_snake_case)]
extern crate raster;
fn main(){
    let image = raster::open("test.png").unwrap();
    let mut resim = raster::Image::blank(image.width/5, image.height/5);
    for w in 0..image.width/5{
        for h in 0..image.height/5{
            let mut avg = 0;
            for xbox in 0..5{
                for ybox in 0..5{
                    let color = image.get_pixel(w*5+xbox, h*5+ybox).unwrap();
                    avg += (color.r/3 + color.b/3 + color.g/3)/25
                }
            }
            let color = raster::Color {r : avg,b : avg, g : avg,a : 255};
            resim.set_pixel(w, h, color).unwrap();
        }
    }
    raster::save(&resim, "out.png").unwrap();
}