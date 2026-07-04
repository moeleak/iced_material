#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypefaceRole {
    Brand,
    Plain,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypeScale {
    pub role: TypefaceRole,
    pub size: f32,
    pub line_height: f32,
    pub tracking: f32,
    pub weight: u16,
}

const fn scale(
    role: TypefaceRole,
    size: f32,
    line_height: f32,
    tracking: f32,
    weight: u16,
) -> TypeScale {
    TypeScale {
        role,
        size,
        line_height,
        tracking,
        weight,
    }
}

pub const WEIGHT_REGULAR: u16 = 400;
pub const WEIGHT_MEDIUM: u16 = 500;
pub const WEIGHT_BOLD: u16 = 700;

pub const DISPLAY_LARGE: TypeScale = scale(TypefaceRole::Brand, 57.0, 64.0, -0.25, WEIGHT_REGULAR);
pub const DISPLAY_MEDIUM: TypeScale = scale(TypefaceRole::Brand, 45.0, 52.0, 0.0, WEIGHT_REGULAR);
pub const DISPLAY_SMALL: TypeScale = scale(TypefaceRole::Brand, 36.0, 44.0, 0.0, WEIGHT_REGULAR);
pub const HEADLINE_LARGE: TypeScale = scale(TypefaceRole::Brand, 32.0, 40.0, 0.0, WEIGHT_REGULAR);
pub const HEADLINE_MEDIUM: TypeScale = scale(TypefaceRole::Brand, 28.0, 36.0, 0.0, WEIGHT_REGULAR);
pub const HEADLINE_SMALL: TypeScale = scale(TypefaceRole::Brand, 24.0, 32.0, 0.0, WEIGHT_REGULAR);
pub const TITLE_LARGE: TypeScale = scale(TypefaceRole::Brand, 22.0, 28.0, 0.0, WEIGHT_REGULAR);
pub const TITLE_MEDIUM: TypeScale = scale(TypefaceRole::Plain, 16.0, 24.0, 0.15, WEIGHT_MEDIUM);
pub const TITLE_SMALL: TypeScale = scale(TypefaceRole::Plain, 14.0, 20.0, 0.1, WEIGHT_MEDIUM);
pub const LABEL_LARGE: TypeScale = scale(TypefaceRole::Plain, 14.0, 20.0, 0.1, WEIGHT_MEDIUM);
pub const LABEL_MEDIUM: TypeScale = scale(TypefaceRole::Plain, 12.0, 16.0, 0.5, WEIGHT_MEDIUM);
pub const LABEL_SMALL: TypeScale = scale(TypefaceRole::Plain, 11.0, 16.0, 0.5, WEIGHT_MEDIUM);
pub const BODY_LARGE: TypeScale = scale(TypefaceRole::Plain, 16.0, 24.0, 0.5, WEIGHT_REGULAR);
pub const BODY_MEDIUM: TypeScale = scale(TypefaceRole::Plain, 14.0, 20.0, 0.25, WEIGHT_REGULAR);
pub const BODY_SMALL: TypeScale = scale(TypefaceRole::Plain, 12.0, 16.0, 0.4, WEIGHT_REGULAR);
