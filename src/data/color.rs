use bevy::{
    prelude::*
};
use serde::{Serialize, Deserialize};

// Names from https://chir.ag/projects/name-that-color/
#[derive(Clone, Copy, Deserialize, Debug, Hash, PartialEq, Serialize)]
pub enum Palette {
    Shark,
    SaltBox,
    Wafer,
    Geraldine,
    DullLavender,
    RegentStBlue,
    JaggedIce,
    LavenderRose,
    Envy,
    Sulu,
    Sandwisp,
    Tequila,
    Tumbleweed,
    Chardonay,
    Picasso,
    EarlyDawn,
    // Dev
    DevWhite,
    DevCustom {
        r: u8,
        g: u8,
        b: u8,
        #[serde(default = "default_alpha")]
        a: u8,
    },
}
fn default_alpha() -> u8 { 255 }

impl Palette {
    pub fn color(&self) -> Color {
        // Vanilla Milkshake Palette by Space Sandwich https://lospec.com/palette-list/vanilla-milkshake
        match self {
            Palette::Shark        => Color::rgb(0.157, 0.157, 0.180), // #28282e
            Palette::SaltBox      => Color::rgb(0.424, 0.337, 0.443), // #6c5671
            Palette::Wafer        => Color::rgb(0.951, 0.784, 0.749), // #d9c8bf
            Palette::Geraldine    => Color::rgb(0.976, 0.510, 0.518), // #f98284
            Palette::DullLavender => Color::rgb(0.690, 0.663, 0.894), // #b0a9e4
            Palette::RegentStBlue => Color::rgb(0.675, 0.800, 0.894), // #accce4
            Palette::JaggedIce    => Color::rgb(0.702, 0.890, 0.855), // #b3e3da
            Palette::LavenderRose => Color::rgb(0.996, 0.667, 0.894), // #feaae4
            Palette::Envy         => Color::rgb(0.529, 0.659, 0.537), // #87a889
            Palette::Sulu         => Color::rgb(0.690, 0.922, 0.576), // #b0eb93
            Palette::Sandwisp     => Color::rgb(0.914, 0.961, 0.616), // #e9f59d
            Palette::Tequila      => Color::rgb(1.000, 0.902, 0.776), // #ffe6c6
            Palette::Tumbleweed   => Color::rgb(0.872, 0.639, 0.545), // #dea38b
            Palette::Chardonay    => Color::rgb(1.000, 0.765, 0.518), // #ffc384
            Palette::Picasso      => Color::rgb(1.000, 0.969, 0.627), // #fff7a0
            Palette::EarlyDawn    => Color::rgb(1.000, 0.969, 0.894), // #fff7e4
            // Dev
            Palette::DevWhite     => Color::WHITE, // #ffffff
            Palette::DevCustom {r, g, b, a} => Color::rgba_u8(r.clone(), g.clone(), b.clone(), a.clone()),
        }
    }
}