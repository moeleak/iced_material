//! Material 3 primary and secondary tab constructors.

use iced_widget::button::{Status, Style};
use iced_widget::core::border::Radius;
use iced_widget::core::text as core_text;
use iced_widget::core::time::Instant;
use iced_widget::core::{
    Background, Border, Color, Element, Layout, Length, Padding, Rectangle, Size, Widget,
    alignment, border, layout, mouse, renderer,
};
use iced_widget::graphics::geometry;
use iced_widget::renderer::wgpu::primitive;
use iced_widget::text;
use iced_widget::{Column, Container, Row, Space, Text};

use super::absolute_line_height;
use super::button::Button;
use super::support::{AnimatedScalar, duration_ms};
use crate::utils::{mix, shadow_from_level};
use crate::{Theme, fonts, tokens};

/// Animated tab selection state.
#[derive(Debug, Clone)]
pub struct State {
    selected_index: usize,
    indicator_position: AnimatedScalar,
}

impl State {
    /// Creates tab selection state with the initial selected index.
    pub fn new(selected_index: usize) -> Self {
        Self {
            selected_index,
            indicator_position: AnimatedScalar::new(selected_index as f32),
        }
    }

    /// Returns the selected tab index.
    pub const fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Starts the Material tab indicator transition to `selected_index`.
    pub fn select(&mut self, selected_index: usize, now: Instant, variant: Variant) {
        if self.selected_index == selected_index {
            return;
        }

        self.selected_index = selected_index;
        self.indicator_position.set_target(
            selected_index as f32,
            now,
            duration_ms(variant.indicator_animation_duration_ms()),
            variant.indicator_animation_easing(),
        );
    }

    /// Advances the running transition.
    pub fn advance(&mut self, now: Instant) -> bool {
        self.indicator_position.advance(now)
    }

    /// Returns whether the indicator transition is still running.
    pub fn is_animating(&self) -> bool {
        self.indicator_position.is_animating()
    }

    fn indicator_position(&self) -> f32 {
        self.indicator_position.value
    }
}

/// The Material tab variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Primary,
    Secondary,
}

impl Variant {
    const fn container_height(self) -> f32 {
        match self {
            Self::Primary => tokens::component::primary_tab::CONTAINER_HEIGHT,
            Self::Secondary => tokens::component::secondary_tab::CONTAINER_HEIGHT,
        }
    }

    const fn indicator_height(self) -> f32 {
        match self {
            Self::Primary => tokens::component::primary_tab::ACTIVE_INDICATOR_HEIGHT,
            Self::Secondary => tokens::component::secondary_tab::ACTIVE_INDICATOR_HEIGHT,
        }
    }

    const fn label_text(self) -> tokens::typography::TypeScale {
        match self {
            Self::Primary => tokens::component::primary_tab::LABEL_TEXT,
            Self::Secondary => tokens::component::secondary_tab::LABEL_TEXT,
        }
    }

    const fn icon_size(self) -> f32 {
        match self {
            Self::Primary => tokens::component::primary_tab::ICON_SIZE,
            Self::Secondary => tokens::component::secondary_tab::ICON_SIZE,
        }
    }

    const fn indicator_animation_duration_ms(self) -> u16 {
        match self {
            Self::Primary => tokens::component::primary_tab::INDICATOR_ANIMATION_DURATION_MS,
            Self::Secondary => tokens::component::secondary_tab::INDICATOR_ANIMATION_DURATION_MS,
        }
    }

    const fn indicator_animation_easing(self) -> tokens::motion::CubicBezier {
        match self {
            Self::Primary => tokens::component::primary_tab::INDICATOR_ANIMATION_EASING,
            Self::Secondary => tokens::component::secondary_tab::INDICATOR_ANIMATION_EASING,
        }
    }
}

/// How a tab renders its active indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicatorMode {
    /// The tab paints its own indicator. Use with [`bar`].
    Fixed,
    /// The tab reserves room for a shared indicator. Use with [`animated_bar`]
    /// or [`animated_tabs`].
    Shared,
}

