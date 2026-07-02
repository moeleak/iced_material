//! Material 3 navigation bar, rail, drawer, and adaptive layout helpers.

use iced_widget::button;
use iced_widget::core::text as core_text;
use iced_widget::core::time::Instant;
use iced_widget::core::{Background, Color, Element, Length, Padding, alignment, border};
use iced_widget::text::{self, LineHeight};
use iced_widget::{Button, Column, Container, Row, Space, Stack, Text};

use super::support::{AnimatedScalar, duration_ms, lerp};
use crate::button as button_style;
use crate::utils::{
    HOVERED_LAYER_OPACITY, PRESSED_LAYER_OPACITY, mix, shadow_from_level, state_layer,
};
use crate::{Theme, tokens};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdaptiveLayout {
    NavigationBar,
    NavigationRail,
}

impl AdaptiveLayout {
    pub fn from_size(width: f32, height: f32) -> Self {
        adaptive_layout(width, height)
    }

    pub fn item_animation_duration_ms(self) -> u16 {
        match self {
            Self::NavigationBar => tokens::component::navigation_bar::ITEM_ANIMATION_DURATION_MS,
            Self::NavigationRail => tokens::component::navigation_rail::ITEM_ANIMATION_DURATION_MS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowWidthClass {
    Compact,
    Medium,
    Expanded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowHeightClass {
    Compact,
    Medium,
    Expanded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowSizeClass {
    pub width: WindowWidthClass,
    pub height: WindowHeightClass,
}

impl WindowSizeClass {
    pub fn from_size(width: f32, height: f32) -> Self {
        Self {
            width: width_class(width),
            height: height_class(height),
        }
    }

    pub fn adaptive_navigation_layout(self) -> AdaptiveLayout {
        if matches!(self.width, WindowWidthClass::Compact)
            || matches!(self.height, WindowHeightClass::Compact)
        {
            AdaptiveLayout::NavigationBar
        } else {
            AdaptiveLayout::NavigationRail
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Selection<Id> {
    selected: Id,
    previous: Option<Id>,
    selected_start: f32,
    previous_start: f32,
    progress: f32,
}

impl<Id: Copy + Eq> Selection<Id> {
    pub fn new(selected: Id) -> Self {
        Self {
            selected,
            previous: None,
            selected_start: 1.0,
            previous_start: 0.0,
            progress: 1.0,
        }
    }

    pub fn transitioning(selected: Id, previous: Id, progress: f32) -> Self {
        Self::transitioning_from(selected, previous, 0.0, 1.0, progress)
    }

    pub fn transitioning_from(
        selected: Id,
        previous: Id,
        selected_start: f32,
        previous_start: f32,
        progress: f32,
    ) -> Self {
        Self {
            selected,
            previous: Some(previous),
            selected_start: selected_start.clamp(0.0, 1.0),
            previous_start: previous_start.clamp(0.0, 1.0),
            progress: progress.clamp(0.0, 1.0),
        }
    }

    pub fn selected(self) -> Id {
        self.selected
    }

    pub fn progress(self, id: Id) -> f32 {
        if id == self.selected {
            lerp(self.selected_start, 1.0, self.progress)
        } else if self.previous.is_some_and(|previous| previous == id) {
            lerp(self.previous_start, 0.0, self.progress)
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NavigationState<Id> {
    selected: Id,
    previous: Option<Id>,
    selected_start: f32,
    previous_start: f32,
    progress: AnimatedScalar,
}

impl<Id: Copy + Eq> NavigationState<Id> {
    pub fn new(selected: Id) -> Self {
        Self {
            selected,
            previous: None,
            selected_start: 1.0,
            previous_start: 0.0,
            progress: AnimatedScalar::new(1.0),
        }
    }

    pub fn selected(&self) -> Id {
        self.selected
    }

    pub fn selection(&self) -> Selection<Id> {
        if let Some(previous) = self.previous {
            Selection::transitioning_from(
                self.selected,
                previous,
                self.selected_start,
                self.previous_start,
                self.progress.value,
            )
        } else {
            Selection::new(self.selected)
        }
    }

    pub fn select(&mut self, selected: Id, now: Instant, layout: AdaptiveLayout) {
        if selected == self.selected {
            return;
        }

        let current = self.selection();
        let previous = self.selected;
        let selected_start = current.progress(selected);
        let previous_start = current.progress(previous);

        self.selected = selected;
        self.previous = Some(previous);
        self.selected_start = selected_start;
        self.previous_start = previous_start;
        self.progress = AnimatedScalar::new(0.0);
        self.progress.set_target(
            1.0,
            now,
            duration_ms(layout.item_animation_duration_ms()),
            tokens::motion::EASING_LEGACY,
        );
    }

    pub fn is_animating(&self) -> bool {
        self.previous.is_some()
    }

    pub fn advance(&mut self, now: Instant) -> bool {
        if !self.progress.advance(now) {
            self.progress.value = 1.0;
            self.previous = None;
            self.selected_start = 1.0;
            self.previous_start = 0.0;
            false
        } else {
            true
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Destination<Id> {
    pub id: Id,
    pub icon: &'static str,
    pub label: &'static str,
}

impl<Id> Destination<Id> {
    pub const fn new(id: Id, icon: &'static str, label: &'static str) -> Self {
        Self { id, icon, label }
    }
}

pub fn width_class(width: f32) -> WindowWidthClass {
    if width < tokens::component::adaptive_navigation::WIDTH_COMPACT_MAX {
        WindowWidthClass::Compact
    } else if width < tokens::component::adaptive_navigation::WIDTH_MEDIUM_MAX {
        WindowWidthClass::Medium
    } else {
        WindowWidthClass::Expanded
    }
}

pub fn height_class(height: f32) -> WindowHeightClass {
    if height < tokens::component::adaptive_navigation::HEIGHT_COMPACT_MAX {
        WindowHeightClass::Compact
    } else if height < tokens::component::adaptive_navigation::HEIGHT_MEDIUM_MAX {
        WindowHeightClass::Medium
    } else {
        WindowHeightClass::Expanded
    }
}

pub fn adaptive_layout(width: f32, height: f32) -> AdaptiveLayout {
    WindowSizeClass::from_size(width, height).adaptive_navigation_layout()
}

pub fn item_animation_duration_ms(layout: AdaptiveLayout) -> u16 {
    layout.item_animation_duration_ms()
}

pub fn navigation_suite<'a, Id, Message, Renderer, F>(
    width: f32,
    height: f32,
    destinations: &'a [Destination<Id>],
    selection: Selection<Id>,
    on_select: F,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Element<'a, Message, Theme, Renderer>
where
    Id: Copy + Eq + 'a,
    Message: Clone + 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
    F: Fn(Id) -> Message + Clone + 'a,
{
    navigation_suite_for_layout(
        adaptive_layout(width, height),
        destinations,
        selection,
        on_select,
        content,
    )
}

pub fn navigation_suite_for_layout<'a, Id, Message, Renderer, F>(
    layout: AdaptiveLayout,
    destinations: &'a [Destination<Id>],
    selection: Selection<Id>,
    on_select: F,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Element<'a, Message, Theme, Renderer>
where
    Id: Copy + Eq + 'a,
    Message: Clone + 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
    F: Fn(Id) -> Message + Clone + 'a,
{
    let content = content.into();

    match layout {
        AdaptiveLayout::NavigationBar => Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(content)
            .push(navigation_bar(destinations, selection, on_select))
            .into(),
        AdaptiveLayout::NavigationRail => Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(navigation_rail(destinations, selection, on_select))
            .push(content)
            .into(),
    }
}

pub fn navigation_bar<'a, Id, Message, Renderer, F>(
    destinations: &'a [Destination<Id>],
    selection: Selection<Id>,
    on_select: F,
) -> Container<'a, Message, Theme, Renderer>
where
    Id: Copy + Eq + 'a,
    Message: Clone + 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
    F: Fn(Id) -> Message + Clone + 'a,
{
    let mut items = Row::new()
        .width(Length::Fill)
        .height(Length::Fixed(
            tokens::component::navigation_bar::CONTAINER_HEIGHT,
        ))
        .spacing(tokens::component::navigation_bar::ITEM_HORIZONTAL_PADDING);

    for destination in destinations {
        items = items.push(
            navigation_bar_item(*destination, selection, on_select.clone())
                .width(Length::FillPortion(1)),
        );
    }

    Container::new(items)
        .width(Length::Fill)
        .height(Length::Fixed(
            tokens::component::navigation_bar::CONTAINER_HEIGHT,
        ))
        .padding(Padding {
            top: 0.0,
            right: tokens::component::navigation_bar::ITEM_HORIZONTAL_PADDING,
            bottom: 0.0,
            left: tokens::component::navigation_bar::ITEM_HORIZONTAL_PADDING,
        })
        .style(navigation_bar_container)
}

pub fn navigation_rail<'a, Id, Message, Renderer, F>(
    destinations: &'a [Destination<Id>],
    selection: Selection<Id>,
    on_select: F,
) -> Container<'a, Message, Theme, Renderer>
where
    Id: Copy + Eq + 'a,
    Message: Clone + 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
    F: Fn(Id) -> Message + Clone + 'a,
{
    let mut items = Column::new()
        .width(Length::Fixed(
            tokens::component::navigation_rail::CONTAINER_WIDTH,
        ))
        .height(Length::Fill)
        .spacing(tokens::component::navigation_rail::VERTICAL_PADDING)
        .align_x(alignment::Horizontal::Center);

    for destination in destinations {
        items = items.push(navigation_rail_item(
            *destination,
            selection,
            on_select.clone(),
        ));
    }

    Container::new(items)
        .width(Length::Fixed(
            tokens::component::navigation_rail::CONTAINER_WIDTH,
        ))
        .height(Length::Fill)
        .padding(Padding {
            top: tokens::component::navigation_rail::VERTICAL_PADDING,
            right: 0.0,
            bottom: tokens::component::navigation_rail::VERTICAL_PADDING,
            left: 0.0,
        })
        .style(navigation_rail_container)
}

pub fn navigation_drawer<'a, Id, Message, Renderer, F>(
    headline: &'static str,
    destinations: &'a [Destination<Id>],
    selection: Selection<Id>,
    on_select: F,
) -> Container<'a, Message, Theme, Renderer>
where
    Id: Copy + Eq + 'a,
    Message: Clone + 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
    F: Fn(Id) -> Message + Clone + 'a,
{
    let headline_scale = tokens::component::navigation_drawer::HEADLINE_TEXT;
    let mut items = Column::new()
        .width(Length::Fixed(
            tokens::component::navigation_drawer::CONTAINER_WIDTH,
        ))
        .height(Length::Fill)
        .spacing(0)
        .push(
            Container::new(type_text(headline, headline_scale).style(headline_text_style))
                .height(Length::Fixed(
                    tokens::component::navigation_drawer::ACTIVE_INDICATOR_HEIGHT,
                ))
                .padding(Padding {
                    top: 0.0,
                    right: tokens::component::navigation_drawer::ITEM_HORIZONTAL_PADDING
                        + tokens::component::navigation_drawer::ITEM_CONTENT_TRAILING_SPACE,
                    bottom: 0.0,
                    left: tokens::component::navigation_drawer::ITEM_HORIZONTAL_PADDING
                        + tokens::component::navigation_drawer::ITEM_CONTENT_LEADING_SPACE,
                })
                .align_y(alignment::Vertical::Center),
        );

    for destination in destinations {
        items = items.push(navigation_drawer_item(
            *destination,
            selection,
            on_select.clone(),
        ));
    }

    Container::new(items)
        .width(Length::Fixed(
            tokens::component::navigation_drawer::CONTAINER_WIDTH,
        ))
        .height(Length::Fill)
        .padding(Padding {
            top: tokens::component::navigation_drawer::ITEM_HORIZONTAL_PADDING,
            right: tokens::component::navigation_drawer::ITEM_HORIZONTAL_PADDING,
            bottom: tokens::component::navigation_drawer::ITEM_HORIZONTAL_PADDING,
            left: tokens::component::navigation_drawer::ITEM_HORIZONTAL_PADDING,
        })
        .style(navigation_drawer_container)
}

fn navigation_bar_item<'a, Id, Message, Renderer, F>(
    destination: Destination<Id>,
    selection: Selection<Id>,
    on_select: F,
) -> Button<'a, Message, Theme, Renderer>
where
    Id: Copy + Eq + 'a,
    Message: Clone + 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
    F: Fn(Id) -> Message + Clone + 'a,
{
    let progress = selection.progress(destination.id);
    let scale = tokens::component::navigation_bar::LABEL_TEXT;
    let message = on_select(destination.id);
    let indicator = indicator_icon_stack(
        destination.icon,
        tokens::component::navigation_bar::ICON_SIZE,
        tokens::component::navigation_bar::ACTIVE_INDICATOR_WIDTH,
        tokens::component::navigation_bar::ACTIVE_INDICATOR_HEIGHT,
        progress,
        false,
        message.clone(),
    );
    let label = type_text(destination.label, scale).style(move |theme| text::Style {
        color: Some(bar_or_rail_label_color(theme, progress)),
    });
    let content = Column::new()
        .width(Length::Fill)
        .spacing(tokens::component::navigation_bar::INDICATOR_TO_LABEL_PADDING)
        .align_x(alignment::Horizontal::Center)
        .push(indicator)
        .push(label);

    Button::new(
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fixed(
                tokens::component::navigation_bar::CONTAINER_HEIGHT,
            ))
            .padding(Padding {
                top: tokens::component::navigation_bar::INDICATOR_VERTICAL_OFFSET,
                right: 0.0,
                bottom: navigation_bar_item_bottom_padding(),
                left: 0.0,
            })
            .align_y(alignment::Vertical::Center),
    )
    .height(Length::Fixed(
        tokens::component::navigation_bar::CONTAINER_HEIGHT,
    ))
    .padding(Padding::ZERO)
    .style(navigation_button)
    .on_press(message)
}

fn navigation_rail_item<'a, Id, Message, Renderer, F>(
    destination: Destination<Id>,
    selection: Selection<Id>,
    on_select: F,
) -> Button<'a, Message, Theme, Renderer>
where
    Id: Copy + Eq + 'a,
    Message: Clone + 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
    F: Fn(Id) -> Message + Clone + 'a,
{
    let progress = selection.progress(destination.id);
    let scale = tokens::component::navigation_rail::LABEL_TEXT;
    let message = on_select(destination.id);
    let indicator = indicator_icon_stack(
        destination.icon,
        tokens::component::navigation_rail::ICON_SIZE,
        tokens::component::navigation_rail::ACTIVE_INDICATOR_WIDTH,
        tokens::component::navigation_rail::ACTIVE_INDICATOR_HEIGHT,
        progress,
        false,
        message.clone(),
    );
    let label = type_text(destination.label, scale).style(move |theme| text::Style {
        color: Some(bar_or_rail_label_color(theme, progress)),
    });
    let content = Column::new()
        .width(Length::Fixed(
            tokens::component::navigation_rail::ITEM_WIDTH,
        ))
        .height(Length::Fixed(
            tokens::component::navigation_rail::ITEM_HEIGHT,
        ))
        .spacing(tokens::component::navigation_rail::ITEM_VERTICAL_PADDING)
        .align_x(alignment::Horizontal::Center)
        .push(indicator)
        .push(label);

    Button::new(content)
        .width(Length::Fixed(
            tokens::component::navigation_rail::ITEM_WIDTH,
        ))
        .height(Length::Fixed(
            tokens::component::navigation_rail::ITEM_HEIGHT,
        ))
        .padding(Padding::ZERO)
        .style(navigation_button)
        .on_press(message)
}

fn navigation_drawer_item<'a, Id, Message, Renderer, F>(
    destination: Destination<Id>,
    selection: Selection<Id>,
    on_select: F,
) -> Button<'a, Message, Theme, Renderer>
where
    Id: Copy + Eq + 'a,
    Message: Clone + 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
    F: Fn(Id) -> Message + Clone + 'a,
{
    let progress = selection.progress(destination.id);
    let scale = tokens::component::navigation_drawer::LABEL_TEXT;
    let message = on_select(destination.id);
    let icon = destination_icon::<Renderer>(
        destination.icon,
        tokens::component::navigation_drawer::ICON_SIZE,
        progress,
        true,
    );
    let label = type_text(destination.label, scale).style(move |theme| text::Style {
        color: Some(drawer_content_color(theme, progress)),
    });
    let content = Row::new()
        .width(Length::Fill)
        .height(Length::Fixed(
            tokens::component::navigation_drawer::ACTIVE_INDICATOR_HEIGHT,
        ))
        .spacing(tokens::component::navigation_drawer::ICON_LABEL_SPACE)
        .align_y(alignment::Vertical::Center)
        .push(icon)
        .push(label);
    let content = Container::new(content)
        .width(Length::Fixed(
            tokens::component::navigation_drawer::ACTIVE_INDICATOR_WIDTH,
        ))
        .height(Length::Fixed(
            tokens::component::navigation_drawer::ACTIVE_INDICATOR_HEIGHT,
        ))
        .padding(Padding {
            top: 0.0,
            right: tokens::component::navigation_drawer::ITEM_CONTENT_TRAILING_SPACE,
            bottom: 0.0,
            left: tokens::component::navigation_drawer::ITEM_CONTENT_LEADING_SPACE,
        })
        .align_y(alignment::Vertical::Center);
    let indicator = Stack::new()
        .width(Length::Fixed(
            tokens::component::navigation_drawer::ACTIVE_INDICATOR_WIDTH,
        ))
        .height(Length::Fixed(
            tokens::component::navigation_drawer::ACTIVE_INDICATOR_HEIGHT,
        ))
        .push(
            Space::new()
                .width(Length::Fixed(
                    tokens::component::navigation_drawer::ACTIVE_INDICATOR_WIDTH,
                ))
                .height(Length::Fixed(
                    tokens::component::navigation_drawer::ACTIVE_INDICATOR_HEIGHT,
                )),
        )
        .push(indicator_layer(
            tokens::component::navigation_drawer::ACTIVE_INDICATOR_WIDTH,
            tokens::component::navigation_drawer::ACTIVE_INDICATOR_HEIGHT,
            progress,
        ))
        .push(indicator_state_layer(
            tokens::component::navigation_drawer::ACTIVE_INDICATOR_WIDTH,
            tokens::component::navigation_drawer::ACTIVE_INDICATOR_HEIGHT,
            NavigationStateLayer::Drawer { progress },
            message.clone(),
        ))
        .push(content);

    Button::new(indicator)
        .width(Length::Fixed(
            tokens::component::navigation_drawer::ACTIVE_INDICATOR_WIDTH,
        ))
        .height(Length::Fixed(
            tokens::component::navigation_drawer::ACTIVE_INDICATOR_HEIGHT,
        ))
        .padding(Padding::ZERO)
        .style(navigation_button)
        .on_press(message)
}

fn indicator_icon_stack<'a, Message, Renderer>(
    icon: &'static str,
    icon_size: f32,
    indicator_width: f32,
    indicator_height: f32,
    progress: f32,
    drawer: bool,
    on_press: Message,
) -> Stack<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    Stack::new()
        .width(Length::Fixed(indicator_width))
        .height(Length::Fixed(indicator_height))
        .push(
            Space::new()
                .width(Length::Fixed(indicator_width))
                .height(Length::Fixed(indicator_height)),
        )
        .push(indicator_layer(indicator_width, indicator_height, progress))
        .push(indicator_state_layer(
            indicator_width,
            indicator_height,
            NavigationStateLayer::BarOrRail,
            on_press,
        ))
        .push(
            Container::new(destination_icon::<Renderer>(
                icon, icon_size, progress, drawer,
            ))
            .width(Length::Fixed(indicator_width))
            .height(Length::Fixed(indicator_height))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center),
        )
}

fn indicator_state_layer<'a, Message, Renderer>(
    target_width: f32,
    height: f32,
    layer: NavigationStateLayer,
    on_press: Message,
) -> Button<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    Button::new(
        Space::new()
            .width(Length::Fixed(target_width))
            .height(Length::Fixed(height)),
    )
    .width(Length::Fixed(target_width))
    .height(Length::Fixed(height))
    .padding(Padding::ZERO)
    .style(move |theme, status| indicator_state_layer_style(theme, status, layer))
    .on_press(on_press)
}

fn indicator_layer<'a, Message, Renderer>(
    target_width: f32,
    height: f32,
    progress: f32,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    let indicator = Container::new(Space::new())
        .width(Length::Fixed(animated_indicator_width(
            target_width,
            progress,
        )))
        .height(Length::Fixed(height))
        .style(active_indicator);

    Container::new(indicator)
        .width(Length::Fixed(target_width))
        .height(Length::Fixed(height))
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
}

fn indicator_state_layer_style(
    theme: &Theme,
    status: button::Status,
    layer: NavigationStateLayer,
) -> button::Style {
    let layer_color = navigation_state_layer_color(theme, layer);
    let opacity = match status {
        button::Status::Hovered => HOVERED_LAYER_OPACITY,
        button::Status::Pressed => PRESSED_LAYER_OPACITY,
        button::Status::Active | button::Status::Disabled => 0.0,
    };

    button::Style {
        background: (opacity > 0.0).then_some(Background::Color(state_layer(layer_color, opacity))),
        text_color: layer_color,
        border: border::rounded(tokens::shape::CORNER_FULL),
        snap: cfg!(feature = "crisp"),
        ..button::Style::default()
    }
}

#[derive(Debug, Clone, Copy)]
enum NavigationStateLayer {
    BarOrRail,
    Drawer { progress: f32 },
}

fn animated_indicator_width(target_width: f32, progress: f32) -> f32 {
    // AndroidX Material3 measures the selected indicator width from animation progress.
    target_width * progress.clamp(0.0, 1.0)
}

fn navigation_bar_item_bottom_padding() -> f32 {
    let label = tokens::component::navigation_bar::LABEL_TEXT;

    (tokens::component::navigation_bar::CONTAINER_HEIGHT
        - tokens::component::navigation_bar::INDICATOR_VERTICAL_OFFSET
        - tokens::component::navigation_bar::ACTIVE_INDICATOR_HEIGHT
        - tokens::component::navigation_bar::INDICATOR_TO_LABEL_PADDING
        - label.line_height)
        .max(0.0)
}

fn destination_icon<'a, Renderer>(
    icon: &'static str,
    size: f32,
    progress: f32,
    drawer: bool,
) -> Text<'a, Theme, Renderer>
where
    Renderer: core_text::Renderer + 'a,
{
    Text::new(icon)
        .size(size)
        .line_height(LineHeight::Absolute(size.into()))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .center()
        .style(move |theme| {
            let color = if drawer {
                drawer_content_color(theme, progress)
            } else {
                bar_or_rail_icon_color(theme, progress)
            };

            text::Style { color: Some(color) }
        })
}

