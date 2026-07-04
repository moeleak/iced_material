//! Material select widget.
//!
//! iced's built-in pick list opens the menu on whichever side of the field has
//! more vertical space. Material selects should prefer opening below the field
//! when the menu fits there, so this widget keeps the iced pick list behavior
//! while adjusting the overlay anchor before handing off to iced's menu overlay.

use std::borrow::Borrow;
use std::fmt;

use iced_widget::core::text::paragraph;
use iced_widget::core::text::{self, Text};
use iced_widget::core::time::Instant;
use iced_widget::core::widget::tree::{self, Tree};
use iced_widget::core::{
    Clipboard, Color, Element, Event, Layout, Length, Padding, Pixels, Point, Rectangle, Shell,
    Size, Vector, Widget, alignment, keyboard, layout, mouse, overlay, renderer, touch, window,
};
use iced_widget::overlay::menu;
use iced_widget::pick_list::{self as iced_select, Handle, Icon, Status};

use super::{absolute_line_height, menu_overlay};
use crate::style::{menu as menu_style, select as select_style};
use crate::{Theme, tokens};

const MAX_VISIBLE_OPTIONS: usize = 5;
const DIRECTION_EPSILON: f32 = 0.5;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct MenuAnchor {
    pub(crate) position: Point,
    pub(crate) target_height: f32,
}

/// Creates a Material outlined select field.
pub fn outlined<'a, T, L, V, Message, Renderer>(
    options: L,
    selected: Option<V>,
    on_select: impl Fn(T) -> Message + 'a,
) -> Select<'a, T, L, V, Message, Renderer>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone + 'a,
    Renderer: text::Renderer + 'a,
{
    Select::new(options, selected, on_select)
        .padding(Padding {
            top: tokens::component::text_field::TOP_SPACE,
            right: tokens::component::text_field::TRAILING_SPACE,
            bottom: tokens::component::text_field::BOTTOM_SPACE,
            left: tokens::component::text_field::LEADING_SPACE,
        })
        .option_padding(menu_option_padding())
        .text_size(tokens::component::text_field::INPUT_TEXT_SIZE)
        .text_line_height(absolute_line_height(
            tokens::component::text_field::INPUT_TEXT_LINE_HEIGHT,
        ))
        .width(Length::Fill)
        .style(select_style::default)
        .menu_style(menu_style::outlined_select)
}

/// A Material select field.
pub struct Select<'a, T, L, V, Message, Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Renderer: text::Renderer,
{
    on_select: Box<dyn Fn(T) -> Message + 'a>,
    on_open: Option<Message>,
    on_close: Option<Message>,
    options: L,
    label: Option<String>,
    placeholder: Option<String>,
    selected: Option<V>,
    width: Length,
    field_padding: Padding,
    option_padding: Padding,
    text_size: Option<Pixels>,
    text_line_height: text::LineHeight,
    text_shaping: text::Shaping,
    font: Option<Renderer::Font>,
    handle: Handle<Renderer::Font>,
    class: <Theme as iced_select::Catalog>::Class<'a>,
    menu_class: <Theme as menu::Catalog>::Class<'a>,
    last_status: Option<Status>,
    menu_height: Length,
}

impl<T, L, V, Message, Renderer> fmt::Debug for Select<'_, T, L, V, Message, Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]>,
    V: Borrow<T>,
    Renderer: text::Renderer,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Select").finish_non_exhaustive()
    }
}

