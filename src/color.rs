use crate::rtweekend::*;
use std::io::{self, Write};

pub type Color = Vec3;

#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) -> io::Result<()> {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);
    let rbyte = (255.999 * intensity.clamp(r)) as u8;
    let gbyte = (255.999 * intensity.clamp(g)) as u8;
    let bbyte = (255.999 * intensity.clamp(b)) as u8;

    write!(out, "{} {} {}\n", rbyte, gbyte, bbyte)?;

    Ok(())
}