/// How an icon-label tab arranges the icon and label.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconLabelLayout {
    /// Icon above label. Material defines this layout for primary tabs.
    Stacked,
    /// Icon and label in one row.
    Inline,
}

/// The content shown inside a tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Content<'a> {
    Label(text::Fragment<'a>),
    IconLabel {
        icon: text::Fragment<'a>,
        label: text::Fragment<'a>,
        layout: IconLabelLayout,
    },
}

impl<'a> Content<'a> {
    /// Creates label-only tab content.
    pub fn label(label: impl text::IntoFragment<'a>) -> Self {
        Self::Label(label.into_fragment())
    }

    /// Creates stacked icon-label tab content.
    pub fn stacked_icon_label(
        icon: impl text::IntoFragment<'a>,
        label: impl text::IntoFragment<'a>,
    ) -> Self {
        Self::IconLabel {
            icon: icon.into_fragment(),
            label: label.into_fragment(),
            layout: IconLabelLayout::Stacked,
        }
    }

    /// Creates inline icon-label tab content.
    pub fn inline_icon_label(
        icon: impl text::IntoFragment<'a>,
        label: impl text::IntoFragment<'a>,
    ) -> Self {
        Self::IconLabel {
            icon: icon.into_fragment(),
            label: label.into_fragment(),
            layout: IconLabelLayout::Inline,
        }
    }
}

/// Creates an equal-width Material tab bar.
pub fn bar<'a, Message, Renderer>(
    tabs: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
) -> Row<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    Row::with_children(tabs.into_iter())
        .spacing(0)
        .align_y(alignment::Vertical::Bottom)
        .width(Length::Fill)
}

/// Creates an equal-width Material tab bar with an animated shared indicator.
pub fn animated_bar<'a, Message, Renderer>(
    variant: Variant,
    tab_count: usize,
    state: &State,
    tabs: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
) -> Column<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    Column::new()
        .push(bar(tabs))
        .push(MovingIndicator {
            variant,
            tab_count,
            position: state.indicator_position(),
        })
        .spacing(0)
        .width(Length::Fill)
}

/// Creates a Material tab.
pub fn tab<'a, Message, Renderer>(
    variant: Variant,
    content: Content<'a>,
    active: bool,
    indicator_mode: IndicatorMode,
) -> Button<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: geometry::Renderer + core_text::Renderer + 'a,
    iced_widget::core::Font: Into<Renderer::Font>,
{
    match content {
        Content::Label(label) => label_tab(variant, label, active, indicator_mode),
        Content::IconLabel {
            icon,
            label,
            layout,
        } => icon_label_tab(variant, icon, label, active, indicator_mode, layout),
    }
}

/// Creates an animated Material tab bar from action items.
pub fn animated_tabs<'a, Message, Renderer>(
    variant: Variant,
    state: &State,
    tabs: impl IntoIterator<Item = (Content<'a>, Message)>,
) -> Column<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: geometry::Renderer + primitive::Renderer + core_text::Renderer + 'a,
    iced_widget::core::Font: Into<Renderer::Font>,
{
    let tabs: Vec<_> = tabs
        .into_iter()
        .enumerate()
        .map(|(index, (content, on_press))| {
            tab(
                variant,
                content,
                state.selected_index() == index,
                IndicatorMode::Shared,
            )
            .on_press(on_press)
            .into()
        })
        .collect();

    animated_bar(variant, tabs.len(), state, tabs)
}

fn label_tab<'a, Message, Renderer>(
    variant: Variant,
    label: text::Fragment<'a>,
    active: bool,
    indicator_mode: IndicatorMode,
) -> Button<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: geometry::Renderer + core_text::Renderer + 'a,
{
    let label_text = variant.label_text();
    tab_button_for_mode(
        variant,
        Text::new(label)
            .size(label_text.size)
            .line_height(absolute_line_height(label_text.line_height))
            .into(),
        active,
        variant.container_height(),
        indicator_mode,
    )
}

