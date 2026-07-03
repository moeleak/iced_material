//! Material 3 tooltip constructors with token-backed layout defaults.

use super::*;

pub use iced_tooltip::Position;

pub fn plain<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    supporting_text: impl text::IntoFragment<'a>,
    position: Position,
) -> Tooltip<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    let type_scale = tokens::component::tooltip::PLAIN_SUPPORTING_TEXT;

    let tooltip = Container::new(plain_supporting_text::<Renderer>(
        supporting_text,
        type_scale,
    ))
    .padding(Padding {
        top: 0.0,
        right: plain_tooltip_inner_horizontal_padding(),
        bottom: 0.0,
        left: plain_tooltip_inner_horizontal_padding(),
    })
    .max_width(plain_tooltip_inner_max_width());

    Tooltip::new(content, tooltip, position)
        .gap(tokens::component::tooltip::SPACING_BETWEEN_TOOLTIP_AND_ANCHOR)
        .padding(tokens::component::tooltip::PLAIN_VERTICAL_SPACE)
        .style(tooltip_style::plain)
}

pub fn rich<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    supporting_text: impl text::IntoFragment<'a>,
    position: Position,
) -> RichTooltip<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    rich_surface(content, None, supporting_text, None, position)
}

pub fn rich_with_title<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    title: impl text::IntoFragment<'a>,
    supporting_text: impl text::IntoFragment<'a>,
    position: Position,
) -> RichTooltip<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    rich_surface(
        content,
        Some(title.into_fragment()),
        supporting_text,
        None,
        position,
    )
}

pub fn rich_with_title_action<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    title: impl text::IntoFragment<'a>,
    supporting_text: impl text::IntoFragment<'a>,
    action: impl Into<Element<'a, Message, Theme, Renderer>>,
    position: Position,
) -> RichTooltip<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    rich_surface(
        content,
        Some(title.into_fragment()),
        supporting_text,
        Some(action.into()),
        position,
    )
}

pub fn rich_action<'a, Message, Renderer>(
    label: impl text::IntoFragment<'a>,
) -> button::Button<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_widget::graphics::geometry::Renderer + core_text::Renderer + 'a,
{
    button::text(label)
}

pub fn rich_action_button<'a, Message, Renderer>(
    label: impl text::IntoFragment<'a>,
    on_press: Message,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_widget::graphics::geometry::Renderer + core_text::Renderer + 'a,
{
    rich_action(label).on_press(on_press).into()
}

fn plain_supporting_text<'a, Renderer>(
    supporting_text: impl text::IntoFragment<'a>,
    type_scale: tokens::typography::TypeScale,
) -> Text<'a, Theme, Renderer>
where
    Renderer: core_text::Renderer,
{
    text_with_metrics(supporting_text, type_scale.size, type_scale.line_height)
        .wrapping(text::Wrapping::Word)
}

fn rich_surface<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    title: Option<text::Fragment<'a>>,
    supporting_text: impl text::IntoFragment<'a>,
    action: Option<Element<'a, Message, Theme, Renderer>>,
    position: Position,
) -> RichTooltip<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    let has_title = title.is_some();
    let has_action = action.is_some();
    let mut tooltip = iced_widget::Column::new()
        .width(Length::Fill)
        .padding(Padding {
            top: 0.0,
            right: tokens::component::tooltip::RICH_HORIZONTAL_SPACE,
            bottom: 0.0,
            left: tokens::component::tooltip::RICH_HORIZONTAL_SPACE,
        });

    if let Some(title) = title {
        tooltip = tooltip.push(
            Container::new(rich_title_text::<Renderer>(title))
                .padding(Padding {
                    top: rich_title_top_padding(),
                    right: 0.0,
                    bottom: 0.0,
                    left: 0.0,
                })
                .width(Length::Fill),
        );
    }

    tooltip = tooltip.push(
        Container::new(rich_supporting_text::<Renderer>(supporting_text))
            .padding(rich_supporting_text_padding(has_title, has_action))
            .width(Length::Fill),
    );

    if let Some(action) = action {
        tooltip = tooltip.push(
            Container::new(action)
                .padding(Padding {
                    top: 0.0,
                    right: 0.0,
                    bottom: tokens::component::tooltip::RICH_ACTION_LABEL_BOTTOM_PADDING,
                    left: 0.0,
                })
                .width(Length::Fill),
        );
    }

    let tooltip = Container::new(tooltip)
        .width(Length::Fill)
        .height(Length::Shrink)
        .max_width(tokens::component::tooltip::RICH_MAX_WIDTH);

    RichTooltip::new(content, tooltip, position)
        .gap(tokens::component::tooltip::SPACING_BETWEEN_TOOLTIP_AND_ANCHOR)
        .padding(0.0)
        .style(tooltip_style::rich)
}

