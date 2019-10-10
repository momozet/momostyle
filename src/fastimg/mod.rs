//! Functions for filtering images.
use rayon::prelude::*;
//~ use std::sync::{Arc, Mutex};

use image::{GenericImage, GenericImageView, GrayImage, ImageBuffer, Luma, Pixel, Primitive};

extern crate image;
extern crate rayon;

use image::{Rgb, RgbImage};
//~ use rayon::prelude::*;
use super::hsl::HSL;
use super::yuv::YUV;

fn hsl_rank(clr: image::Rgb<u8>)-> image::Rgb<u8> {
    let rgb_in = [clr[0],clr[1],clr[2]];
    let hsl_in = HSL::from_rgb(&rgb_in);
    //~ let ll = match hsl_in.l {
            //~ l if 0.1<l && l<0.35 => 0.225,
            //~ l if 0.355<l && l<0.65 => 0.5,
            //~ l if 0.655<l && l<0.9 => 0.775,
            //~ l => l,
    //~ };
    let ll = match hsl_in.l {
            l if 0.1<l && l<0.35 => l*0.3+0.225-0.225*0.3,
            l if 0.36<l && l<0.65 => l*0.3+0.5-0.5*0.3,
            l if 0.66<l && l<0.9 => l*0.3+0.775-0.775*0.3,
            l => l,
    };
    //~ let hh = (hsl_in.h*0.05).round()*20.0;
    let hsl_out = HSL { h: hsl_in.h, s: hsl_in.s, l: ll };
    //~ let hsl_out = HSL { h: hh, s: hsl_in.s, l: ll };
    //~ let hsl_out = HSL { h: hsl_in.h, s: 0.5, l: ll };
    let (r, g, b) = hsl_out.to_rgb();
    Rgb([r,g,b])
}
pub fn yuv_rank(clr: image::Rgb<u8>)-> image::Rgb<u8> {
    //~ const bnd: f64 = 0.005;
    //~ const bnd: f64 = 0.003;
    const bnd: f64 = 0.002;
    //~ const bnd: f64 = 0.001;
    //~ let ub = [0.1*255.0, (0.35+0.005)*255.0, (0.65+0.005)*255.0];
    //~ let db = [0.35*255.0, 0.65*255.0, 0.9*255.0];
    //~ let cb = [0.225*255.0, 0.5*255.0, 0.775*255.0];
    /////////////////////
    //~ const ub: [f64; 3] = [0.05*255.0, (0.35+0.005)*255.0, (0.65+0.005)*255.0];
    //~ const db: [f64; 3] = [0.35*255.0, 0.65*255.0, 0.95*255.0];
    //~ const cb: [f64; 3] = [0.2*255.0, 0.5*255.0, 0.8*255.0];
    ///////////////////
    const ub: [f64; 3] = [0.1*255.0, (0.3+bnd)*255.0, (0.6+bnd)*255.0];
    const db: [f64; 3] = [(0.3-bnd)*255.0, (0.6-bnd)*255.0, 0.9*255.0];
    const cb: [f64; 3] = [0.2*255.0, 0.5*255.0, 0.8*255.0];
    //~ let atn = 0.5;
    //~ const atn: f64 = 0.5;
    const atn: [f64; 3] = [0.6, 0.6, 0.6];
    let rgb_in = [clr[0],clr[1],clr[2]];
    let yuv_in = YUV::from_rgb(&rgb_in);
    
    let sl = |l,a,c|{l*a+c*(1.0-a)};
    //~ let y255=yuv_in.y*0.00392;
    let yy = match yuv_in.y {
            //~ l if ub[0]<l && l<db[0] => l*atn+cb[0]-cb[0]*atn,
            //~ l if ub[1]<l && l<db[1] => l*atn+cb[1]-cb[1]*atn,
            //~ l if ub[2]<l && l<db[2] => l*atn+cb[2]-cb[2]*atn,
            ////
            l if ub[0]<l && l<db[0] => sl(l,atn[0],cb[0]),
            l if ub[1]<l && l<db[1] => sl(l,atn[1],cb[1]),
            l if ub[2]<l && l<db[2] => sl(l,atn[2],cb[2]),
            ////
            //~ l if ub[0]>l  => l*1.1,
            //~ l if db[0]<l && l<ub[1] => sl(l,1.3,db[0]+bnd*0.5),
            //~ l if db[1]<l && l<ub[2] => sl(l,1.3,db[1]+bnd*0.5),
            //~ l if db[2]>l  => l*1.1-0.1,
            l => l,
    };

    let hsl_out = YUV { y: yy, u: yuv_in.u, v:yuv_in.v  };

    let (r, g, b) = hsl_out.to_rgb();
    Rgb([r,g,b])
}

//    = note: expected type `image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>`
//    = note: expected type `fn(image::Rgb<u8>) -> image::Rgb<u8> {hsl_rank}`
pub fn para_map(image: &image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>)->image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>{
    //~ let imgvec = *image.into_raw
    let (width, height) = (image.width() as usize, image.height() as usize);
    
    let vimg: Vec<_> =
    image.enumerate_pixels()
    //~ out.pixels_mut()
        .collect::<Vec<(u32, u32, &Rgb<u8>)>>()
        //~ .collect::<Vec<&mut Rgb<u8>>>()
        .par_iter_mut()
        .map(|(x, y, pixel)| {
            let prgb = Rgb([pixel[0], pixel[1], pixel[2]]);
            let post=yuv_rank(prgb);
            
            post
        }).collect();
    let mut vr = vec![0; width*height*3];
    for (i,v) in vimg.iter().enumerate(){
        vr[i*3]=v[0];
        vr[i*3+1]=v[1];
        vr[i*3+2]=v[2];
    }
    let ret: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> = image::ImageBuffer::from_raw(width as u32, height as u32, vr).unwrap();
    
    ret
}

pub fn para_map2(image1: &image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,image2: &image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>)->image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>{
    //~ let imgvec = *image.into_raw
    let (width, height) = (image1.width() as usize, image1.height() as usize);
    //~ let mut out = *image.clone();
    let mut out: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> = image::ImageBuffer::new(width as u32,height as u32);
    out= image1.clone();
    
    out.pixels_mut()
        .collect::<Vec<&mut Rgb<u8>>>()
        .par_iter_mut().zip(
            image2.pixels().collect::<Vec<&Luma<u8>>>()
            .par_iter()
        )
        .for_each(|(p1, p2)| {
          //~ let r = 255u8-p[0];
            let hh = |a|{if a < 0{0_u8}else{a as u8}};
            //~ let r = p[0] as i32;
            //~ let r = hh(p[0] as i32 - 0) as i32;
            let r = (p2[0] >>1) as i32;
            //~ Rgb([p[0]&q[0],p[0]&q[1],p[0]&q[2]])//line and color
            //~ Rgb([r&q[0],r&q[1],r&q[2]])//line and color
            //~ Rgba([p[0]&q[0],p[0]&q[1],p[0]&q[2],255])
            //~ *p=Rgb([hh(q[0] as i32 - r),hh(q[1] as i32 - r),hh(q[2] as i32 - r)]);
            p1[0]=hh(p1[0] as i32 - r);
            p1[1]=hh(p1[1] as i32 - r);
            p1[2]=hh(p1[2] as i32 - r);
        });
    
    out

}
pub fn paraf(){
    let z=1;
    //~ let iii=[1,20,3].par_iter().map(|i|{i+z}).max();
    let iii: Vec<_>=(1..10).into_par_iter().map(|i|{i+z}).collect();
    println!("{:?}",iii);
}
