/// Color represented in HSL
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct HSL {
    /// Hue in 0-360 degree
    pub h: f64,
    /// Saturation in 0...1 (percent)
    pub s: f64,
    /// Luminosity in 0...1 (percent)
    pub l: f64,
}

impl HSL {
    /// Convert RGB pixel value to HSL
    ///
    /// Expects RGB pixel to be a slice of three `u8`s representing the red, green and blue values.
    ///
    /// ```rust
    /// use hsl::HSL;
    /// let blue = HSL::from_rgb(&[0, 0, 255]);
    /// ```
    ///
    /// Algorithm from [go-color] by Brandon Thomson <bt@brandonthomson.com>. (Iternally converts
    /// the pixel to RGB before converting it to HSL.)
    ///
    /// [go-color]: https://github.com/bthomson/go-color
    //~ #[cfg_attr(feature = "dev", allow(float_cmp))]
    pub fn from_rgb(rgb: &[u8]) -> HSL {
        //~ use std::cmp::{max, min};

        let mut h: f64;
        let s: f64;
        let l: f64;

        let (r, g, b) = (rgb[0], rgb[1], rgb[2]);
        //~ let maxin = [r, g, b];
        //~ let max = max(max(r, g), b);
        //~ let min = min(min(r, g), b);
        
        let max =
        if g>b {
            if r>g {
                r
            }else{
                g
            }
        }else{
            if r>b {
                r
            }else{
                b
            }
        };
        let min =
        if g<b {
            if r<g {
                r
            }else{
                g
            }
        }else{
            if r<b {
                r
            }else{
                b
            }
        };
        //~ let m = maxin.iter();
        //~ let max = *(m.max().unwrap());
        //~ let min = *(m.min().unwrap());
        //~ let () = min;
        //~ let (max, min)=(r,b);
        //~ let (max, min)=(g,b);
        //~ let (max, min)=(r,g);
        //~ let (max, min)=(g,r);
        //~ let (max, min)=(b,r);

        // Normalized RGB: Divide everything by 255 to get percentages of colors.
        let (r, g, b) = (r as f64 * 0.0039216_f64,
                         g as f64 * 0.0039216_f64,
                         b as f64 * 0.0039216_f64);
        let (min, max) = (min as f64 * 0.0039216_f64, max as f64 * 0.0039216_f64);

        // Luminosity is the average of the max and min rgb color intensities.
        l = (max + min) * 0.5_f64;

        // Saturation
        let delta: f64 = max - min;
        if delta == 0_f64 {
            // it's gray
            return HSL { h: 0_f64, s: 0_f64, l: l };
        }

        // it's not gray
        if l < 0.5_f64 {
            s = delta / (max + min);
        } else {
            s = delta / (2_f64 - max - min);
        }

        // Hue
        //~ let r2 = (((max - r) * 0.1666667_f64) + (delta * 0.5_f64)) / delta;
        //~ let g2 = (((max - g) * 0.1666667_f64) + (delta * 0.5_f64)) / delta;
        //~ let b2 = (((max - b) * 0.1666667_f64) + (delta * 0.5_f64)) / delta;
        /////////////////////
        let r2 = ((max - r) * 0.1666667_f64) / delta + 0.5_f64;
        let g2 = ((max - g) * 0.1666667_f64) / delta + 0.5_f64;
        let b2 = ((max - b) * 0.1666667_f64) / delta + 0.5_f64;

        h = match max {
            x if x == r => b2 - g2,
            x if x == g => 0.333333333_f64 + r2 - b2,
            _ => 0.66666667_f64 + g2 - r2,
        };

        // Fix wraparounds
        if h < 0 as f64 {
            h += 1_f64;
        } else if h > 1 as f64 {
            h -= 1_f64;
        }

        // Hue is precise to milli-degrees, e.g. `74.52deg`.
        //~ let h_degrees = (h * 360_f64 * 100_f64).round() * 0.01_f64;
        let h_degrees = (h * 360_f64 * 100_f64) * 0.01_f64;

        HSL { h: h_degrees, s: s, l: l }
    }

    /// Convert HSL color to RGB
    ///
    /// ```rust
    /// use hsl::HSL;
    ///
    /// let cyan = HSL { h: 180_f64, s: 1_f64, l: 0.5_f64 };
    /// assert_eq!(cyan.to_rgb(), (0, 255, 255));
    /// ```
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        //~ if self.s == 0.0 {
            //~ // Achromatic, i.e., grey.
            //~ let l = percent_to_byte(self.l);
            //~ return (l, l, l);
        //~ }

        let h = self.h * 0.00278;// / 360.0; // treat this as 0..1 instead of degrees
        let s = self.s;
        let l = self.l;

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - (l * s)
        };
        let p = 2.0 * l - q;

        (percent_to_byte(hue_to_rgb(p, q, h + 0.33333)),
         percent_to_byte(hue_to_rgb(p, q, h)),
         percent_to_byte(hue_to_rgb(p, q, h - 0.33333)))
    }
}

fn percent_to_byte(percent: f64) -> u8 {
    //~ (percent * 255.0).round() as u8
    (percent * 255.0) as u8
}

/// Convert Hue to RGB Ratio
///
/// From <https://github.com/jariz/vibrant.js/> by Jari Zwarts
fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    // Normalize
    let t = if t < 0.0 {
        t + 1.0
    } else if t > 1.0 {
        t - 1.0
    } else {
        t
    };
    
    if t < 0.166666667 {
        p + (q - p) * 6.0 * t
    } else if t < 0.5 {
        q
    } else if t < 0.66666667 {
        p + (q - p) * (0.66666667 - t) * 6.0
    } else {
        p
    }
}