impl<'a, T, L, V, Message, Renderer> Select<'a, T, L, V, Message, Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone,
    Renderer: text::Renderer,
{
    /// Creates a new [`Select`] with the given list of options, selected value,
    /// and message to produce when an option is selected.
    pub fn new(options: L, selected: Option<V>, on_select: impl Fn(T) -> Message + 'a) -> Self {
        let option_count = options.borrow().len();

        Self {
            on_select: Box::new(on_select),
            on_open: None,
            on_close: None,
            options,
            label: None,
            placeholder: None,
            selected,
            width: Length::Shrink,
            field_padding: iced_widget::button::DEFAULT_PADDING,
            option_padding: menu_option_padding(),
            text_size: None,
            text_line_height: text::LineHeight::default(),
            text_shaping: text::Shaping::default(),
            font: None,
            handle: Handle::default(),
            class: <Theme as iced_select::Catalog>::default(),
            menu_class: <Theme as iced_select::Catalog>::default_menu(),
            last_status: None,
            menu_height: material_menu_height(option_count),
        }
    }

    /// Sets the placeholder of the select.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the floating label of the select.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the width of the select.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the menu.
    pub fn menu_height(mut self, menu_height: impl Into<Length>) -> Self {
        self.menu_height = menu_height.into();
        self
    }

    /// Sets the padding of the select field.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.field_padding = padding.into();
        self
    }

    /// Sets the padding of each menu option.
    pub fn option_padding(mut self, padding: impl Into<Padding>) -> Self {
        self.option_padding = padding.into();
        self
    }

    /// Sets the text size of the select and its menu items.
    pub fn text_size(mut self, size: impl Into<Pixels>) -> Self {
        self.text_size = Some(size.into());
        self
    }

    /// Sets the text line height of the select and its menu items.
    pub fn text_line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.text_line_height = line_height.into();
        self
    }

    /// Sets the text shaping strategy.
    pub fn text_shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }

    /// Sets the font.
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the trailing handle.
    pub fn handle(mut self, handle: Handle<Renderer::Font>) -> Self {
        self.handle = handle;
        self
    }

    /// Sets the message produced when the menu is opened.
    pub fn on_open(mut self, on_open: Message) -> Self {
        self.on_open = Some(on_open);
        self
    }

    /// Sets the message produced when the menu is closed.
    pub fn on_close(mut self, on_close: Message) -> Self {
        self.on_close = Some(on_close);
        self
    }

    /// Sets the style of the select.
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> iced_select::Style + 'a) -> Self
    where
        <Theme as iced_select::Catalog>::Class<'a>: From<iced_select::StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as iced_select::StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style of the menu.
    pub fn menu_style(mut self, style: impl Fn(&Theme) -> menu::Style + 'a) -> Self
    where
        <Theme as menu::Catalog>::Class<'a>: From<menu::StyleFn<'a, Theme>>,
    {
        self.menu_class = (Box::new(style) as menu::StyleFn<'a, Theme>).into();
        self
    }

    fn intrinsic_menu_height(&self, renderer: &Renderer) -> f32 {
        let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());
        let option_height =
            f32::from(self.text_line_height.to_absolute(text_size)) + self.option_padding.y();

        option_height * self.options.borrow().len() as f32
    }
}

impl<'a, T, L, V, Message, Renderer> Widget<Message, Theme, Renderer>
    for Select<'a, T, L, V, Message, Renderer>
