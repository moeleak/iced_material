//! Material 3 bottom sheet surface constructors.

use iced_widget::core::text as core_text;
use iced_widget::core::time::Instant;
use iced_widget::core::{Background, Color, Element, Length, Padding, alignment, border};
use iced_widget::{Column, Container, Space};

use super::support::{AnimatedScalar, alpha_color, duration_ms, lerp};
use crate::utils::shadow_from_level;
use crate::{Theme, tokens};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Hidden,
    PartiallyExpanded,
    Expanded,
}

impl Value {
    const fn progress(self) -> f32 {
        match self {
            Self::Hidden => 0.0,
            Self::PartiallyExpanded => 0.5,
            Self::Expanded => 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    from: Value,
    target: Value,
    progress: AnimatedScalar,
}

impl State {
    pub fn new(value: Value) -> Self {
        Self {
            from: value,
            target: value,
            progress: AnimatedScalar::new(1.0),
        }
    }

    pub const fn target(&self) -> Value {
        self.target
    }

    pub fn show(&mut self, now: Instant) {
        self.set_target(Value::PartiallyExpanded, now);
    }

    pub fn expand(&mut self, now: Instant) {
        self.set_target(Value::Expanded, now);
    }

    pub fn partial_expand(&mut self, now: Instant) {
        self.set_target(Value::PartiallyExpanded, now);
    }

    pub fn hide(&mut self, now: Instant) {
        self.set_target(Value::Hidden, now);
    }

    pub fn set_target(&mut self, target: Value, now: Instant) {
        if self.target == target && !self.progress.is_animating() {
            return;
        }

        self.from = self.current_value();
        self.target = target;
        self.progress = AnimatedScalar::new(0.0);
        self.progress.set_target(
            1.0,
            now,
            duration_ms(tokens::component::bottom_sheet::ANIMATION_DURATION_MS),
            tokens::component::bottom_sheet::ANIMATION_EASING,
        );
    }

    pub fn advance(&mut self, now: Instant) -> bool {
        self.progress.advance(now)
    }

    pub fn is_animating(&self) -> bool {
        self.progress.is_animating()
    }

    pub fn visibility_progress(&self) -> f32 {
        lerp(
            self.from.progress(),
            self.target.progress(),
            self.progress.value.clamp(0.0, 1.0),
        )
    }

    fn current_value(&self) -> Value {
        if self.progress.value >= 1.0 {
            self.target
        } else {
            self.from
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    const fn alignment(self) -> alignment::Horizontal {
        match self {
            Self::Left => alignment::Horizontal::Left,
            Self::Right => alignment::Horizontal::Right,
        }
    }

    const fn hidden_translation_fraction(self) -> f32 {
        match self {
            Self::Left => -1.0,
            Self::Right => 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideValue {
    Hidden,
    Expanded,
}

impl SideValue {
    const fn progress(self) -> f32 {
        match self {
            Self::Hidden => 0.0,
            Self::Expanded => 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SideState {
    from: SideValue,
    target: SideValue,
    progress: AnimatedScalar,
}

impl SideState {
    pub fn new(value: SideValue) -> Self {
        Self {
            from: value,
            target: value,
            progress: AnimatedScalar::new(1.0),
        }
    }

    pub const fn target(&self) -> SideValue {
        self.target
    }

    pub fn show(&mut self, now: Instant) {
        self.set_target(SideValue::Expanded, now);
    }

    pub fn hide(&mut self, now: Instant) {
        self.set_target(SideValue::Hidden, now);
    }

    pub fn set_target(&mut self, target: SideValue, now: Instant) {
        if self.target == target && !self.progress.is_animating() {
            return;
        }

        self.from = self.current_value();
        self.target = target;
        self.progress = AnimatedScalar::new(0.0);
        self.progress.set_target(
            1.0,
            now,
            duration_ms(tokens::component::side_sheet::ANIMATION_DURATION_MS),
            tokens::component::side_sheet::ANIMATION_EASING,
        );
    }

    pub fn advance(&mut self, now: Instant) -> bool {
        self.progress.advance(now)
    }

    pub fn is_animating(&self) -> bool {
        self.progress.is_animating()
    }

    pub fn visibility_progress(&self) -> f32 {
        lerp(
            self.from.progress(),
            self.target.progress(),
            self.progress.value.clamp(0.0, 1.0),
        )
    }

    pub fn translation_fraction(&self, side: Side) -> f32 {
        side.hidden_translation_fraction() * (1.0 - self.visibility_progress())
    }

    fn current_value(&self) -> SideValue {
        if self.progress.value >= 1.0 {
            self.target
        } else {
            self.from
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Modal,
    Standard,
}

pub fn modal_bottom<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    bottom_sheet(Kind::Modal, content, true)
}

pub fn standard_bottom<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    bottom_sheet(Kind::Standard, content, true)
}

pub fn bottom_sheet<'a, Message, Renderer>(
    kind: Kind,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    show_drag_handle: bool,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    let mut column = Column::new().width(Length::Fill);

    if show_drag_handle {
        column = column.push(drag_handle());
    }

    column = column.push(content.into());

    Container::new(column)
        .width(Length::Fill)
        .max_width(tokens::component::bottom_sheet::SHEET_MAX_WIDTH)
        .style(move |theme| bottom_sheet_style(theme, kind))
}

pub fn bottom_content<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    Container::new(content)
        .padding(Padding {
            top: 0.0,
            right: tokens::component::bottom_sheet::CONTENT_PADDING,
            bottom: tokens::component::bottom_sheet::CONTENT_PADDING,
            left: tokens::component::bottom_sheet::CONTENT_PADDING,
        })
        .width(Length::Fill)
}

pub fn drag_handle<'a, Message, Renderer>() -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    let handle = Container::new(
        Space::new()
            .width(Length::Fixed(
                tokens::component::bottom_sheet::DRAG_HANDLE_WIDTH,
            ))
            .height(Length::Fixed(
                tokens::component::bottom_sheet::DRAG_HANDLE_HEIGHT,
            )),
    )
    .style(drag_handle_style);

    Container::new(handle)
        .width(Length::Fill)
        .padding(Padding {
            top: tokens::component::bottom_sheet::DRAG_HANDLE_VERTICAL_PADDING,
            right: 0.0,
            bottom: tokens::component::bottom_sheet::DRAG_HANDLE_VERTICAL_PADDING,
            left: 0.0,
        })
        .align_x(alignment::Horizontal::Center)
}

pub fn scrim<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(scrim_style)
}

pub fn modal_overlay<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    scrim(
        Container::new(modal_bottom(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Bottom),
    )
}

pub fn modal_side<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    modal_side_on(Side::Right, content)
}

pub fn modal_side_on<'a, Message, Renderer>(
    side: Side,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    side_sheet(Kind::Modal, side, false, content)
}

pub fn detached_modal_side<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    detached_modal_side_on(Side::Right, content)
}

pub fn detached_modal_side_on<'a, Message, Renderer>(
    side: Side,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    side_sheet(Kind::Modal, side, true, content)
}

pub fn standard_side<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    standard_side_on(Side::Right, content)
}

