extern crate image;
extern crate imageproc;
//~ extern crate x11_screenshot;
extern crate scrap;

use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
//~ use std::fs::File;
use std::thread;
use std::time::Duration;

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
const CAP_WIDTH: u32 = 1080;
const CAP_HEIGHT: u32 = 720;
//~ const CAP_WIDTH: u32 = 100;
//~ const CAP_HEIGHT: u32 = 100;
//~ const WIDTH: usize = 100;
//~ const HEIGHT: usize = 100;

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

fn wrender(img: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>{
    let lapl = vec![1,1,1, 1,-8,1, 1,1,1];//laplacian8 filter
    let img3: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> = filter3x3(&img,&lapl);
    //~ let img3: image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>> = filter3x3(&img,&lapl);
    let img4 = grayscale(&img3);
    //~ let img4 = img3;
    let img5 = map_colors(&img4,|p| {
            //~ let sss = |x: u8|{255u8-(16u8-x)*(16u8-x)-1};
            Luma(
                match p[0] {
                    p => [(p as f32 *0.3 )as u8],
                })}
            );

    let img6 = map_colors(&img5,|p| {Luma([255u8-p[0]])});
    let img7 = map_colors2(&img6,&img,|p, q| {
            Rgb([p[0]&q[0],p[0]&q[1],p[0]&q[2]])//line and color
            });
    img7
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
    
    tx1.send((capx, capy)).unwrap();
    let mut img = rx2.recv().unwrap();
    //~ let mut img = screen.capture_area(CAP_WIDTH ,CAP_HEIGHT , 100, 100).expect("Failed to take screenshot");
    //~ let hsl_rank = |clr: image::Rgb<u8>| {clr};   
    //~ let hsl_rank3 = |clr: image::Rgb<u8>| {
            //~ let rgb_in = [clr[0],clr[1],clr[2]];
            //~ let hsl_in = HSL::from_rgb(&rgb_in);
            //~ let ll = match hsl_in.l {
                    //~ 0.0 ... 0.2 => 0.05,
                    //~ 0.2 ... 0.4 => 0.3+0.05,
                    //~ 0.4 ... 0.6 => 0.5+0.05,
                    //~ 0.6 ... 0.8 => 0.7+0.05,
                    //~ _ => 1.0,
            //~ };
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
            //~ let hsl_out = HSL { h: hsl_in.h, s: hsl_in.s, l: ll };
            //~ let hsl_out = HSL { h: hsl_in.h, s: 0.8, l: ll };
            //~ let (r, g, b) = hsl_out.to_rgb();
            //~ Rgb([r,g,b])
        //~ };
    //~ let iro = |cl: u8| {cl};
    //~ let iro = |cl: u8| {
                    //~ match cl {
                        //~ 0 ... 50 => 50,
                        //~ 51 ... 100 => 100,
                        //~ 101 ... 150 => 200,
                        //~ _ => 255,
                    //~ }
                //~ };
    //~ let iro = |cl: u8| {
        //~ match cl {
            //~ 0 ... 25 => 20,
            //~ 26 ... 50 => 50,
            //~ 51 ... 75 => 75,
            //~ 76 ... 100 => 100,
            //~ 101 ... 125 => 125,
            //~ 126 ... 150 => 150,
            //~ 151 ... 175 => 175,
            //~ 176 ... 250 => 200,
            //~ _ => 255,
        //~ }
    //~ };
    let mut img19 = map_colors(&img,|p| {Rgb([0,0,0])});//grayscale(&img);
    //~ let lapl = vec![0,1,0, 1,-4,1, 0,1,0];//laplacian filter
    let mut img2 = median_filter(&img19, 1);
    let lapl = vec![1,1,1, 1,-8,1, 1,1,1];//laplacian8 filter
    //~ let lapl = vec![1,2,1, 2,-12,2, 1,2,1];//laplacian8 filter
    //~ let lapl = vec![-1,0,1, -2, 0, 2, -1,0,1];//laplacian8 filter
    let mut img3: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> = filter3x3(&img,&lapl);
    let mut img4 = grayscale(&img3);
    let mut img5 = map_colors(&img4,|p| {Luma([0])});
    let mut img6 = map_colors(&img5,|p| {Luma([255u8-p[0]])});
    //~ let mut img7 = map_colors2(&img6,&img2,|p, q| {Rgba([255,255,255,255])});
    let mut img7 = map_colors2(&img6,&img2,|p, q| {Rgb([255,255,255])});
    let filtype = image::FilterType::Lanczos3;
    //~ let filtype = image::FilterType::CatmullRom;
    //~ let filtype = image::FilterType::Gaussian;
    //~ let filtype = image::FilterType::Nearest;
    //~ let filtype = image::FilterType::Triangle;
    let mut img8 = resize(&img7, WIDTH as u32, HEIGHT as u32,filtype);
    //~ let mut img8 = img7.clone();
    
    while winf.is_open() && !winf.is_key_down(Key::Escape) {
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
        
        
        
        img = rx2.recv().unwrap();
        //~ img = screen.capture_area(CAP_WIDTH ,CAP_HEIGHT, 100, 100).expect("Failed to take screenshot");
        //~ let img = screen.capture_area(150 ,100 , 300, 300).expect("Failed to take screenshot");        
        //~ let img = match frame {
                //~ Some(fr) => fr,
                //~ None => return,
            //~ };
        
        //~ let lapl = vec![0,1,0, 1,-4,1, 0,1,0];//laplacian filter
        //~ lapl = vec![1,1,1, 1,-8,1, 1,1,1];//laplacian8 filter
        ////////////////////////////////////////////////////////
        // median filter reduce noise
        //~ img19 = map_colors(&img,hsl_rank);//grayscale(&img);
        //~ let av = vec![0.0,0.1,0.0, 0.1,0.6,0.1, 0.0,0.1,0.0];//average filter
        //~ img2 = filter3x3(&img19,&av);
        //~ img2 = median_filter(&img19, 1);
        //~ img2 = img.clone();
        img2 = map_colors(&img,hsl_rank);
        img3 = filter3x3(&img,&lapl);
        //~ img3 = img.clone();
        ////////////////////////////////////////////////////
        //~ let ss = vec![-1, 0 ,1];
        //~ let ssi = vec![1, 0 ,-1];
        //~ let ss1 = vec![1, 3 ,1];
        //~ let imgh = separable_filter(&img,&ss,&ss1);
        //~ let imghh = separable_filter(&img,&ssi,&ss1);
        //~ let imgv = separable_filter(&img,&ss1,&ss);
        //~ let imgvv = separable_filter(&img,&ssi,&ss1);
        //~ let lplp: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> = filter3x3(&img,&lapl);
        //~ let hv = map_colors2(&imgh,&imgv,|p, q| {
            //~ Rgb([p[0]|q[0],p[0]|q[1],p[0]|q[2]]) });
        //~ let hhvv = map_colors2(&imghh,&imgvv,|p, q| {
            //~ Rgb([p[0]|q[0],p[0]|q[1],p[0]|q[2]]) });
        //~ let hvs = map_colors2(&hv,&hhvv,|p, q| {
            //~ Rgb([p[0]|q[0],p[0]|q[1],p[0]|q[2]]) });
        //~ img3 = map_colors2(&hvs,&lplp,|p, q| {
            //~ Rgb([p[0]|q[0],p[0]|q[1],p[0]|q[2]]) });
        ////////////////////////////////////////////////////////
        img4 = grayscale(&img3);
        img5 = map_colors(&img4,|p| {
            //~ let sss = |x: u8|{255u8-(16u8-x)*(16u8-x)-1};
            Luma(
                match p[0] {
                    p => [(p as f32 *0.8 )as u8],
                    //~ p => [(p as f32 *0.9 )as u8],
                    //~ 0 => [0],
                    //~ p @ 1 ... 15 => [0],
                    //~ p @ 16 ... 50 => [p+30],
                    //~ p @ 51 ... 150 => [p+30],
                    //~ p => [180],
                ///////////////////////////////
                    //~ 0 ... 15 => [0],
                    //~ p => [(sss(p-15) as f32 * 0.8) as u8],
                    //~ p => [(p as f32 * 0.7 ) as u8]
                    //~ _ => [0],
                    //~ p @ 16 ... 200 => [p-15],
                    //~ _ => [185],
                    
                })}
            );
        //~ let invg = |p| {Luma(p[0])};
        //~ let limu8 = |p, q| {
            //~ let s = p as u
            //~ };
        //~ let gaus = vec![1.0/16.0,2.0/16.0,1.0/16.0,
                        //~ 2.0/16.0,4.0/16.0,2.0/16.0,
                        //~ 1.0/16.0,2.0/16.0,1.0/16.0];//gausian filter
        //~ let gaus = vec![0.0,2.0/16.0,0.0,
                        //~ 2.0/16.0,1.0,2.0/16.0,
                        //~ 0.0,2.0/16.0,0.0];
        //~ let imgg: image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>> = filter3x3(&img5,&gaus);
        img6 = map_colors(&img5,|p| {Luma([255u8-p[0]])});
        img7 = map_colors2(&img6,&img2,|p, q| {
            Rgb([p[0]&q[0],p[0]&q[1],p[0]&q[2]])//line and color
            //~ Rgba([p[0]&q[0],p[0]&q[1],p[0]&q[2],255])
            //~ Rgba([p[0]+q[0],p[0]+q[1],p[0]+q[2],255])
            //~ Rgba([q[0],q[1],q[2],255])
            //~ Rgba([p[0],p[0],p[0],255])
            //~ Rgba([255,255,255,255])
            });
        //~ let img6 = map_colors(&img5,|p| {Luma([255u8-p[0]])});//morphology::open(&img5,Norm::LInf,1);
        //~ let img7 = morphology::dilate(&img6,Norm::LInf,1);
        //~ let img8 = morphology::erode(&img7,Norm::LInf,1);
        //~ display_image("title",&img,150,150);
        //~ let filtype = image::FilterType::Lanczos3;//CatmullRom
        //~ let filtype = image::FilterType::CatmullRom;
        //~ let filtype = image::FilterType::Gaussian;
        //~ let filtype = image::FilterType::Nearest;
        //~ let filtype = image::FilterType::Triangle;
        //~ img8 = resize(&img7, WIDTH as u32, HEIGHT as u32,filtype);
        //~ img8 = wrender(img7.clone());
    //~ }
    //~ while winf.is_open() && !winf.is_key_down(Key::Escape) {
    //~ buffer.iter_mut().zip(img8.pixels()).map(|(i, j)|{1});
    //~ buffer.par_iter_mut().zip(img8.pixels()).map(|(i, j)|{1});
    //~ let () = img8;
    //~ buffer.iter_mut().map(|i: &mut u32|{
        //~ *i=255u32;});
    //~ println!("{}",buffer[2]);
    //~ imm.par_iter().zip(buffer.par_iter_mut()).map(|(i, j): (&u8,&mut u32)|{*j=255*255u32;});
    //~ let () = img8.par_iter_mut();   : (u32, &mut u32)
    for (i, j) in buffer.iter_mut().zip(img7.pixels()) {
            let r = j[0] as u32 * 65536;//16777216;
            let g = j[1] as u32 * 256;//65536;
            let b = j[2] as u32 ;
            //let a = j[3] as u32 * 16777216;
            *i = r  + g  + b ; // write something more funny here!
        }
    //~ for i in img8.pixels() {
        
    //~ }
        //~ }
        //~ let men = &img7.into_raw();
        //~ println!("{}",men.len());
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        winf.update_with_buffer(&buffer).unwrap();
    }
    // Save image
    // For documentation on the image crate, see http://www.piston.rs/image/image/index.html
    //~ frame.save("./example_screenshot.png").unwrap();
}
