#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElevationLevels {
    pub active: u8,
    pub hovered: u8,
    pub pressed: u8,
    pub dragged: u8,
    pub disabled: u8,
}

pub const CONTAINER_SHAPE: f32 = super::super::shape::CORNER_MEDIUM;
pub const ICON_SIZE: f32 = 24.0;
pub const DISABLED_CONTAINER_OPACITY: f32 = 0.38;
pub const OUTLINED_OUTLINE_WIDTH: f32 = 1.0;
pub const OUTLINED_DISABLED_OUTLINE_OPACITY: f32 = 0.12;

pub const ELEVATED_ELEVATION: ElevationLevels = ElevationLevels {
    active: 1,
    hovered: 2,
    pressed: 1,
    dragged: 4,
    disabled: 1,
};
pub const FILLED_ELEVATION: ElevationLevels = ElevationLevels {
    active: 0,
    hovered: 1,
    pressed: 0,
    dragged: 3,
    disabled: 0,
};
pub const OUTLINED_ELEVATION: ElevationLevels = ElevationLevels {
    active: 0,
    hovered: 1,
    pressed: 0,
    dragged: 3,
    disabled: 0,
};
