use std::f32;

//~ use num_traits::{NumCast, ToPrimitive, Zero};

use image::{GenericImage, GenericImageView, GrayImage, ImageBuffer, Luma, Pixel, Primitive};
//~ use buffer::{ImageBuffer, Pixel};
//~ use image::GenericImageView;
//~ use math::utils::clamp;
//~ use traits::{Enlargeable, Primitive};
use std::cmp::{max, min};

use rayon::prelude::*;

/// Perform a 3x3 box filter on the supplied image.
/// ```kernel``` is an array of the filter weights of length 9.
//~ let img4: image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>> = image::imageops::filter3x3(&gr,&lapl);//gr crate image
pub fn flt3x3f(image: &image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>, kernel: &[f64]) -> image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>
{
    //~ const taps: &[(isize, isize)] = &[
        //~ (-1, -1),
        //~ (0, -1),
        //~ (1, -1),
        //~ (-1, 0),
        //~ (0, 0),
        //~ (1, 0),
        //~ (-1, 1),
        //~ (0, 1),
        //~ (1, 1),
    //~ ];
    let taps: &[(f64, isize, isize)] = &[
        (kernel[0], -1, -1),
        (kernel[1], 0, -1),
        (kernel[2], 1, -1),
        (kernel[3], -1, 0),
        (kernel[4], 0, 0),
        (kernel[5], 1, 0),
        (kernel[6], -1, 1),
        (kernel[7], 0, 1),
        (kernel[8], 1, 1),
    ];

    let (width, height) = image.dimensions();
    let image_padded = padding(&image);
    //~ let mut out = ImageBuffer::new(width, height);

    //~ let max = 255.0;
    //~ let max: f32 = NumCast::from(max).unwrap();

    //~ let sum = match kernel.iter().fold(0.0, |s, &item| s + item) {
        //~ x if x == 0.0 => 1.0,
        //~ sum => sum,
    //~ };
    //~ let sum = 1.0;
    //~ let sum = (sum, sum, sum, sum);
    let img_vec = image_padded.into_raw();

    let flt: Vec<u8> =
    //~ (0..(height*width)).into_par_iter().map(|i|{
    image.enumerate_pixels()
        .collect::<Vec<(u32, u32, &Luma<u8>)>>()
        .par_iter()
        .map(|(x, y, _)| {
            let mut t = 0.0;
            //~ let ob=|n,lim|{
                    //~ if n < 1 {
                        //~ 1
                    //~ } else if n > lim {
                        //~ lim
                    //~ } else {
                        //~ n
                    //~ }
                //~ };
            //~ let x = ob(i%width,width-2);
            //~ let y = ob(i/width,height-2);
            //~ let x = i%width+1;
            //~ let y = i/width+1;
            //~ let x = 10;
            //~ let y = 10;
            //~ for k_y in 0..3 {
                //~ let y0 = *y as isize + (k_y-1) +1;
                //~ for k_x in 0..3 {
                    //~ let x0 = *x as isize + (k_x-1) +1;
            for &(k, a, b) in taps.iter() {
            //~ for (&k, &(a, b)) in kernel.iter().zip(taps.iter()) {
                let x0 = *x as isize + a + 1;
                let y0 = *y as isize + b + 1;
                
                //~ let p = image_padded.get_pixel(x0 as u32, y0 as u32);
                //~ let (v,_,_,_) = p.channels4();
                //~ let v = p.channels()[0];
                //~ let k=kernel[(k_x+k_y*3) as usize];
                let index = (x0+((width+2) as isize)*y0) as usize;
                let v = img_vec[index] as f64;

                //~ let vf = v as f64;

                t += v * k;
                //~ }
            }
            clampf(t) as u8
        }).collect();
        //~ println!("{}",flt.len());
    let ret: image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>> = image::ImageBuffer::from_raw(width, height, flt).unwrap();
    
    ret
}
pub fn clampf(input: f64) -> f64{
    if input < 0.0 {
        0.0
    } else if input > 255.0 {
        255.0
    } else {
        input
    }
}
pub fn padding(image: &image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>) -> image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>
{
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width+2, height+2);
    
    for (x, y, p) in image.enumerate_pixels() {
        out.put_pixel(x+1, y+1, Luma([p.channels()[0]]));
    }
    out
}
pub fn flt3x3(image: &image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>, kernel: &[i32]) -> image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>
{
    let (width, height) = image.dimensions();
    let image_padded = padding1d(&image);
    //~ let sum = match kernel.iter().fold(0.0, |s, &item| s + item) {
        //~ x if x == 0.0 => 1.0,
        //~ sum => sum,
    //~ };
    //~ let sum = 1.0;
    //~ let sum = (sum, sum, sum, sum);
    //~ let img_vec = image_padded.into_raw();
    //~ let img_vec = image.into_raw();

    let flt: Vec<u8> =
    (0..width*height).into_par_iter()
        .map(|i| {
            let mut t = 0;
            for k_y in 0..3 {
                //~ let y0 = y as isize + k_y;
                //~ let dy = (k_y*width) as usize;
                let dy = (k_y*width);
                for k_x in 0..3 {
                    //~ let x0 = x as isize + k_x;
                    //~ let dx = k_x as usize;
                    let dx = k_x;
                    
                    let k=kernel[(k_x+k_y*3) as usize];
                    //~ let index = (x0+((width+2) as isize)*y0) as usize;
                    //~ let v = img_vec[index] as i32;
                    //~ let p = unsafe { image_padded.unsafe_get_pixel(x0 as u32, y0 as u32) };
                    //~ let v = p.channels()[0] as i32;
                    
                    //~ let v = image_padded[i as usize +dx+dy] as i32;
                    let v = image_padded[(i+dx+dy) as usize] as i32;
                    
                    t += v * k;
                }
            }
            clamp(t) as u8
        }).collect();
        //~ println!("{}",flt.len());
    let ret: image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>> = image::ImageBuffer::from_raw(width, height, flt).unwrap();
    
    ret
}
pub fn padding1d(image: &image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>) -> std::vec::Vec<u8>
{
    let (width, height) = image.dimensions();
    //~ let mut out = ImageBuffer::new(width, height+2);
    let mut out = vec![0; (width*(height+2)+2) as usize];
    
    for (x, y, p) in image.enumerate_pixels() {
        //~ out.put_pixel(x, y+1, Luma([p.channels()[0]]));
        out[(x+(y+1)*width+1) as usize] = p.channels()[0];
    }
    out
}
pub fn clamp(input: i32) -> i32{
    if input < 0 {
        0
    } else if input > 255 {
        255
    } else {
        input
    }
}