where
    T: Clone + ToString + PartialEq + 'a,
    L: Borrow<[T]>,
    V: Borrow<T>,
    Message: Clone + 'a,
    Renderer: text::Renderer + 'a,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::<Renderer::Paragraph>::new())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

        let font = self.font.unwrap_or_else(|| renderer.default_font());
        let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());
        let options = self.options.borrow();

        state.options.resize_with(options.len(), Default::default);

        let option_text = Text {
            content: "",
            bounds: Size::new(
                f32::INFINITY,
                self.text_line_height.to_absolute(text_size).into(),
            ),
            size: text_size,
            line_height: self.text_line_height,
            font,
            align_x: text::Alignment::Default,
            align_y: alignment::Vertical::Center,
            shaping: self.text_shaping,
            wrapping: text::Wrapping::default(),
        };

        for (option, paragraph) in options.iter().zip(state.options.iter_mut()) {
            let label = option.to_string();

            let _ = paragraph.update(Text {
                content: &label,
                ..option_text
            });
        }

        if let Some(placeholder) = &self.placeholder {
            let _ = state.placeholder.update(Text {
                content: placeholder,
                ..option_text
            });
        }

        if let Some(label) = &self.label {
            let _ = state.label.update(Text {
                content: label,
                size: Pixels(tokens::component::text_field::LABEL_TEXT_POPULATED_SIZE),
                line_height: text::LineHeight::Absolute(Pixels(
                    tokens::component::text_field::LABEL_TEXT_POPULATED_LINE_HEIGHT,
                )),
                ..option_text
            });
        }

        let max_width = match self.width {
            Length::Shrink => {
                let labels_width = state.options.iter().fold(0.0, |width, paragraph| {
                    f32::max(width, paragraph.min_width())
                });

                labels_width
                    .max(
                        self.placeholder
                            .as_ref()
                            .map(|_| state.placeholder.min_width())
                            .unwrap_or(0.0),
                    )
                    .max(
                        self.label
                            .as_ref()
                            .map(|_| state.label.min_width())
                            .unwrap_or(0.0),
                    )
            }
            _ => 0.0,
        };

        let size = {
            let intrinsic = Size::new(
                max_width + text_size.0 + self.field_padding.left,
                f32::from(self.text_line_height.to_absolute(text_size)),
            );

            limits
                .width(self.width)
                .shrink(self.field_padding)
                .resolve(self.width, Length::Shrink, intrinsic)
                .expand(self.field_padding)
        };

        layout::Node::new(size)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if state.is_open {
                    state.is_open = false;

                    if let Some(on_close) = &self.on_close {
                        shell.publish(on_close.clone());
                    }

                    shell.capture_event();
                } else if cursor.is_over(layout.bounds()) {
                    let selected = self.selected.as_ref().map(Borrow::borrow);

                    state.is_open = true;
                    state.hovered_option = self
                        .options
                        .borrow()
                        .iter()
                        .position(|option| Some(option) == selected);
                    state
                        .menu
                        .start_open(self.options.borrow().len(), Instant::now());

                    if let Some(on_open) = &self.on_open {
                        shell.publish(on_open.clone());
                    }

                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled {
                delta: mouse::ScrollDelta::Lines { y, .. },
            }) => {
                if state.keyboard_modifiers.command()
                    && cursor.is_over(layout.bounds())
                    && !state.is_open
                {
                    let options = self.options.borrow();
                    let selected = self.selected.as_ref().map(Borrow::borrow);

                    let next_option = if *y < 0.0 {
                        if let Some(selected) = selected {
                            find_next(selected, options.iter())
                        } else {
                            options.first()
                        }
                    } else if *y > 0.0 {
                        if let Some(selected) = selected {
                            find_next(selected, options.iter().rev())
                        } else {
                            options.last()
                        }
                    } else {
                        None
                    };

                    if let Some(next_option) = next_option {
                        shell.publish((self.on_select)(next_option.clone()));
                    }

                    shell.capture_event();
                }
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                state.keyboard_modifiers = *modifiers;
            }
            _ => {}
        };

        let status = {
            let is_hovered = cursor.is_over(layout.bounds());

            if state.is_open {
                Status::Opened { is_hovered }
            } else if is_hovered {
                Status::Hovered
            } else {
                Status::Active
            }
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.last_status = Some(status);
        } else if self
            .last_status
            .is_some_and(|last_status| last_status != status)
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let font = self.font.unwrap_or_else(|| renderer.default_font());
        let selected = self.selected.as_ref().map(Borrow::borrow);
        let state = tree.state.downcast_ref::<State<Renderer::Paragraph>>();

        let bounds = layout.bounds();

        let style = <Theme as iced_select::Catalog>::style(
            theme,
            &self.class,
            self.last_status.unwrap_or(Status::Active),
        );

        let label_width = self
            .label
            .as_ref()
            .map(|_| state.label.min_width())
            .unwrap_or(0.0);

        draw_select_outline(
            renderer,
            bounds,
            style.border,
            style.background,
            label_width,
            theme.colors().surface.container.high,
        );

        let handle = match &self.handle {
            Handle::Arrow { size } => Some((
                Renderer::ICON_FONT,
                Renderer::ARROW_DOWN_ICON,
                *size,
                text::LineHeight::default(),
                text::Shaping::Basic,
            )),
            Handle::Static(Icon {
                font,
                code_point,
                size,
                line_height,
                shaping,
            }) => Some((*font, *code_point, *size, *line_height, *shaping)),
            Handle::Dynamic { open, closed } => {
                if state.is_open {
                    Some((
                        open.font,
                        open.code_point,
                        open.size,
                        open.line_height,
                        open.shaping,
                    ))
                } else {
                    Some((
                        closed.font,
                        closed.code_point,
                        closed.size,
                        closed.line_height,
                        closed.shaping,
                    ))
                }
            }
            Handle::None => None,
        };

        if let Some((font, code_point, size, line_height, shaping)) = handle {
            let size = size.unwrap_or_else(|| renderer.default_size());

            renderer.fill_text(
                Text {
                    content: code_point.to_string(),
                    size,
                    line_height,
                    font,
                    bounds: Size::new(bounds.width, f32::from(line_height.to_absolute(size))),
                    align_x: text::Alignment::Right,
                    align_y: alignment::Vertical::Center,
                    shaping,
                    wrapping: text::Wrapping::default(),
                },
                Point::new(
                    bounds.x + bounds.width - self.field_padding.right,
                    bounds.center_y(),
                ),
                style.handle_color,
                *viewport,
            );
        }

        let label = selected.map(ToString::to_string);

        if let Some(label) = label.or_else(|| self.placeholder.clone()) {
            let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());

            renderer.fill_text(
                Text {
                    content: label,
                    size: text_size,
                    line_height: self.text_line_height,
                    font,
                    bounds: Size::new(
                        bounds.width - self.field_padding.x(),
                        f32::from(self.text_line_height.to_absolute(text_size)),
                    ),
                    align_x: text::Alignment::Default,
                    align_y: alignment::Vertical::Center,
                    shaping: self.text_shaping,
                    wrapping: text::Wrapping::default(),
                },
                Point::new(bounds.x + self.field_padding.left, bounds.center_y()),
                if selected.is_some() {
                    style.text_color
                } else {
                    style.placeholder_color
                },
                *viewport,
            );
        }

        if let Some(label) = &self.label {
            let label_size = Pixels(tokens::component::text_field::LABEL_TEXT_POPULATED_SIZE);
            let label_line_height = text::LineHeight::Absolute(Pixels(
                tokens::component::text_field::LABEL_TEXT_POPULATED_LINE_HEIGHT,
            ));

            renderer.fill_text(
                Text {
                    content: label.clone(),
                    size: label_size,
                    line_height: label_line_height,
                    font,
                    bounds: Size::new(
                        label_width,
                        f32::from(label_line_height.to_absolute(label_size)),
                    ),
                    align_x: text::Alignment::Default,
                    align_y: alignment::Vertical::Center,
                    shaping: self.text_shaping,
                    wrapping: text::Wrapping::None,
                },
                Point::new(
                    bounds.x + tokens::component::text_field::LEADING_SPACE,
                    bounds.y,
                ),
                select_label_color(theme, self.last_status.unwrap_or(Status::Active)),
                *viewport,
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();
        let font = self.font.unwrap_or_else(|| renderer.default_font());

        if state.is_open {
            let bounds = layout.bounds();
            let on_select = &self.on_select;

            let mut menu = menu_overlay::Menu::new(
                &mut state.menu,
                self.options.borrow(),
                &mut state.hovered_option,
                |option| {
                    state.is_open = false;

                    (on_select)(option)
                },
                None,
                &self.menu_class,
            )
            .width(bounds.width)
            .padding(self.option_padding)
            .font(font)
            .text_shaping(self.text_shaping);

            if let Some(text_size) = self.text_size {
                menu = menu.text_size(text_size);
            }

            let anchor = prefer_down_when_menu_fits(
                layout.position() + translation,
                *viewport,
                bounds.height,
                resolved_menu_height(
                    self.menu_height,
                    self.intrinsic_menu_height(renderer),
                    viewport.height,
                ),
            );

            Some(menu.overlay(
                anchor.position,
                *viewport,
                anchor.target_height,
                self.menu_height,
            ))
        } else {
            None
        }
    }
}

