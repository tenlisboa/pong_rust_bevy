use bevy::color::Color;

pub const fn from_rgb(r: f32, g: f32, b: f32) -> Color {
    Color::srgb(r / 255., g / 255., b / 255.)
}
