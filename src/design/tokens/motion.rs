#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubicBezier {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl CubicBezier {
    pub const fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    pub fn transform(self, progress: f32) -> f32 {
        if progress <= 0.0 {
            return 0.0;
        }

        if progress >= 1.0 {
            return 1.0;
        }

        let target_x = progress.clamp(0.0, 1.0);
        let mut start = 0.0;
        let mut end = 1.0;

        for _ in 0..20 {
            let midpoint = (start + end) / 2.0;

            if bezier_axis(midpoint, self.x1, self.x2) < target_x {
                start = midpoint;
            } else {
                end = midpoint;
            }
        }

        bezier_axis((start + end) / 2.0, self.y1, self.y2).clamp(0.0, 1.0)
    }
}

fn bezier_axis(t: f32, p1: f32, p2: f32) -> f32 {
    let inverse = 1.0 - t;

    3.0 * inverse * inverse * t * p1 + 3.0 * inverse * t * t * p2 + t * t * t
}

pub const DURATION_SHORT1_MS: u16 = 50;
pub const DURATION_SHORT2_MS: u16 = 100;
pub const DURATION_SHORT3_MS: u16 = 150;
pub const DURATION_SHORT4_MS: u16 = 200;
pub const DURATION_MEDIUM1_MS: u16 = 250;
pub const DURATION_MEDIUM2_MS: u16 = 300;
pub const DURATION_MEDIUM3_MS: u16 = 350;
pub const DURATION_MEDIUM4_MS: u16 = 400;
pub const DURATION_LONG1_MS: u16 = 450;
pub const DURATION_LONG2_MS: u16 = 500;
pub const DURATION_LONG3_MS: u16 = 550;
pub const DURATION_LONG4_MS: u16 = 600;
pub const DURATION_EXTRA_LONG1_MS: u16 = 700;
pub const DURATION_EXTRA_LONG2_MS: u16 = 800;
pub const DURATION_EXTRA_LONG3_MS: u16 = 900;
pub const DURATION_EXTRA_LONG4_MS: u16 = 1000;

pub const EASING_EMPHASIZED: CubicBezier = CubicBezier::new(0.2, 0.0, 0.0, 1.0);
pub const EASING_EMPHASIZED_ACCELERATE: CubicBezier = CubicBezier::new(0.3, 0.0, 0.8, 0.15);
pub const EASING_EMPHASIZED_DECELERATE: CubicBezier = CubicBezier::new(0.05, 0.7, 0.1, 1.0);
pub const EASING_STANDARD: CubicBezier = CubicBezier::new(0.2, 0.0, 0.0, 1.0);
pub const EASING_STANDARD_ACCELERATE: CubicBezier = CubicBezier::new(0.3, 0.0, 1.0, 1.0);
pub const EASING_STANDARD_DECELERATE: CubicBezier = CubicBezier::new(0.0, 0.0, 0.0, 1.0);
pub const EASING_LINEAR: CubicBezier = CubicBezier::new(0.0, 0.0, 1.0, 1.0);
pub const EASING_LEGACY: CubicBezier = CubicBezier::new(0.4, 0.0, 0.2, 1.0);
pub const EASING_LEGACY_ACCELERATE: CubicBezier = CubicBezier::new(0.4, 0.0, 1.0, 1.0);
pub const EASING_LEGACY_DECELERATE: CubicBezier = CubicBezier::new(0.0, 0.0, 0.2, 1.0);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    pub damping_ratio: f32,
    pub stiffness: f32,
}

pub const SPRING_DEFAULT_DISPLACEMENT_THRESHOLD: f32 = 0.01;

pub const EXPRESSIVE_DEFAULT_SPATIAL: Spring = Spring {
    damping_ratio: 0.8,
    stiffness: 380.0,
};
pub const EXPRESSIVE_DEFAULT_EFFECTS: Spring = Spring {
    damping_ratio: 1.0,
    stiffness: 1600.0,
};
pub const EXPRESSIVE_FAST_SPATIAL: Spring = Spring {
    damping_ratio: 0.6,
    stiffness: 800.0,
};
pub const EXPRESSIVE_FAST_EFFECTS: Spring = Spring {
    damping_ratio: 1.0,
    stiffness: 3800.0,
};
pub const EXPRESSIVE_SLOW_SPATIAL: Spring = Spring {
    damping_ratio: 0.8,
    stiffness: 200.0,
};
pub const EXPRESSIVE_SLOW_EFFECTS: Spring = Spring {
    damping_ratio: 1.0,
    stiffness: 800.0,
};