fn icon_label_tab<'a, Message, Renderer>(
    variant: Variant,
    icon: text::Fragment<'a>,
    label: text::Fragment<'a>,
    active: bool,
    indicator_mode: IndicatorMode,
    layout: IconLabelLayout,
) -> Button<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: geometry::Renderer + core_text::Renderer + 'a,
    iced_widget::core::Font: Into<Renderer::Font>,
{
    match (variant, layout) {
        (Variant::Primary, IconLabelLayout::Stacked) => {
            stacked_icon_label_tab(variant, icon, label, active, indicator_mode)
        }
        _ => inline_icon_label_tab(variant, icon, label, active, indicator_mode),
    }
}

fn stacked_icon_label_tab<'a, Message, Renderer>(
    variant: Variant,
    icon: text::Fragment<'a>,
    label: text::Fragment<'a>,
    active: bool,
    indicator_mode: IndicatorMode,
) -> Button<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: geometry::Renderer + core_text::Renderer + 'a,
    iced_widget::core::Font: Into<Renderer::Font>,
{
    let label_text = variant.label_text();
    let content = Column::<Message, Theme, Renderer>::new()
        .push(fonts::filled_icon(icon, variant.icon_size()))
        .push(
            Text::new(label)
                .size(label_text.size)
                .line_height(absolute_line_height(label_text.line_height)),
        )
        .spacing(tokens::component::primary_tab::STACKED_ICON_LABEL_SPACE)
        .align_x(alignment::Horizontal::Center);

    tab_button_for_mode(
        variant,
        content.into(),
        active,
        tokens::component::primary_tab::WITH_ICON_AND_LABEL_TEXT_CONTAINER_HEIGHT,
        indicator_mode,
    )
}

fn inline_icon_label_tab<'a, Message, Renderer>(
    variant: Variant,
    icon: text::Fragment<'a>,
    label: text::Fragment<'a>,
    active: bool,
    indicator_mode: IndicatorMode,
) -> Button<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: geometry::Renderer + core_text::Renderer + 'a,
    iced_widget::core::Font: Into<Renderer::Font>,
{
    let label_text = variant.label_text();
    let gap = match variant {
        Variant::Primary => tokens::component::primary_tab::INLINE_ICON_LABEL_SPACE,
        Variant::Secondary => tokens::component::secondary_tab::ICON_LABEL_SPACE,
    };
    let content = Row::<Message, Theme, Renderer>::new()
        .push(fonts::filled_icon(icon, variant.icon_size()))
        .push(
            Text::new(label)
                .size(label_text.size)
                .line_height(absolute_line_height(label_text.line_height)),
        )
        .spacing(gap)
        .align_y(alignment::Vertical::Center);

    tab_button_for_mode(
        variant,
        content.into(),
        active,
        variant.container_height(),
        indicator_mode,
    )
}

fn tab_button_for_mode<'a, Message, Renderer>(
    variant: Variant,
    content: Element<'a, Message, Theme, Renderer>,
    active: bool,
    height: f32,
    indicator_mode: IndicatorMode,
) -> Button<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: geometry::Renderer + core_text::Renderer + 'a,
{
    match indicator_mode {
        IndicatorMode::Fixed => tab_button(variant, content, active, height),
        IndicatorMode::Shared => animated_tab_button(variant, content, active, height),
    }
}

fn tab_button<'a, Message, Renderer>(
    variant: Variant,
    content: Element<'a, Message, Theme, Renderer>,
    active: bool,
    height: f32,
) -> Button<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: geometry::Renderer + core_text::Renderer + 'a,
{
    tab_button_with_indicator(variant, content, active, height, true)
}

fn animated_tab_button<'a, Message, Renderer>(
    variant: Variant,
    content: Element<'a, Message, Theme, Renderer>,
    active: bool,
    height: f32,
) -> Button<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: geometry::Renderer + core_text::Renderer + 'a,
{
    tab_button_with_indicator(
        variant,
        content,
        active,
        height - variant.indicator_height(),
        false,
    )
}