/// A Material rich tooltip that remains interactive while the pointer moves
/// from its anchor to the tooltip surface.
pub struct RichTooltip<'a, Message, Renderer = iced_widget::Renderer>
where
    Renderer: core_text::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    tooltip: Element<'a, Message, Theme, Renderer>,
    position: Position,
    gap: f32,
    padding: f32,
    snap_within_viewport: bool,
    class: <Theme as iced_container::Catalog>::Class<'a>,
}

impl<Message, Renderer> std::fmt::Debug for RichTooltip<'_, Message, Renderer>
where
    Renderer: core_text::Renderer,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RichTooltip")
            .field("position", &self.position)
            .field("gap", &self.gap)
            .field("padding", &self.padding)
            .field("snap_within_viewport", &self.snap_within_viewport)
            .finish_non_exhaustive()
    }
}

impl<'a, Message, Renderer> RichTooltip<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: core_text::Renderer + 'a,
{
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        tooltip: impl Into<Element<'a, Message, Theme, Renderer>>,
        position: Position,
    ) -> Self {
        Self {
            content: content.into(),
            tooltip: tooltip.into(),
            position,
            gap: 0.0,
            padding: 0.0,
            snap_within_viewport: true,
            class: <Theme as iced_container::Catalog>::default(),
        }
    }

    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().0;
        self
    }

    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = padding.into().0;
        self
    }

    pub fn snap_within_viewport(mut self, snap: bool) -> Self {
        self.snap_within_viewport = snap;
        self
    }

    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> iced_container::Style + 'a) -> Self
    where
        <Theme as iced_container::Catalog>::Class<'a>: From<iced_container::StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as iced_container::StyleFn<'a, Theme>).into();
        self
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for RichTooltip<'_, Message, Renderer>
where
    Renderer: core_text::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<RichTooltipState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(RichTooltipState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content), Tree::new(&self.tooltip)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.content.as_widget(), self.tooltip.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let Event::Mouse(_) = event {
            let state = tree.state.downcast_mut::<RichTooltipState>();
            let cursor_position = cursor.position_over(layout.bounds());

            match (*state, cursor_position) {
                (RichTooltipState::Idle, Some(cursor_position)) => {
                    *state = RichTooltipState::Open { cursor_position };
                    shell.invalidate_layout();
                    shell.request_redraw();
                }
                (
                    RichTooltipState::Open {
                        cursor_position: last_position,
                    },
                    Some(cursor_position),
                ) if self.position == Position::FollowCursor
                    && cursor_position != last_position =>
                {
                    *state = RichTooltipState::Open { cursor_position };
                    shell.request_redraw();
                }
                (RichTooltipState::Open { .. }, None) => {
                    *state = RichTooltipState::Idle;
                    shell.invalidate_layout();
                    shell.request_redraw();
                }
                _ => {}
            }
        }

        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            defaults,
            layout,
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_ref::<RichTooltipState>();
        let mut children = tree.children.iter_mut();

        let content = self.content.as_widget_mut().overlay(
            children.next().unwrap(),
            layout,
            renderer,
            viewport,
            translation,
        );

        let content_bounds = translated_bounds(layout.bounds(), translation);
        let tooltip = if let RichTooltipState::Open { cursor_position } = *state {
            Some(overlay::Element::new(Box::new(RichTooltipOverlay {
                tooltip: &mut self.tooltip,
                tree: children.next().unwrap(),
                cursor_position,
                content_bounds,
                snap_within_viewport: self.snap_within_viewport,
                position: self.position,
                gap: self.gap,
                padding: self.padding,
                class: &self.class,
            })))
        } else {
            None
        };

        if content.is_some() || tooltip.is_some() {
            Some(
                overlay::Group::with_children(content.into_iter().chain(tooltip).collect())
                    .overlay(),
            )
        } else {
            None
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn core_widget::Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                layout,
                renderer,
                operation,
            );
        });
    }
}

