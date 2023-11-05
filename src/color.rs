#[derive(Clone, Copy)]
pub struct Color(pub [f32; 4]);

impl From<Color> for [f32; 4] {
    fn from(color: Color) -> [f32; 4] {
        color.0
    }
}

use std::ops::{Add, Div, Mul};

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from(
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
        )
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Self::from(
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
            self.0[3] * rhs.0[3],
        )
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Self::from(
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        )
    }
}

impl Div<f32> for Color {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::from(
            self.0[0] / rhs,
            self.0[1] / rhs,
            self.0[2] / rhs,
            self.0[3] / rhs,
        )
    }
}

#[allow(clippy::eq_op)]
impl Color {
    fn from(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self([r, g, b, a])
    }
    pub fn into_u8(self) -> [u8; 4] {
        [
            (self.0[0] * 255.0).max(0.0).min(255.0) as u8,
            (self.0[1] * 255.0).max(0.0).min(255.0) as u8,
            (self.0[2] * 255.0).max(0.0).min(255.0) as u8,
            (self.0[3] * 255.0).max(0.0).min(255.0) as u8,
        ]
    }
    pub const LIGHTGRAY: Color =
        Color([200.0 / 255.0, 200.0 / 255.0, 200.0 / 255.0, 255.0 / 255.0]);
    pub const GRAY: Color = Color([130.0 / 255.0, 130.0 / 255.0, 130.0 / 255.0, 255.0 / 255.0]);
    pub const DARKGRAY: Color = Color([80.0 / 255.0, 80.0 / 255.0, 80.0 / 255.0, 255.0 / 255.0]);
    pub const YELLOW: Color = Color([253.0 / 255.0, 249.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0]);
    pub const GOLD: Color = Color([255.0 / 255.0, 203.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0]);
    pub const ORANGE: Color = Color([255.0 / 255.0, 161.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0]);
    pub const PINK: Color = Color([255.0 / 255.0, 109.0 / 255.0, 194.0 / 255.0, 255.0 / 255.0]);
    pub const RED: Color = Color([230.0 / 255.0, 41.0 / 255.0, 55.0 / 255.0, 255.0 / 255.0]);
    pub const MAROON: Color = Color([190.0 / 255.0, 33.0 / 255.0, 55.0 / 255.0, 255.0 / 255.0]);
    pub const GREEN: Color = Color([0.0 / 255.0, 228.0 / 255.0, 48.0 / 255.0, 255.0 / 255.0]);
    pub const LIME: Color = Color([0.0 / 255.0, 158.0 / 255.0, 47.0 / 255.0, 255.0 / 255.0]);
    pub const DARKGREEN: Color = Color([0.0 / 255.0, 117.0 / 255.0, 44.0 / 255.0, 255.0 / 255.0]);
    pub const SKYBLUE: Color = Color([102.0 / 255.0, 191.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0]);
    pub const BLUE: Color = Color([0.0 / 255.0, 121.0 / 255.0, 241.0 / 255.0, 255.0 / 255.0]);
    pub const DARKBLUE: Color = Color([0.0 / 255.0, 82.0 / 255.0, 172.0 / 255.0, 255.0 / 255.0]);
    pub const PURPLE: Color = Color([200.0 / 255.0, 122.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0]);
    pub const VIOLET: Color = Color([135.0 / 255.0, 60.0 / 255.0, 190.0 / 255.0, 255.0 / 255.0]);
    pub const DARKPURPLE: Color =
        Color([112.0 / 255.0, 31.0 / 255.0, 126.0 / 255.0, 255.0 / 255.0]);
    pub const BEIGE: Color = Color([211.0 / 255.0, 176.0 / 255.0, 131.0 / 255.0, 255.0 / 255.0]);
    pub const BROWN: Color = Color([127.0 / 255.0, 106.0 / 255.0, 79.0 / 255.0, 255.0 / 255.0]);
    pub const DARKBROWN: Color = Color([76.0 / 255.0, 63.0 / 255.0, 47.0 / 255.0, 255.0 / 255.0]);
    pub const WHITE: Color = Color([255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0]);
    pub const BLACK: Color = Color([0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0]);
    pub const BLANK: Color = Color([0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0]);
    pub const MAGENTA: Color = Color([255.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0]);
    pub const RAYWHITE: Color = Color([245.0 / 255.0, 245.0 / 255.0, 245.0 / 255.0, 255.0 / 255.0]);
}