fn type_text<'a, Renderer>(
    content: &'static str,
    scale: tokens::typography::TypeScale,
) -> Text<'a, Theme, Renderer>
where
    Renderer: core_text::Renderer + 'a,
{
    Text::new(content)
        .size(scale.size)
        .line_height(LineHeight::Absolute(scale.line_height.into()))
}

fn navigation_button(theme: &Theme, status: button::Status) -> button::Style {
    let mut style = button_style::text(theme, status);
    style.text_color = theme.colors().surface.text;
    style.background = None;

    style
}

fn navigation_bar_container(theme: &Theme) -> iced_widget::container::Style {
    let colors = theme.colors();

    iced_widget::container::Style {
        background: Some(Background::Color(colors.surface.color)),
        text_color: Some(colors.surface.text),
        border: border::rounded(tokens::shape::CORNER_NONE),
        shadow: shadow_from_level(
            tokens::component::navigation_bar::CONTAINER_ELEVATION_LEVEL,
            colors.shadow,
        ),
        ..iced_widget::container::Style::default()
    }
}

fn navigation_rail_container(theme: &Theme) -> iced_widget::container::Style {
    let colors = theme.colors();

    iced_widget::container::Style {
        background: Some(Background::Color(colors.surface.color)),
        text_color: Some(colors.surface.text),
        border: border::rounded(tokens::shape::CORNER_NONE),
        shadow: shadow_from_level(
            tokens::component::navigation_rail::CONTAINER_ELEVATION_LEVEL,
            colors.shadow,
        ),
        ..iced_widget::container::Style::default()
    }
}

