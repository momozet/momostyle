extern crate image;
extern crate imageproc;
//~ extern crate x11_screenshot;
extern crate scrap;

use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
//~ use std::fs::File;
use std::time::{Duration, Instant};
use std::thread;
//~ use std::time::Duration;

//~ use std::thread;
use std::sync::mpsc;
//~ use rayon::prelude::*;

use minifb::{Key, WindowOptions};//, Window};

//~ const WIDTH: usize = 640;
//~ const HEIGHT: usize = 480;
//~ const WIDTH: usize = 800;
//~ const HEIGHT: usize = 600;
//~ const WIDTH: usize = 900;
//~ const HEIGHT: usize = 600;
//~ const WIDTH: usize = 512;
//~ const HEIGHT: usize = 400;
const WIDTH: usize = 1080;
const HEIGHT: usize = 720;
//~ const CAP_WIDTH: u32 = 800;
//~ const CAP_HEIGHT: u32 = 600;
//~ const CAP_WIDTH: u32 = 1080;
//~ const CAP_HEIGHT: u32 = 720;
//~ const CAP_WIDTH: u32 = 100;
//~ const CAP_HEIGHT: u32 = 100;
//~ const WIDTH: usize = 100;
//~ const HEIGHT: usize = 100;
const CAP_WIDTH: u32 = WIDTH as u32;
const CAP_HEIGHT: u32 = HEIGHT as u32;


use image::ImageBuffer;
use image::{Luma, Rgb, Rgba};
use image::imageops::colorops::grayscale;
use image::imageops::resize;
use imageproc::contrast;
use imageproc::map::map_colors;
use imageproc::map::map_colors2;//colors2 needs input of 2images
use imageproc::filter::filter3x3;
use imageproc::filter::separable_filter;
use imageproc::filter::median_filter;
//~ use imageproc::filter::gaussian_blur_f32;
use imageproc::morphology;
use imageproc::distance_transform::Norm;

use hsl::HSL;

fn hsl_rank(clr: image::Rgb<u8>)-> image::Rgb<u8> {
    let rgb_in = [clr[0],clr[1],clr[2]];
    let hsl_in = HSL::from_rgb(&rgb_in);
    //~ let ll = match hsl_in.l {
            //~ 0.0 ... 0.3 => 0.3,
            //~ 0.3 ... 0.7 => 0.5+0.07,
            //~ _ => 1.0,
    //~ };
    //~ let ll = match hsl_in.l {
            //~ 0.0 ... 0.025 => 0.0,
            //~ 0.025 ... 0.05 => 0.01,
            //~ 0.05 ... 0.075 => 0.02,
            //~ 0.075 ... 0.1 => 0.03,
            //~ 0.1 ... 0.125 => 0.04,
            //~ 0.125 ... 0.15 => 0.05,
            //~ 0.15 ... 0.2 => 0.06,
            
            //~ 0.2 ... 0.4 => 0.3+0.03,
            //~ 0.4 ... 0.6 => 0.5+0.04,
            //~ 0.6 ... 0.8 => 0.7+0.07,
            //~ _ => 1.0,
    //~ };
    let ll = match hsl_in.l {
            l @ 0.0 ... 0.2 => l,
            
            0.2 ... 0.4 => 0.3+0.03,
            l @ 0.4 ... 0.405 => (l * 1.3)-0.075,
            0.405 ... 0.6 => 0.5+0.04,
            l @ 0.6 ... 0.605 => (l * 1.3)-0.0825,
            0.605 ... 0.8 => 0.7+0.07,
            l @ 0.8 ... 0.805 => (l * 1.3)-0.10,
            //~ _ => 0.95,
            0.805 ... 0.99 => 0.99,
            l => l,
    };
    //~ let ll = match hsl_in.l {
            //~ 0.0 ... 0.1 => 0.0,
            //~ 0.1 ... 0.2 => 0.15,
            //~ 0.2 ... 0.3 => 0.25,
            //~ 0.3 ... 0.4 => 0.35,
            //~ 0.4 ... 0.5 => 0.45,
            //~ 0.5 ... 0.6 => 0.55,
            //~ 0.6 ... 0.7 => 0.65,
            //~ 0.7 ... 0.8 => 0.75,
            //~ 0.8 ... 0.9 => 0.85,
            //~ _ => 1.0,
    //~ };
    //~ let ll = 0.5;
    //~ let ll = hsl_in.l;
    let hsl_out = HSL { h: hsl_in.h, s: hsl_in.s, l: ll };
    //~ let hsl_out = HSL { h: hsl_in.h, s: 0.8, l: ll };
    let (r, g, b) = hsl_out.to_rgb();
    Rgb([r,g,b])
}