pub fn standard_side_on<'a, Message, Renderer>(
    side: Side,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    side_sheet(Kind::Standard, side, false, content)
}

pub fn detached_standard_side<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    detached_standard_side_on(Side::Right, content)
}

pub fn detached_standard_side_on<'a, Message, Renderer>(
    side: Side,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    side_sheet(Kind::Standard, side, true, content)
}

pub fn side_sheet<'a, Message, Renderer>(
    kind: Kind,
    side: Side,
    detached: bool,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    let surface = Container::new(content)
        .width(Length::Fixed(
            tokens::component::side_sheet::DOCKED_CONTAINER_WIDTH,
        ))
        .height(Length::Fill)
        .style(move |theme| side_sheet_style(theme, kind, side, detached));

    Container::new(surface)
        .height(Length::Fill)
        .padding(if detached {
            tokens::component::side_sheet::DETACHED_MARGIN
        } else {
            0.0
        })
}

pub fn side_content<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    Container::new(content)
        .padding(tokens::component::side_sheet::CONTENT_PADDING)
        .width(Length::Fill)
        .height(Length::Fill)
}

pub fn modal_side_overlay<'a, Message, Renderer>(
    side: Side,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    side_scrim(
        Container::new(modal_side_on(side, content))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(side.alignment()),
    )
}