fn navigation_drawer_container(theme: &Theme) -> iced_widget::container::Style {
    let colors = theme.colors();

    iced_widget::container::Style {
        background: Some(Background::Color(colors.surface.color)),
        text_color: Some(colors.surface.text),
        border: border::rounded(tokens::shape::CORNER_LARGE),
        shadow: shadow_from_level(
            tokens::component::navigation_drawer::STANDARD_CONTAINER_ELEVATION_LEVEL,
            colors.shadow,
        ),
        ..iced_widget::container::Style::default()
    }
}

fn active_indicator(theme: &Theme) -> iced_widget::container::Style {
    iced_widget::container::Style {
        background: Some(Background::Color(theme.colors().secondary.container)),
        text_color: Some(theme.colors().secondary.container_text),
        border: border::rounded(tokens::shape::CORNER_FULL),
        ..iced_widget::container::Style::default()
    }
}

fn headline_text_style(theme: &Theme) -> text::Style {
    text::Style {
        color: Some(theme.colors().surface.text_variant),
    }
}

fn bar_or_rail_icon_color(theme: &Theme, progress: f32) -> Color {
    let colors = theme.colors();

    mix(
        colors.surface.text_variant,
        colors.secondary.container_text,
        progress,
    )
}

fn bar_or_rail_label_color(theme: &Theme, progress: f32) -> Color {
    let colors = theme.colors();

    mix(colors.surface.text_variant, colors.surface.text, progress)
}