fn main() {
    //buffer for display
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    //~ let mut wincap = minifb::Window::new("Input frame",
                                 //~ CAP_WIDTH as usize,
                                 //~ 50,
                                 //~ WindowOptions::default()).unwrap_or_else(|e| {
        //~ panic!("{}", e);
    //~ });
    
    let mut winf = minifb::Window::new("press Up, Down, Left, Right to adjust View - R to reset - Esc to exit - \"WIN + G\" is shortcut for Game DVR (Record) on Windows10",
                                 WIDTH,
                                 HEIGHT,
                                 //~ WindowOptions {
                                        //~ scale: minifb::Scale::FitScreen,
                                        //~ ..WindowOptions::default()}
                                 WindowOptions::default()
                                 ).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    
    //~ let screen = x11_screenshot::Screen::open().expect("Failed to open screen");
    let (mut capx, mut capy) = (100isize, 100isize);
    //~ wincap.set_position(capx, capy);
    
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    //capture loop thread
    thread::spawn(move || {
        let mut disp = 0;
        loop {
            let one_second = Duration::new(1, 0);
            let one_frame = one_second / 60;

            let display = Display::primary().expect("no display.");
            //~ let mut displays = Display::all().expect("no display.");//.iter().nth(disp as usize).unwrap();
            //~ let display = displayall.iter().nth(disp as usize).unwrap();
            //~ let display = &displays[0];
            let mut capturer = Capturer::new(display).expect("no capture.");
            let (mut x, mut y) = (100, 100);
            let (w, h) = (capturer.width(), capturer.height());
            'capt: loop {
                let buffer = match capturer.frame() {
                    Ok(buffer) => buffer,
                    Err(error) => {
                        if error.kind() == WouldBlock {
                            // Keep spinning.
                            thread::sleep(one_frame);
                            continue;
                        } else {
                            panic!("Error: {}", error);
                        }
                    }
                };
                ////////////////////////////
                let mut imgsc: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(w as u32, h as u32, buffer.to_vec()).unwrap();
                //~ ui  img.save("ueo.png").unwrap();
                //show
                let fr_got = image::imageops::crop(&mut imgsc,x as u32, y as u32, WIDTH as u32,HEIGHT as u32).to_image();
                let fr_send = map_colors(&fr_got,|p| {Rgb([p[2],p[1],p[0]])} );
                    //~ let (xx, yy) = rx1.recv().unwrap();
                    match rx1.recv().unwrap() {
                        (xx, yy) => {x=xx;y=yy;tx2.send(fr_send).unwrap();},
                        (d @ -10 ... -1 , _) => {
                            disp = -d;
                            tx2.send(fr_send).unwrap();
                            break 'capt;
                            },
                        _ => {},
                    }
                    //~ x=xx;
                    //~ y=yy;
                //~ tx2.send(fr_send).unwrap();
            }
        }
    });
    
        //~ let lapl = vec![0,1,0, 1,-4,1, 0,1,0];//laplacian filter
    let lapl = vec![1,1,1, 1,-8,1, 1,1,1];//laplacian8 filter
    //~ let lapl = vec![0.707,1.0,0.707, 1.0,-6.828,1.0, 0.707,1.0,0.707];//laplacian8 filter
    //~ let lapl = vec![0.7,1.0,0.7, 1.0,-6.8,1.0, 0.7,1.0,0.7];//laplacian8 filter
    //~ let lapl = vec![-1,0,1, -2, 0, 2, -1,0,1];//sobel filter
    
    while winf.is_open() && !winf.is_key_down(Key::Escape) {
        let start = Instant::now();
        //~ let frame = screen.capture();//.expect("Failed to take screenshot");
        //~ let img = screen.capture().expect("Failed to take screenshot");
        //~ wincap.update();
        if winf.is_key_down(Key::Right) {
            capx=capx+10;
        }
        if winf.is_key_down(Key::Left) {
            capx=capx-10;
        }
        if winf.is_key_down(Key::Up) {
            capy=capy-10;
        }
        if winf.is_key_down(Key::Down) {
            capy=capy+10;
        }
        if winf.is_key_down(Key::R) {
            capx=100;
            capy=100;
        }
        //~ match capx {
            
        //~ }
        tx1.send((capx, capy)).unwrap();
        //~ wincap.set_position(capx, capy);
        
        
        
        let img = rx2.recv().unwrap();

        let img2 = map_colors(&img,hsl_rank);
        let img3: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> = filter3x3(&img,&lapl);
        let img4 = grayscale(&img3);
        let img7 = map_colors2(&img4,&img2,|p, q| {
            //~ let r = 255u8-p[0];
            let hh = |a|{if a < 0{0}else{a as u8}};
            //~ let r = p[0] as i32;
            //~ let r = hh(p[0] as i32 - 0) as i32;
            let r = (p[0] as f64 *0.6) as i32;
            //~ Rgb([p[0]&q[0],p[0]&q[1],p[0]&q[2]])//line and color
            //~ Rgb([r&q[0],r&q[1],r&q[2]])//line and color
            
            //~ Rgba([p[0]&q[0],p[0]&q[1],p[0]&q[2],255])
            Rgb([hh(q[0] as i32 - r),hh(q[1] as i32 - r),hh(q[2] as i32 - r)])
            //~ Rgba([q[0],q[1],q[2],255])
            //~ Rgba([p[0],p[0],p[0],255])
            //~ Rgba([255,255,255,255])
            });
    for (i, j) in buffer.iter_mut().zip(img7.pixels()) {
            let r = j[0] as u32 * 65536;//16777216;
            let g = j[1] as u32 * 256;//65536;
            let b = j[2] as u32 ;
            //let a = j[3] as u32 * 16777216;
            let a = 255 as u32 * 16777216;
            *i = r  + g  + b + a; // write something more funny here!
        }
    //~ for i in img8.pixels() {
        
    //~ }
        //~ }
        //~ let men = &img7.into_raw();
        //~ println!("{}",men.len());
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        winf.update_with_buffer(&buffer).unwrap();
        
        let end = start.elapsed();
        println!("{}.{:03}sec/frame", end.as_secs(), end.subsec_nanos() / 1_000_000);

    }
    // Save image
    // For documentation on the image crate, see http://www.piston.rs/image/image/index.html
    //~ frame.save("./example_screenshot.png").unwrap();
}
