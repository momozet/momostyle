extern crate image;
extern crate imageproc;
extern crate scrap;

use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use minifb::{Key, WindowOptions};
use image::ImageBuffer;
use image::{Rgb, Rgba};
use image::imageops::colorops::grayscale;
use imageproc::map::map_colors;
use imageproc::map::map_colors2;
use momostyle::fastimg::yuv_rank;
use momostyle::fast3x3;

const WIDTH: usize = 900;const HEIGHT: usize = 600;

fn main() {
    //buffer for display
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut winf = minifb::Window::new("press Up, Down, Left, Right to adjust View - R to reset - Esc to exit - \"WIN + G\" is shortcut for Game DVR (Record) on Windows10",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()
                                 ).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    
    let (mut capx, mut capy) = (100isize, 100isize);    
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    //capture loop thread
    thread::spawn(move || {
        loop {
            let one_second = Duration::new(1, 0);
            let one_frame = one_second / 60;

            let display = Display::primary().expect("no display.");
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
                //show
                let fr_got = image::imageops::crop(&mut imgsc,x as u32, y as u32, WIDTH as u32,HEIGHT as u32).to_image();
                let fr_send = map_colors(&fr_got,|p| {Rgb([p[2],p[1],p[0]])});
                let (xx, yy) = rx1.recv().unwrap();
                x = xx;
                y = yy;
                tx2.send(fr_send).unwrap();
            }
        }
    });

    const LAPL: [i32;9] = [1,1,1, 1,-8,1, 1,1,1];//laplacian8 filter
    
    while winf.is_open() && !winf.is_key_down(Key::Escape) {
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
        tx1.send((capx, capy)).unwrap();
        
        let img = rx2.recv().unwrap();
        let start = Instant::now();
        let img2 = map_colors(&img,yuv_rank);
        let gr = grayscale(&img);
        let img4 = fast3x3::flt3x3(&gr,&LAPL);//custom flt3x3

        let img7 = map_colors2(&img4,&img2,|p, q| {
            let hh = |a|{if a < 0{0}else{a as u8}};
            let r = p[0] as i32 >>1;
            Rgb([hh(q[0] as i32 - r),hh(q[1] as i32 - r),hh(q[2] as i32 - r)])
        });
        for (i, j) in buffer.iter_mut().zip(img7.pixels()) {
            let r = j[0] as u32 * 65536;//16777216;
            let g = j[1] as u32 * 256;//65536;
            let b = j[2] as u32 ;
            let a = 255 as u32 * 16777216;
            *i = r  + g  + b + a; // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        winf.update_with_buffer(&buffer).unwrap();
        
        let end = start.elapsed();
        println!("{}.{:03}sec/frame", end.as_secs(), end.subsec_nanos() / 1_000_000);
    }
}