fn drawer_content_color(theme: &Theme, progress: f32) -> Color {
    let colors = theme.colors();

    mix(
        colors.surface.text_variant,
        colors.secondary.container_text,
        progress,
    )
}

fn navigation_state_layer_color(theme: &Theme, layer: NavigationStateLayer) -> Color {
    let colors = theme.colors();

    match layer {
        // AndroidX Material3 applies the selectable interaction to the whole item, but remaps the
        // ripple onto the active-indicator pill. Its ripple color comes from the navigation
        // container content color, not from the active icon color.
        NavigationStateLayer::BarOrRail => colors.surface.text,
        NavigationStateLayer::Drawer { progress } => mix(
            colors.surface.text,
            colors.secondary.container_text,
            progress,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Page {
        One,
        Two,
    }

    #[test]
    fn window_size_classes_use_material_breakpoints() {
        assert_eq!(width_class(599.0), WindowWidthClass::Compact);
        assert_eq!(width_class(600.0), WindowWidthClass::Medium);
        assert_eq!(width_class(839.0), WindowWidthClass::Medium);
        assert_eq!(width_class(840.0), WindowWidthClass::Expanded);

        assert_eq!(height_class(479.0), WindowHeightClass::Compact);
        assert_eq!(height_class(480.0), WindowHeightClass::Medium);
        assert_eq!(height_class(900.0), WindowHeightClass::Expanded);
    }

    #[test]
    fn adaptive_layout_matches_navigation_suite_default() {
        assert_eq!(adaptive_layout(480.0, 900.0), AdaptiveLayout::NavigationBar);
        assert_eq!(adaptive_layout(700.0, 420.0), AdaptiveLayout::NavigationBar);
        assert_eq!(
            adaptive_layout(700.0, 700.0),
            AdaptiveLayout::NavigationRail
        );
        assert_eq!(
            adaptive_layout(1080.0, 980.0),
            AdaptiveLayout::NavigationRail
        );
        assert_eq!(
            AdaptiveLayout::from_size(1080.0, 980.0),
            AdaptiveLayout::NavigationRail
        );
        assert_eq!(
            WindowSizeClass::from_size(420.0, 900.0).adaptive_navigation_layout(),
            AdaptiveLayout::NavigationBar
        );
        assert_eq!(
            item_animation_duration_ms(AdaptiveLayout::NavigationBar),
            tokens::component::navigation_bar::ITEM_ANIMATION_DURATION_MS
        );
        assert_eq!(
            item_animation_duration_ms(AdaptiveLayout::NavigationRail),
            tokens::component::navigation_rail::ITEM_ANIMATION_DURATION_MS
        );
    }

    #[test]
    fn selection_interpolates_previous_and_selected_destination() {
        let selection = Selection::transitioning(Page::Two, Page::One, 0.25);

        assert_eq!(selection.progress(Page::Two), 0.25);
        assert_eq!(selection.progress(Page::One), 0.75);
        assert_eq!(Selection::new(Page::One).progress(Page::One), 1.0);
    }

    #[test]
    fn navigation_state_owns_selection_animation_progress() {
        let start = Instant::now();
        let mut state = NavigationState::new(Page::One);

        state.select(Page::Two, start, AdaptiveLayout::NavigationRail);

        assert_eq!(state.selected(), Page::Two);
        assert!(state.is_animating());
        assert_eq!(state.selection().progress(Page::Two), 0.0);
        assert_eq!(state.selection().progress(Page::One), 1.0);

        let still_animating = state.advance(
            start + duration_ms(tokens::component::navigation_rail::ITEM_ANIMATION_DURATION_MS / 2),
        );

        assert!(still_animating);
        assert!(state.selection().progress(Page::Two) > 0.0);
        assert!(state.selection().progress(Page::One) < 1.0);

        let finished = state.advance(
            start + duration_ms(tokens::component::navigation_rail::ITEM_ANIMATION_DURATION_MS),
        );

        assert!(!finished);
        assert!(!state.is_animating());
        assert_eq!(state.selection().progress(Page::Two), 1.0);
        assert_eq!(state.selection().progress(Page::One), 0.0);
    }

    #[test]
    fn navigation_state_preserves_progress_when_transition_is_interrupted() {
        let start = Instant::now();
        let mut state = NavigationState::new(Page::One);

        state.select(Page::Two, start, AdaptiveLayout::NavigationRail);
        let _ = state.advance(
            start + duration_ms(tokens::component::navigation_rail::ITEM_ANIMATION_DURATION_MS / 2),
        );

        let two_progress = state.selection().progress(Page::Two);

        state.select(
            Page::One,
            start + duration_ms(tokens::component::navigation_rail::ITEM_ANIMATION_DURATION_MS / 2),
            AdaptiveLayout::NavigationRail,
        );

        assert_eq!(state.selected(), Page::One);
        assert_eq!(state.selection().progress(Page::Two), two_progress);
        assert!(state.selection().progress(Page::One) > 0.0);
    }

    #[test]
    fn active_indicator_width_follows_selection_progress() {
        let target = tokens::component::navigation_bar::ACTIVE_INDICATOR_WIDTH;

        assert_eq!(animated_indicator_width(target, -1.0), 0.0);
        assert_eq!(animated_indicator_width(target, 0.0), 0.0);
        assert_eq!(animated_indicator_width(target, 0.5), target / 2.0);
        assert_eq!(animated_indicator_width(target, 1.0), target);
        assert_eq!(animated_indicator_width(target, 2.0), target);
    }

    #[test]
    fn navigation_bar_item_geometry_matches_material_vertical_offsets() {
        assert_eq!(navigation_bar_item_bottom_padding(), 16.0);
    }

    #[test]
    fn indicator_state_layer_uses_material_state_opacity_on_pill_only() {
        let theme = Theme::Light;

        let active = indicator_state_layer_style(
            &theme,
            button::Status::Active,
            NavigationStateLayer::BarOrRail,
        );
        assert_eq!(active.background, None);

        let inactive_hover = indicator_state_layer_style(
            &theme,
            button::Status::Hovered,
            NavigationStateLayer::BarOrRail,
        );
        assert_eq!(
            inactive_hover.background,
            Some(Background::Color(state_layer(
                theme.colors().surface.text,
                HOVERED_LAYER_OPACITY
            )))
        );

        let selected_pressed = indicator_state_layer_style(
            &theme,
            button::Status::Pressed,
            NavigationStateLayer::BarOrRail,
        );
        assert_eq!(
            selected_pressed.background,
            Some(Background::Color(state_layer(
                theme.colors().surface.text,
                PRESSED_LAYER_OPACITY
            )))
        );

        let drawer_selected_pressed = indicator_state_layer_style(
            &theme,
            button::Status::Pressed,
            NavigationStateLayer::Drawer { progress: 1.0 },
        );
        assert_eq!(
            drawer_selected_pressed.background,
            Some(Background::Color(state_layer(
                theme.colors().secondary.container_text,
                PRESSED_LAYER_OPACITY
            )))
        );
        assert_eq!(
            selected_pressed.border.radius.top_left,
            tokens::shape::CORNER_FULL
        );
    }
}