pub fn side_scrim<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(side_scrim_style)
}

fn bottom_sheet_radius() -> border::Radius {
    border::Radius {
        top_left: tokens::component::bottom_sheet::CONTAINER_SHAPE_TOP,
        top_right: tokens::component::bottom_sheet::CONTAINER_SHAPE_TOP,
        bottom_right: tokens::component::bottom_sheet::CONTAINER_SHAPE_BOTTOM,
        bottom_left: tokens::component::bottom_sheet::CONTAINER_SHAPE_BOTTOM,
    }
}

fn side_sheet_radius(kind: Kind, side: Side, detached: bool) -> border::Radius {
    if detached {
        return border::rounded(tokens::component::side_sheet::DETACHED_CONTAINER_SHAPE).radius;
    }

    match kind {
        Kind::Standard => {
            border::rounded(tokens::component::side_sheet::DOCKED_STANDARD_CONTAINER_SHAPE).radius
        }
        Kind::Modal => {
            let radius = tokens::component::side_sheet::DOCKED_MODAL_CONTAINER_SHAPE;

            match side {
                Side::Left => border::Radius {
                    top_left: 0.0,
                    top_right: radius,
                    bottom_right: radius,
                    bottom_left: 0.0,
                },
                Side::Right => border::Radius {
                    top_left: radius,
                    top_right: 0.0,
                    bottom_right: 0.0,
                    bottom_left: radius,
                },
            }
        }
    }
}

fn bottom_sheet_style(theme: &Theme, kind: Kind) -> iced_widget::container::Style {
    let colors = theme.colors();
    let level = match kind {
        Kind::Modal => tokens::component::bottom_sheet::MODAL_CONTAINER_ELEVATION_LEVEL,
        Kind::Standard => tokens::component::bottom_sheet::STANDARD_CONTAINER_ELEVATION_LEVEL,
    };

    iced_widget::container::Style {
        background: Some(Background::Color(colors.surface.container.low)),
        text_color: Some(colors.surface.text),
        border: border::rounded(bottom_sheet_radius()),
        shadow: shadow_from_level(level, colors.shadow),
        snap: cfg!(feature = "crisp"),
    }
}

fn side_sheet_style(
    theme: &Theme,
    kind: Kind,
    side: Side,
    detached: bool,
) -> iced_widget::container::Style {
    let colors = theme.colors();
    let (container, level) = match kind {
        Kind::Modal => (
            colors.surface.container.low,
            tokens::component::side_sheet::MODAL_CONTAINER_ELEVATION_LEVEL,
        ),
        Kind::Standard => (
            colors.surface.color,
            tokens::component::side_sheet::STANDARD_CONTAINER_ELEVATION_LEVEL,
        ),
    };

    iced_widget::container::Style {
        background: Some(Background::Color(container)),
        text_color: Some(colors.surface.text),
        border: border::rounded(side_sheet_radius(kind, side, detached)),
        shadow: shadow_from_level(level, colors.shadow),
        snap: cfg!(feature = "crisp"),
    }
}

fn drag_handle_style(theme: &Theme) -> iced_widget::container::Style {
    iced_widget::container::Style {
        background: Some(Background::Color(theme.colors().surface.text_variant)),
        border: border::rounded(tokens::shape::CORNER_FULL),
        snap: cfg!(feature = "crisp"),
        ..iced_widget::container::Style::default()
    }
}

fn scrim_style(theme: &Theme) -> iced_widget::container::Style {
    iced_widget::container::Style {
        background: Some(Background::Color(alpha_color(
            Color {
                a: 1.0,
                ..theme.colors().scrim
            },
            tokens::component::bottom_sheet::SCRIM_OPACITY,
        ))),
        text_color: Some(theme.colors().surface.text),
        ..iced_widget::container::Style::default()
    }
}