impl<'a, T, L, V, Message, Renderer> From<Select<'a, T, L, V, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: Clone + ToString + PartialEq + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(select: Select<'a, T, L, V, Message, Renderer>) -> Self {
        Self::new(select)
    }
}

#[derive(Debug)]
struct State<P: text::Paragraph> {
    menu: menu_overlay::State,
    keyboard_modifiers: keyboard::Modifiers,
    is_open: bool,
    hovered_option: Option<usize>,
    options: Vec<paragraph::Plain<P>>,
    placeholder: paragraph::Plain<P>,
    label: paragraph::Plain<P>,
}

impl<P: text::Paragraph> State<P> {
    fn new() -> Self {
        Self {
            menu: menu_overlay::State::default(),
            keyboard_modifiers: keyboard::Modifiers::default(),
            is_open: bool::default(),
            hovered_option: Option::default(),
            options: Vec::new(),
            placeholder: paragraph::Plain::default(),
            label: paragraph::Plain::default(),
        }
    }
}

fn draw_select_outline<Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    border: iced_widget::core::Border,
    background: iced_widget::core::Background,
    label_width: f32,
    notch_background: Color,
) where
    Renderer: iced_widget::core::Renderer,
{
    renderer.fill_quad(
        renderer::Quad {
            bounds,
            border,
            ..renderer::Quad::default()
        },
        background,
    );

    if label_width <= 0.0 || border.width <= 0.0 {
        return;
    }

    let notch_width = label_width + tokens::component::text_field::OUTLINE_LABEL_PADDING * 2.0;
    let notch_x = bounds.x + tokens::component::text_field::LEADING_SPACE
        - tokens::component::text_field::OUTLINE_LABEL_PADDING;

    renderer.fill_quad(
        renderer::Quad {
            bounds: Rectangle {
                x: notch_x,
                y: bounds.y,
                width: notch_width.min((bounds.x + bounds.width - notch_x).max(0.0)),
                height: border.width.ceil() + 1.0,
            },
            ..renderer::Quad::default()
        },
        notch_background,
    );
}