fn tab_button_with_indicator<'a, Message, Renderer>(
    variant: Variant,
    content: Element<'a, Message, Theme, Renderer>,
    active: bool,
    height: f32,
    show_indicator: bool,
) -> Button<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: geometry::Renderer + core_text::Renderer + 'a,
{
    let tab_content = Column::new().push(
        Container::new(content)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .padding(Padding {
                top: 0.0,
                right: horizontal_space(variant),
                bottom: 0.0,
                left: horizontal_space(variant),
            }),
    );
    let tab_content = if show_indicator {
        tab_content.push(indicator(variant, active))
    } else {
        tab_content
    }
    .width(Length::Fill)
    .height(Length::Fixed(height));

    Button::new(tab_content)
        .width(Length::Fill)
        .height(Length::Fixed(height))
        .padding(Padding::ZERO)
        .style(move |theme, status| tab_style(theme, status, variant, active))
}

fn indicator<'a, Message, Renderer>(
    variant: Variant,
    active: bool,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    Container::new(Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(variant.indicator_height()))
        .style(move |theme| indicator_style(theme, variant, active))
}

const fn horizontal_space(variant: Variant) -> f32 {
    match variant {
        Variant::Primary => tokens::component::primary_tab::HORIZONTAL_SPACE,
        Variant::Secondary => tokens::component::secondary_tab::HORIZONTAL_SPACE,
    }
}

/// Returns the container style for a Material tab.
pub fn tab_style(theme: &Theme, status: Status, variant: Variant, active: bool) -> Style {
    let colors = theme.colors();
    let surface = colors.surface;
    let content = tab_content_color(theme, variant, active, status);
    let layer = tab_state_layer_color(theme, variant, active, status);
    let container = surface.color;

    let active_style = Style {
        background: Some(Background::Color(container)),
        text_color: content,
        border: border::rounded(tab_container_shape(variant)),
        shadow: shadow_from_level(tab_container_elevation(variant), colors.shadow),
        snap: cfg!(feature = "crisp"),
    };

    match status {
        Status::Active => active_style,
        Status::Hovered => Style {
            background: Some(Background::Color(mix(
                container,
                layer,
                tab_hover_opacity(variant, active),
            ))),
            ..active_style
        },
        Status::Pressed => Style {
            background: Some(Background::Color(mix(
                container,
                layer,
                tab_pressed_opacity(variant, active),
            ))),
            ..active_style
        },
        Status::Disabled => Style {
            background: Some(Background::Color(container)),
            text_color: Color {
                a: tokens::state::DISABLED_LABEL_TEXT_OPACITY,
                ..surface.text
            },
            ..active_style
        },
    }
}

fn tab_content_color(theme: &Theme, variant: Variant, active: bool, status: Status) -> Color {
    let colors = theme.colors();

    match (variant, active, status) {
        (Variant::Primary, true, _) => colors.primary.color,
        (Variant::Primary, false, Status::Active) => colors.surface.text_variant,
        (Variant::Primary, false, _) => colors.surface.text,
        (Variant::Secondary, true, _) => colors.surface.text,
        (Variant::Secondary, false, Status::Active) => colors.surface.text_variant,
        (Variant::Secondary, false, _) => colors.surface.text,
    }
}

fn tab_state_layer_color(theme: &Theme, variant: Variant, active: bool, status: Status) -> Color {
    let colors = theme.colors();

    match (variant, active, status) {
        (Variant::Primary, true, _) => colors.primary.color,
        (Variant::Primary, false, Status::Pressed) => colors.primary.color,
        (Variant::Primary, false, _) => colors.surface.text,
        (Variant::Secondary, _, _) => colors.surface.text,
    }
}

const fn tab_hover_opacity(variant: Variant, active: bool) -> f32 {
    match (variant, active) {
        (Variant::Primary, true) => {
            tokens::component::primary_tab::ACTIVE_HOVER_STATE_LAYER_OPACITY
        }
        (Variant::Primary, false) => {
            tokens::component::primary_tab::INACTIVE_HOVER_STATE_LAYER_OPACITY
        }
        (Variant::Secondary, _) => tokens::component::secondary_tab::HOVER_STATE_LAYER_OPACITY,
    }
}

