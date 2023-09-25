use std::fs::File;
use std::io::*;
#[path = "../../conversion.rs"]
mod conversion;
use crate::conversion::split_u32;
const CANNOT_WRITE: &str = "ERROR: Could not write to output file.";
const CANNOT_CREATE: &str = "ERROR: Could not create to output file.";
const CANNOT_OPEN: &str = "ERROR: Could not open specified image.";
const EXPECTED_PIXEL: &str = "ERROR: Expected a pixel while parsing image.";
// use image::image

fn main() {
    let mut output = File::create("image.mrgb").expect(CANNOT_CREATE);
    let input = image::open("test.png").expect(CANNOT_OPEN);
    let w = input.width();
    let width_split = split_u32(w, 7);
    write_size(width_split.0, width_split.1, &mut output );
    let pixels = input.to_rgb8();
    let pixel_count = pixels.len() / 3;
    let mut pixels = pixels.pixels().into_iter();

    // let mut previous_color = 0;
    for _ in 0..(pixel_count as f32 / 2.).floor() as usize {
        let pixel1 = pixels.next().expect(EXPECTED_PIXEL).0;
        let pixel2 = pixels.next().expect(EXPECTED_PIXEL).0;
        let b1 = ((pixel1[0] / 17) << 4) + pixel1[1] / 17;
        let b2 = ((pixel1[2] / 17) << 4) + pixel2[0] / 17;
        let b3 = ((pixel2[1] / 17) << 4) + pixel2[2] / 17;

        output.write(&[b1 as u8, b2 as u8, b3 as u8]).expect(CANNOT_WRITE);
    }
}
#[inline]
fn write_size(w1: u32, w2: u32, output: &mut File) {
    println!("{w1} {w2}");
    output.write(&[w1 as u8,w2 as u8]).expect(CANNOT_WRITE);
}
