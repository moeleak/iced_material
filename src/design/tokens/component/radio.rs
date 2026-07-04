pub const ICON_SIZE: f32 = 20.0;
pub const TARGET_SIZE: f32 = 48.0;
pub const OUTER_RING_WIDTH: f32 = 2.0;
pub const INNER_DOT_SIZE: f32 = 10.0;
pub const STATE_LAYER_SIZE: f32 = 40.0;
pub const LABEL_TEXT_SIZE: f32 = super::super::typography::BODY_LARGE.size;
pub const LABEL_TEXT_LINE_HEIGHT: f32 = super::super::typography::BODY_LARGE.line_height;
pub const LABEL_TEXT_WEIGHT: u16 = super::super::typography::BODY_LARGE.weight;
pub const SELECT_TRANSITION_DURATION_MS: u16 = super::super::motion::DURATION_MEDIUM2_MS;
pub const ICON_COLOR_TRANSITION_DURATION_MS: u16 = super::super::motion::DURATION_SHORT1_MS;
pub const SELECT_TRANSITION_EASING: super::super::motion::CubicBezier =
    super::super::motion::EASING_EMPHASIZED_DECELERATE;
pub const DISABLED_SELECTED_ICON_OPACITY: f32 = 0.38;
pub const DISABLED_UNSELECTED_ICON_OPACITY: f32 = 0.38;
