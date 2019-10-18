#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct YUV {
    pub y: f64,
    pub u: f64,
    pub v: f64,
}

impl YUV {
    pub fn from_rgb(rgb: &[u8]) -> YUV {
        let (r, g, b) = (rgb[0] as f64, rgb[1] as f64, rgb[2] as f64);
        
        let yy = r * 0.25 + g * 0.5 + b * 0.25;
        let cg = r * (-0.25) + g * 0.5 + b * (-0.25);
        let co = r * 0.5  + b * (-0.5);
        YUV { y: yy, u: cg, v: co }
    }
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        let y = self.y;
        let u = self.u;
        let v = self.v;
        (
            clamp(y-u+v) as u8,
            clamp(y+u) as u8,
            clamp(y-u-v) as u8
        )
    }
}

pub fn clamp(input: f64) -> f64{
    if input < 0.0 {
        0.0
    } else if input > 255.0 {
        255.0
    } else {
        input
    }
}
