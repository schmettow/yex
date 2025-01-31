use std::time::Instant;
use image::{GenericImageView, ImageBuffer, SubImage};
use imageproc::integral_image::ArrayData;
use nokhwa::{pixel_format::{LumaFormat, RgbFormat}, utils::{CameraIndex, RequestedFormat, RequestedFormatType}, Camera};

type WCImage = ImageBuffer<image::Rgb<u8>, Vec<u8>>;

fn mean_brightness(sub_image: &SubImage<&WCImage>) -> u32 {
    let mut sum_brightness = 0;
    let mut pixel_count = 0;
    for (_, _, pixel) in sub_image.to_image().enumerate_pixels() {
        let pixel_data = pixel.data();
        sum_brightness += (pixel_data[0] as u32 + pixel_data[1] as u32 + pixel_data[2] as u32) / 3;
        pixel_count += 1;
    }
    sum_brightness / pixel_count
}

fn quadbright_1(image: &WCImage) -> Vec<u32> {
    let mut QB = vec![0; 4];
    for (x, y, pixel) in image.enumerate_pixels() {
        match (x < image.width()/2, y < image.height()/2){
            (true, true) => QB[0] =  QB[0] + pixel.data()[0] as u32,
            (false, true) => QB[1] =  QB[1] + pixel.data()[0] as u32,
            (false, false) => QB[2] =  QB[2] + pixel.data()[0] as u32,
            (true, false) => QB[3] =  QB[3] + pixel.data()[0] as u32,
        }
    }
    return QB;
}

fn quadbright_2(image: &WCImage) -> Vec<u32> {
    let height = image.height();
    let width = image.width();
    let ne = image.view(0, 0, width / 2, height / 2);
    let se = image.view(width / 2, 0, width / 2, height / 2);
    let sw = image.view(width / 2, height / 2, width / 2, height / 2);
    let nw = image.view(0, height / 2, width / 2, height / 2);

    vec![ne, se, sw, nw].iter().map(|quad| mean_brightness(quad)).collect()
}



fn main(){
    // first camera in system
    let index = CameraIndex::Index(0);
    // request the absolute highest resolution CameraFormat that can be decoded to RGB.
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
    // make the camera
    let mut camera = Camera::new(index, requested).unwrap();
    println!("{}", camera.info().human_name());
    // get one frame
    let start = std::time::Instant::now();
    loop{
        let frame = camera.frame().unwrap();
        let image: WCImage = frame.decode_image::<RgbFormat>().unwrap();
        let qb = quadbright_2(&image);
        println!("{},{},{},{},{}", start.elapsed().as_millis(), qb[0], qb[1], qb[2], qb[3]);
    }
}