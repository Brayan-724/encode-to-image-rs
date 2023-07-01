use std::fmt::Display;

use image::Rgba;

use crate::utils::ToHex;

#[derive(Debug, Clone)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hex = self.as_hex(true);

        f.write_fmt(format_args!("Color(#{})", hex))
    }
}

impl Color {
    pub const WHITE: Color = Color(255, 255, 255, 255);
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color(r, g, b, 255)
    }

    pub const fn new_alpha(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color(r, g, b, a)
    }

    // MANIPULATORS

    pub fn is_similar(&self, target: &Color, threshold: u8) -> bool {
        let distance = ((self.0 as i32 - target.0 as i32).pow(2)
            + (self.1 as i32 - target.1 as i32).pow(2)
            + (self.2 as i32 - target.2 as i32).pow(2)
            + (self.3 as i32 - target.3 as i32).pow(2)) as f32;

        let distance = distance.sqrt().abs() as u8;
        distance < threshold
    }

    // CONVERTERS

    pub const fn from_slice(slice: &[u8; 4]) -> Color {
        Color(slice[0], slice[1], slice[2], slice[3])
    }

    pub fn from_hex(value: String) -> Color {
        let value = value.trim_start_matches('#').chars();
        let mut r: u8 = 0;
        let mut g: u8 = 0;
        let mut b: u8 = 0;
        let mut a: u8 = 255;

        let mut carrier: Option<char> = None;
        let mut color_i: u8 = 0;
        for c in value {
            match c {
                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                    if let Some(carrier_c) = carrier {
                        let color = carrier_c.to_string() + &c.to_string();
                        let color = u8::from_str_radix(&color, 16).unwrap();
                        match color_i {
                            0 => r = color,
                            1 => g = color,
                            2 => b = color,
                            3 => a = color,
                            _ => break,
                        }
                        carrier = None;
                        color_i += 1;
                    } else {
                        carrier = Some(c);
                    }
                }
                _ => break,
            }
        }

        Color(r, g, b, a)
    }

    pub fn as_hex(&self, alpha: bool) -> String {
        let r = self.0.to_hex();
        let g = self.1.to_hex();
        let b = self.2.to_hex();

        if alpha {
            let a = self.3.to_hex();
            r + &g + &b + &a
        } else {
            r + &g + &b
        }
    }

    pub const fn to_rgba_pixel(&self) -> Rgba<u8> {
        Rgba([self.0, self.1, self.2, self.3])
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color(
            self.0.wrapping_sub(rhs.0),
            self.1.wrapping_sub(rhs.1),
            self.2.wrapping_sub(rhs.2),
            255
            // self.3.abs_diff(rhs.3),
        )
    }
}

impl std::ops::Sub for &Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color(
            self.0.abs_diff(rhs.0),
            self.1.abs_diff(rhs.1),
            self.2.abs_diff(rhs.2),
            self.3.abs_diff(rhs.3),
        )
    }
}
impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.0.wrapping_add(rhs.0),
            self.1.wrapping_add(rhs.1),
            self.2.wrapping_add(rhs.2),
            self.3.wrapping_add(rhs.3),
        )
    }
}

impl From<String> for Color {
    fn from(value: String) -> Self {
        Color::from_hex(value)
    }
}