impl<'a, Message, Renderer> From<RichTooltip<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: core_text::Renderer + 'a,
{
    fn from(tooltip: RichTooltip<'a, Message, Renderer>) -> Self {
        Element::new(tooltip)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum RichTooltipState {
    #[default]
    Idle,
    Open {
        cursor_position: Point,
    },
}

struct RichTooltipOverlay<'a, 'b, Message, Renderer>
where
    Renderer: core_text::Renderer,
{
    tooltip: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut Tree,
    cursor_position: Point,
    content_bounds: Rectangle,
    snap_within_viewport: bool,
    position: Position,
    gap: f32,
    padding: f32,
    class: &'b <Theme as iced_container::Catalog>::Class<'a>,
}

impl<Message, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for RichTooltipOverlay<'_, '_, Message, Renderer>
where
    Renderer: core_text::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let viewport = Rectangle::with_size(bounds);
        let tooltip_layout = self.tooltip.as_widget_mut().layout(
            self.tree,
            renderer,
            &layout::Limits::new(
                Size::ZERO,
                if self.snap_within_viewport {
                    viewport.size()
                } else {
                    Size::INFINITE
                },
            )
            .shrink(Padding::new(self.padding)),
        );

        let tooltip_bounds = rich_tooltip_surface_bounds(
            self.content_bounds,
            tooltip_layout.bounds().size(),
            viewport,
            self.cursor_position,
            self.position,
            self.gap,
            self.padding,
            self.snap_within_viewport,
        );

        layout::Node::with_children(
            tooltip_bounds.size(),
            vec![tooltip_layout.translate(Vector::new(self.padding, self.padding))],
        )
        .translate(Vector::new(tooltip_bounds.x, tooltip_bounds.y))
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let tooltip_bounds = layout.bounds();
        let hover_bounds = rich_tooltip_hover_bounds(self.content_bounds, tooltip_bounds);
        let cursor_in_overlay =
            cursor.is_over(hover_bounds) && !cursor.is_over(self.content_bounds);

        if cursor.is_over(tooltip_bounds)
            && let Some(child_layout) = layout.children().next()
        {
            self.tooltip.as_widget_mut().update(
                self.tree,
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                &Rectangle::with_size(Size::INFINITE),
            );
        }

        if cursor_in_overlay && matches!(event, Event::Mouse(_)) {
            shell.capture_event();
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let tooltip_bounds = layout.bounds();

        if cursor.is_over(tooltip_bounds)
            && let Some(child_layout) = layout.children().next()
        {
            return self.tooltip.as_widget().mouse_interaction(
                self.tree,
                child_layout,
                cursor,
                &Rectangle::with_size(Size::INFINITE),
                renderer,
            );
        }

        mouse::Interaction::default()
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        inherited_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let style = iced_container::Catalog::style(theme, self.class);

        iced_container::draw_background(renderer, &style, layout.bounds());

        let defaults = renderer::Style {
            text_color: style.text_color.unwrap_or(inherited_style.text_color),
        };

        self.tooltip.as_widget().draw(
            self.tree,
            renderer,
            theme,
            &defaults,
            layout.children().next().unwrap(),
            cursor,
            &Rectangle::with_size(Size::INFINITE),
        );
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'c>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
        self.tooltip.as_widget_mut().overlay(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
            &Rectangle::with_size(Size::INFINITE),
            Vector::ZERO,
        )
    }
}

fn translated_bounds(bounds: Rectangle, translation: Vector) -> Rectangle {
    Rectangle {
        x: bounds.x + translation.x,
        y: bounds.y + translation.y,
        ..bounds
    }
}

fn rich_tooltip_surface_bounds(
    content_bounds: Rectangle,
    tooltip_size: Size,
    viewport: Rectangle,
    cursor_position: Point,
    position: Position,
    gap: f32,
    padding: f32,
    snap_within_viewport: bool,
) -> Rectangle {
    let x_center = content_bounds.x + (content_bounds.width - tooltip_size.width) / 2.0;
    let y_center = content_bounds.y + (content_bounds.height - tooltip_size.height) / 2.0;

    let offset = match position {
        Position::Top => Vector::new(
            x_center,
            content_bounds.y - tooltip_size.height - gap - padding,
        ),
        Position::Bottom => Vector::new(
            x_center,
            content_bounds.y + content_bounds.height + gap + padding,
        ),
        Position::Left => Vector::new(
            content_bounds.x - tooltip_size.width - gap - padding,
            y_center,
        ),
        Position::Right => Vector::new(
            content_bounds.x + content_bounds.width + gap + padding,
            y_center,
        ),
        Position::FollowCursor => {
            Vector::new(cursor_position.x, cursor_position.y - tooltip_size.height)
        }
    };

    let mut tooltip_bounds = Rectangle {
        x: offset.x - padding,
        y: offset.y - padding,
        width: tooltip_size.width + padding * 2.0,
        height: tooltip_size.height + padding * 2.0,
    };

    if snap_within_viewport {
        if tooltip_bounds.x < viewport.x {
            tooltip_bounds.x = viewport.x;
        } else if viewport.x + viewport.width < tooltip_bounds.x + tooltip_bounds.width {
            tooltip_bounds.x = viewport.x + viewport.width - tooltip_bounds.width;
        }

        if tooltip_bounds.y < viewport.y {
            tooltip_bounds.y = viewport.y;
        } else if viewport.y + viewport.height < tooltip_bounds.y + tooltip_bounds.height {
            tooltip_bounds.y = viewport.y + viewport.height - tooltip_bounds.height;
        }
    }

    tooltip_bounds
}

fn rich_tooltip_hover_bounds(content_bounds: Rectangle, tooltip_bounds: Rectangle) -> Rectangle {
    let left = content_bounds.x.min(tooltip_bounds.x);
    let top = content_bounds.y.min(tooltip_bounds.y);
    let right =
        (content_bounds.x + content_bounds.width).max(tooltip_bounds.x + tooltip_bounds.width);
    let bottom =
        (content_bounds.y + content_bounds.height).max(tooltip_bounds.y + tooltip_bounds.height);

    Rectangle {
        x: left,
        y: top,
        width: right - left,
        height: bottom - top,
    }
}

fn rich_title_text<'a, Renderer>(title: impl text::IntoFragment<'a>) -> Text<'a, Theme, Renderer>
where
    Renderer: core_text::Renderer,
{
    let scale = tokens::component::tooltip::RICH_SUBHEAD_TEXT;

    text_with_metrics(title, scale.size, scale.line_height)
        .wrapping(text::Wrapping::Word)
        .style(rich_title_style)
}