fn side_scrim_style(theme: &Theme) -> iced_widget::container::Style {
    iced_widget::container::Style {
        background: Some(Background::Color(alpha_color(
            Color {
                a: 1.0,
                ..theme.colors().scrim
            },
            tokens::component::side_sheet::SCRIM_OPACITY,
        ))),
        text_color: Some(theme.colors().surface.text),
        ..iced_widget::container::Style::default()
    }
}

#[cfg(test)]
mod tests {
    use iced_widget::core::time::Duration;

    use super::*;

    #[test]
    fn bottom_sheet_radius_rounds_top_corners_only() {
        let radius = bottom_sheet_radius();

        assert_eq!(
            radius.top_left,
            tokens::component::bottom_sheet::CONTAINER_SHAPE_TOP
        );
        assert_eq!(
            radius.top_right,
            tokens::component::bottom_sheet::CONTAINER_SHAPE_TOP
        );
        assert_eq!(radius.bottom_left, 0.0);
        assert_eq!(radius.bottom_right, 0.0);
    }

    #[test]
    fn bottom_sheet_style_uses_surface_container_low_and_level1() {
        let theme = Theme::Light;
        let colors = theme.colors();
        let style = bottom_sheet_style(&theme, Kind::Modal);

        assert_eq!(
            style.background,
            Some(Background::Color(colors.surface.container.low))
        );
        assert_eq!(style.text_color, Some(colors.surface.text));
        assert_eq!(
            style.shadow.offset.y,
            tokens::elevation::shadow(1).ambient.y
        );
        assert_eq!(
            style.shadow.blur_radius,
            tokens::elevation::shadow(1).ambient.blur
        );
    }

    #[test]
    fn side_sheet_radius_matches_docked_and_detached_edges() {
        let right = side_sheet_radius(Kind::Modal, Side::Right, false);
        assert_eq!(
            right.top_left,
            tokens::component::side_sheet::DOCKED_MODAL_CONTAINER_SHAPE
        );
        assert_eq!(right.top_right, 0.0);
        assert_eq!(right.bottom_right, 0.0);
        assert_eq!(
            right.bottom_left,
            tokens::component::side_sheet::DOCKED_MODAL_CONTAINER_SHAPE
        );

        let left = side_sheet_radius(Kind::Modal, Side::Left, false);
        assert_eq!(left.top_left, 0.0);
        assert_eq!(
            left.top_right,
            tokens::component::side_sheet::DOCKED_MODAL_CONTAINER_SHAPE
        );
        assert_eq!(
            left.bottom_right,
            tokens::component::side_sheet::DOCKED_MODAL_CONTAINER_SHAPE
        );
        assert_eq!(left.bottom_left, 0.0);

        let standard = side_sheet_radius(Kind::Standard, Side::Right, false);
        assert_eq!(standard.top_left, 0.0);
        assert_eq!(standard.top_right, 0.0);
        assert_eq!(standard.bottom_left, 0.0);
        assert_eq!(standard.bottom_right, 0.0);

        let detached = side_sheet_radius(Kind::Standard, Side::Right, true);
        assert_eq!(
            detached.top_left,
            tokens::component::side_sheet::DETACHED_CONTAINER_SHAPE
        );
        assert_eq!(
            detached.top_right,
            tokens::component::side_sheet::DETACHED_CONTAINER_SHAPE
        );
        assert_eq!(
            detached.bottom_left,
            tokens::component::side_sheet::DETACHED_CONTAINER_SHAPE
        );
        assert_eq!(
            detached.bottom_right,
            tokens::component::side_sheet::DETACHED_CONTAINER_SHAPE
        );
    }

    #[test]
    fn side_sheet_styles_use_material_container_roles_and_elevation() {
        let theme = Theme::Light;
        let colors = theme.colors();

        let modal = side_sheet_style(&theme, Kind::Modal, Side::Right, false);
        assert_eq!(
            modal.background,
            Some(Background::Color(colors.surface.container.low))
        );
        assert_eq!(modal.text_color, Some(colors.surface.text));
        assert_eq!(
            modal.shadow.offset.y,
            tokens::elevation::shadow(1).ambient.y
        );

        let standard = side_sheet_style(&theme, Kind::Standard, Side::Right, false);
        assert_eq!(
            standard.background,
            Some(Background::Color(colors.surface.color))
        );
        assert_eq!(
            standard.shadow.offset.y,
            tokens::elevation::shadow(0).ambient.y
        );
        assert_eq!(
            standard.shadow.blur_radius,
            tokens::elevation::shadow(0).ambient.blur
        );
    }

