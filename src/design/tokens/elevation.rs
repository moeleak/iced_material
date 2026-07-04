pub const LEVEL0: f32 = 0.0;
pub const LEVEL1: f32 = 1.0;
pub const LEVEL2: f32 = 3.0;
pub const LEVEL3: f32 = 6.0;
pub const LEVEL4: f32 = 8.0;
pub const LEVEL5: f32 = 12.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadowLayer {
    pub y: f32,
    pub blur: f32,
    pub spread: f32,
    pub opacity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shadow {
    pub key: ShadowLayer,
    pub ambient: ShadowLayer,
}

const fn layer(y: f32, blur: f32, spread: f32, opacity: f32) -> ShadowLayer {
    ShadowLayer {
        y,
        blur,
        spread,
        opacity,
    }
}

pub const fn level(level: u8) -> f32 {
    match level {
        0 => LEVEL0,
        1 => LEVEL1,
        2 => LEVEL2,
        3 => LEVEL3,
        4 => LEVEL4,
        _ => LEVEL5,
    }
}

pub const fn shadow(level: u8) -> Shadow {
    match level {
        0 => Shadow {
            key: layer(0.0, 0.0, 0.0, 0.3),
            ambient: layer(0.0, 0.0, 0.0, 0.15),
        },
        1 => Shadow {
            key: layer(1.0, 2.0, 0.0, 0.3),
            ambient: layer(1.0, 3.0, 1.0, 0.15),
        },
        2 => Shadow {
            key: layer(1.0, 2.0, 0.0, 0.3),
            ambient: layer(2.0, 6.0, 2.0, 0.15),
        },
        3 => Shadow {
            key: layer(1.0, 3.0, 0.0, 0.3),
            ambient: layer(4.0, 8.0, 3.0, 0.15),
        },
        4 => Shadow {
            key: layer(2.0, 3.0, 0.0, 0.3),
            ambient: layer(6.0, 10.0, 4.0, 0.15),
        },
        _ => Shadow {
            key: layer(4.0, 4.0, 0.0, 0.3),
            ambient: layer(8.0, 12.0, 6.0, 0.15),
        },
    }
}