fn select_label_color(theme: &Theme, status: Status) -> Color {
    let colors = theme.colors();

    match status {
        Status::Opened { .. } => colors.primary.color,
        Status::Hovered => colors.surface.text,
        Status::Active => colors.surface.text_variant,
    }
}

impl<P: text::Paragraph> Default for State<P> {
    fn default() -> Self {
        Self::new()
    }
}

fn find_next<'a, T: PartialEq>(
    selected: &'a T,
    mut options: impl Iterator<Item = &'a T>,
) -> Option<&'a T> {
    let _ = options.find(|&option| option == selected);

    options.next()
}

pub(crate) fn menu_option_padding() -> Padding {
    let vertical = (tokens::component::select::MENU_LIST_ITEM_CONTAINER_HEIGHT
        - tokens::component::text_field::INPUT_TEXT_LINE_HEIGHT)
        / 2.0;

    Padding {
        top: vertical,
        right: tokens::component::text_field::TRAILING_SPACE,
        bottom: vertical,
        left: tokens::component::text_field::LEADING_SPACE,
    }
}

pub(crate) fn material_menu_height(option_count: usize) -> Length {
    let visible_options = option_count.clamp(1, MAX_VISIBLE_OPTIONS) as f32;

    Length::Fixed(tokens::component::select::MENU_LIST_ITEM_CONTAINER_HEIGHT * visible_options)
}

pub(crate) fn resolved_menu_height(
    menu_height: Length,
    intrinsic_height: f32,
    viewport_height: f32,
) -> f32 {
    match menu_height {
        Length::Fixed(height) => height,
        Length::Shrink => intrinsic_height,
        Length::Fill | Length::FillPortion(_) => viewport_height,
    }
}

pub(crate) fn prefer_down_when_menu_fits(
    position: Point,
    viewport: Rectangle,
    target_height: f32,
    menu_height: f32,
) -> MenuAnchor {
    let down_anchor_y = position.y + target_height;
    let space_below = viewport.height - down_anchor_y;

    if space_below < menu_height {
        return MenuAnchor {
            position,
            target_height,
        };
    }

    if space_below > position.y {
        return MenuAnchor {
            position,
            target_height,
        };
    }

    let adjusted_y = position.y.min((space_below - DIRECTION_EPSILON).max(0.0));

    MenuAnchor {
        position: Point::new(position.x, adjusted_y),
        target_height: down_anchor_y - adjusted_y,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_menu_height_uses_five_visible_options_max() {
        assert_eq!(
            material_menu_height(3),
            Length::Fixed(tokens::component::select::MENU_LIST_ITEM_CONTAINER_HEIGHT * 3.0)
        );
        assert_eq!(
            material_menu_height(8),
            Length::Fixed(
                tokens::component::select::MENU_LIST_ITEM_CONTAINER_HEIGHT
                    * MAX_VISIBLE_OPTIONS as f32
            )
        );
    }

    #[test]
    fn material_option_padding_produces_m3_menu_item_height() {
        let padding = menu_option_padding();

        assert_eq!(
            tokens::component::text_field::INPUT_TEXT_LINE_HEIGHT + padding.y(),
            tokens::component::select::MENU_LIST_ITEM_CONTAINER_HEIGHT
        );
    }

    #[test]
    fn select_prefers_down_when_menu_fits_below() {
        let position = Point::new(0.0, 500.0);
        let target_height = 56.0;
        let viewport = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 940.0,
        };

        let anchor = prefer_down_when_menu_fits(position, viewport, target_height, 144.0);

        assert_eq!(anchor.position.y + anchor.target_height, 556.0);
        assert!(viewport.height - (anchor.position.y + anchor.target_height) > anchor.position.y);
    }

    #[test]
    fn select_keeps_default_anchor_when_menu_does_not_fit_below() {
        let position = Point::new(0.0, 500.0);
        let target_height = 56.0;
        let viewport = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 620.0,
        };

        let anchor = prefer_down_when_menu_fits(position, viewport, target_height, 144.0);

        assert_eq!(anchor.position, position);
        assert_eq!(anchor.target_height, target_height);
    }
}