    #[test]
    fn drag_handle_uses_on_surface_variant_color() {
        let theme = Theme::Light;
        let style = drag_handle_style(&theme);

        assert_eq!(
            style.background,
            Some(Background::Color(theme.colors().surface.text_variant))
        );
        assert_eq!(style.border.radius.top_left, tokens::shape::CORNER_FULL);
    }

    #[test]
    fn sheet_content_wrappers_apply_material_fill_constraints() {
        let bottom: Container<'_, (), Theme, iced_widget::Renderer> = bottom_content(Space::new());
        let bottom_size =
            iced_widget::core::Widget::<(), Theme, iced_widget::Renderer>::size(&bottom);
        assert_eq!(bottom_size.width, Length::Fill);

        let side: Container<'_, (), Theme, iced_widget::Renderer> = side_content(Space::new());
        let side_size = iced_widget::core::Widget::<(), Theme, iced_widget::Renderer>::size(&side);
        assert_eq!(side_size.width, Length::Fill);
        assert_eq!(side_size.height, Length::Fill);
    }

    #[test]
    fn scrim_uses_material_opacity_over_scrim_color() {
        let theme = Theme::Light;
        let style = scrim_style(&theme);
        let Some(Background::Color(color)) = style.background else {
            panic!("expected solid scrim background");
        };

        assert_eq!(color.a, tokens::component::bottom_sheet::SCRIM_OPACITY);
    }

    #[test]
    fn side_scrim_uses_material_opacity_over_scrim_color() {
        let theme = Theme::Light;
        let style = side_scrim_style(&theme);
        let Some(Background::Color(color)) = style.background else {
            panic!("expected solid scrim background");
        };

        assert_eq!(color.a, tokens::component::side_sheet::SCRIM_OPACITY);
    }

    #[test]
    fn sheet_state_animates_with_material_timing() {
        let start = Instant::now();
        let mut state = State::new(Value::Hidden);

        state.show(start);
        assert_eq!(state.target(), Value::PartiallyExpanded);
        assert_eq!(state.visibility_progress(), 0.0);
        assert!(state.is_animating());

        let _ = state.advance(start + Duration::from_millis(150));
        assert!(state.visibility_progress() > 0.0);
        assert!(state.visibility_progress() < 0.5);

        let _ = state.advance(
            start
                + Duration::from_millis(u64::from(
                    tokens::component::bottom_sheet::ANIMATION_DURATION_MS,
                )),
        );
        assert_eq!(state.visibility_progress(), 0.5);
        assert!(!state.is_animating());
    }

    #[test]
    fn side_sheet_state_animates_with_material_timing() {
        let start = Instant::now();
        let mut state = SideState::new(SideValue::Hidden);

        state.show(start);
        assert_eq!(state.target(), SideValue::Expanded);
        assert_eq!(state.visibility_progress(), 0.0);
        assert_eq!(state.translation_fraction(Side::Right), 1.0);
        assert_eq!(state.translation_fraction(Side::Left), -1.0);
        assert!(state.is_animating());

        let _ = state.advance(start + Duration::from_millis(138));
        assert!(state.visibility_progress() > 0.0);
        assert!(state.visibility_progress() < 1.0);

        let _ = state.advance(
            start
                + Duration::from_millis(u64::from(
                    tokens::component::side_sheet::ANIMATION_DURATION_MS,
                )),
        );
        assert_eq!(state.visibility_progress(), 1.0);
        assert_eq!(state.translation_fraction(Side::Right), 0.0);
        assert!(!state.is_animating());

        state.hide(start);
        assert_eq!(state.target(), SideValue::Hidden);
    }
}