fn rich_supporting_text<'a, Renderer>(
    supporting_text: impl text::IntoFragment<'a>,
) -> Text<'a, Theme, Renderer>
where
    Renderer: core_text::Renderer,
{
    let scale = tokens::component::tooltip::RICH_SUPPORTING_TEXT;

    text_with_metrics(supporting_text, scale.size, scale.line_height)
        .wrapping(text::Wrapping::Word)
        .style(rich_supporting_text_style)
}

fn rich_title_top_padding() -> f32 {
    (tokens::component::tooltip::RICH_HEIGHT_TO_SUBHEAD_FIRST_LINE
        - tokens::component::tooltip::RICH_SUBHEAD_TEXT.line_height)
        .max(0.0)
}

fn rich_supporting_text_padding(has_title: bool, has_action: bool) -> Padding {
    if !has_title && !has_action {
        return Padding {
            top: tokens::component::tooltip::RICH_TEXT_VERTICAL_SPACE_WITHOUT_TITLE_OR_ACTION,
            right: 0.0,
            bottom: tokens::component::tooltip::RICH_TEXT_VERTICAL_SPACE_WITHOUT_TITLE_OR_ACTION,
            left: 0.0,
        };
    }

    Padding {
        top: (tokens::component::tooltip::RICH_HEIGHT_FROM_SUBHEAD_TO_TEXT_FIRST_LINE
            - tokens::component::tooltip::RICH_SUPPORTING_TEXT.line_height)
            .max(0.0),
        right: 0.0,
        bottom: tokens::component::tooltip::RICH_TEXT_BOTTOM_PADDING,
        left: 0.0,
    }
}