const fn tab_pressed_opacity(variant: Variant, active: bool) -> f32 {
    match (variant, active) {
        (Variant::Primary, true) => {
            tokens::component::primary_tab::ACTIVE_PRESSED_STATE_LAYER_OPACITY
        }
        (Variant::Primary, false) => {
            tokens::component::primary_tab::INACTIVE_PRESSED_STATE_LAYER_OPACITY
        }
        (Variant::Secondary, _) => tokens::component::secondary_tab::PRESSED_STATE_LAYER_OPACITY,
    }
}

const fn tab_container_shape(variant: Variant) -> f32 {
    match variant {
        Variant::Primary => tokens::component::primary_tab::CONTAINER_SHAPE,
        Variant::Secondary => tokens::component::secondary_tab::CONTAINER_SHAPE,
    }
}

const fn tab_container_elevation(variant: Variant) -> u8 {
    match variant {
        Variant::Primary => tokens::component::primary_tab::CONTAINER_ELEVATION_LEVEL,
        Variant::Secondary => tokens::component::secondary_tab::CONTAINER_ELEVATION_LEVEL,
    }
}

fn indicator_style(theme: &Theme, variant: Variant, active: bool) -> iced_widget::container::Style {
    let colors = theme.colors();
    let background = if active {
        colors.primary.color
    } else {
        Color::TRANSPARENT
    };

    iced_widget::container::Style {
        background: Some(Background::Color(background)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: indicator_radius(variant),
        },
        snap: cfg!(feature = "crisp"),
        ..Default::default()
    }
}

fn indicator_radius(variant: Variant) -> Radius {
    match variant {
        Variant::Primary => Radius {
            top_left: tokens::component::primary_tab::ACTIVE_INDICATOR_SHAPE_TOP,
            top_right: tokens::component::primary_tab::ACTIVE_INDICATOR_SHAPE_TOP,
            bottom_right: tokens::component::primary_tab::ACTIVE_INDICATOR_SHAPE_BOTTOM,
            bottom_left: tokens::component::primary_tab::ACTIVE_INDICATOR_SHAPE_BOTTOM,
        },
        Variant::Secondary => Radius::new(tokens::component::secondary_tab::ACTIVE_INDICATOR_SHAPE),
    }
}

#[derive(Debug, Clone, Copy)]
struct MovingIndicator {
    variant: Variant,
    tab_count: usize,
    position: f32,
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for MovingIndicator
where
    Renderer: iced_widget::core::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fixed(self.variant.indicator_height()),
        }
    }

    fn layout(
        &mut self,
        _tree: &mut iced_widget::core::widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(limits.resolve(
            Length::Fill,
            Length::Fixed(self.variant.indicator_height()),
            Size::ZERO,
        ))
    }

    fn draw(
        &self,
        _tree: &iced_widget::core::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _defaults: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        if self.tab_count == 0 {
            return;
        }

        let bounds = layout.bounds();
        let tab_width = bounds.width / self.tab_count as f32;

        if tab_width <= 0.0 {
            return;
        }

        let position = self
            .position
            .clamp(0.0, self.tab_count.saturating_sub(1) as f32);
        let indicator_width = moving_indicator_width(self.variant, tab_width);
        let x = bounds.x + tab_width * position + (tab_width - indicator_width) / 2.0;
        let indicator_bounds = Rectangle {
            x,
            y: bounds.y,
            width: indicator_width,
            height: self.variant.indicator_height(),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: indicator_bounds,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: indicator_radius(self.variant),
                },
                snap: cfg!(feature = "crisp"),
                ..renderer::Quad::default()
            },
            Background::Color(theme.colors().primary.color),
        );
    }
}

impl<'a, Message, Renderer> From<MovingIndicator> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    fn from(indicator: MovingIndicator) -> Self {
        Element::new(indicator)
    }
}

fn moving_indicator_width(variant: Variant, tab_width: f32) -> f32 {
    match variant {
        Variant::Primary => (tab_width - horizontal_space(variant) * 2.0).max(0.0),
        Variant::Secondary => tab_width,
    }
}

#[cfg(test)]
#[path = "../../../tests/widget/component/tabs.rs"]
mod tests;