fn rich_title_style(theme: &Theme) -> iced_widget::text::Style {
    iced_widget::text::Style {
        color: Some(theme.colors().surface.text_variant),
    }
}

fn rich_supporting_text_style(theme: &Theme) -> iced_widget::text::Style {
    iced_widget::text::Style {
        color: Some(theme.colors().surface.text_variant),
    }
}

fn plain_tooltip_inner_horizontal_padding() -> f32 {
    (tokens::component::tooltip::PLAIN_HORIZONTAL_SPACE
        - tokens::component::tooltip::PLAIN_VERTICAL_SPACE)
        .max(0.0)
}

fn plain_tooltip_inner_max_width() -> f32 {
    tokens::component::tooltip::PLAIN_MAX_WIDTH
        - tokens::component::tooltip::PLAIN_VERTICAL_SPACE * 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_tooltip_text_shrinks_under_material_max_width() {
        let type_scale = tokens::component::tooltip::PLAIN_SUPPORTING_TEXT;
        let text: Text<'_, Theme, iced_widget::Renderer> =
            plain_supporting_text("Material 3 plain tooltip", type_scale);

        assert_eq!(
            Widget::<(), Theme, iced_widget::Renderer>::size(&text).width,
            Length::Shrink
        );
        assert_eq!(plain_tooltip_inner_horizontal_padding(), 4.0);
        assert_eq!(plain_tooltip_inner_max_width(), 192.0);
        assert_eq!(
            plain_tooltip_inner_max_width()
                + tokens::component::tooltip::PLAIN_VERTICAL_SPACE * 2.0,
            tokens::component::tooltip::PLAIN_MAX_WIDTH
        );
    }

    #[test]
    fn rich_tooltip_padding_matches_androidx_material_layout_constants() {
        assert_eq!(rich_title_top_padding(), 8.0);
        assert_eq!(
            rich_supporting_text_padding(false, false),
            Padding {
                top: 4.0,
                right: 0.0,
                bottom: 4.0,
                left: 0.0,
            }
        );
        assert_eq!(
            rich_supporting_text_padding(true, true),
            Padding {
                top: 4.0,
                right: 0.0,
                bottom: 16.0,
                left: 0.0,
            }
        );
    }

    #[test]
    fn rich_tooltip_surface_keeps_material_gap_from_anchor() {
        let content = Rectangle {
            x: 120.0,
            y: 160.0,
            width: 80.0,
            height: 32.0,
        };
        let tooltip = rich_tooltip_surface_bounds(
            content,
            Size::new(180.0, 96.0),
            Rectangle::new(Point::ORIGIN, Size::new(400.0, 400.0)),
            Point::ORIGIN,
            Position::Top,
            tokens::component::tooltip::SPACING_BETWEEN_TOOLTIP_AND_ANCHOR,
            0.0,
            true,
        );

        assert_eq!(
            content.y - (tooltip.y + tooltip.height),
            tokens::component::tooltip::SPACING_BETWEEN_TOOLTIP_AND_ANCHOR
        );
    }

    #[test]
    fn rich_tooltip_hover_bounds_span_anchor_surface_and_gap() {
        let content = Rectangle {
            x: 120.0,
            y: 160.0,
            width: 80.0,
            height: 32.0,
        };
        let tooltip = Rectangle {
            x: 70.0,
            y: 60.0,
            width: 180.0,
            height: 96.0,
        };
        let hover = rich_tooltip_hover_bounds(content, tooltip);

        assert!(hover.contains(Point::new(160.0, 158.0)));
        assert!(hover.contains(Point::new(160.0, 120.0)));
        assert!(hover.contains(Point::new(160.0, 176.0)));
    }
}
