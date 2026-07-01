//! Material 3 data table constructors with token-backed layout defaults.

use iced_widget::core::text as core_text;
use iced_widget::core::{Length, Padding, alignment};
use iced_widget::table::{self as iced_table, Column, Table};
use iced_widget::text;
use iced_widget::{Container, Text};

use super::absolute_line_height;
use crate::{Theme, text as text_style, tokens};

const CELL_HORIZONTAL_PADDING: f32 = 16.0;

pub fn standard<'a, 'b, T, Message, Renderer>(
    columns: impl IntoIterator<Item = Column<'a, 'b, T, Message, Theme, Renderer>>,
    rows: impl IntoIterator<Item = T>,
) -> Table<'a, Message, Theme, Renderer>
where
    T: Clone,
    Renderer: iced_widget::core::Renderer,
{
    iced_table::table(columns, rows)
        .width(Length::Fill)
        .padding(0.0)
        .separator(tokens::component::data_table::OUTLINE_WIDTH)
}

pub fn column<'a, 'b, T, F, Message, Renderer>(
    header: impl text::IntoFragment<'a>,
    view: impl Fn(T) -> F + 'b,
) -> Column<'a, 'b, T, Message, Theme, Renderer>
where
    T: 'a,
    F: text::IntoFragment<'a> + 'a,
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    iced_table::column(header_cell(header), move |row| body_cell(view(row)))
        .width(Length::Fill)
        .align_y(alignment::Vertical::Center)
}

pub fn numeric_column<'a, 'b, T, F, Message, Renderer>(
    header: impl text::IntoFragment<'a>,
    view: impl Fn(T) -> F + 'b,
) -> Column<'a, 'b, T, Message, Theme, Renderer>
where
    T: 'a,
    F: text::IntoFragment<'a> + 'a,
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    iced_table::column(header_cell_right(header), move |row| {
        body_cell_right(view(row))
    })
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Right)
    .align_y(alignment::Vertical::Center)
}

pub fn header_cell<'a, Message, Renderer>(
    label: impl text::IntoFragment<'a>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    header_cell_aligned(label, alignment::Horizontal::Left)
}

pub fn header_cell_right<'a, Message, Renderer>(
    label: impl text::IntoFragment<'a>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    header_cell_aligned(label, alignment::Horizontal::Right)
}

pub fn body_cell<'a, Message, Renderer>(
    label: impl text::IntoFragment<'a>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    body_cell_aligned(label, alignment::Horizontal::Left)
}

pub fn body_cell_right<'a, Message, Renderer>(
    label: impl text::IntoFragment<'a>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    body_cell_aligned(label, alignment::Horizontal::Right)
}

fn header_cell_aligned<'a, Message, Renderer>(
    label: impl text::IntoFragment<'a>,
    align_x: alignment::Horizontal,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    let label_text = tokens::typography::LABEL_LARGE;

    cell(
        Text::new(label)
            .size(label_text.size)
            .line_height(absolute_line_height(label_text.line_height))
            .style(text_style::surface_variant),
        tokens::component::data_table::HEADER_CONTAINER_HEIGHT,
        align_x,
    )
}

fn body_cell_aligned<'a, Message, Renderer>(
    label: impl text::IntoFragment<'a>,
    align_x: alignment::Horizontal,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    let body_text = tokens::typography::BODY_MEDIUM;

    cell(
        Text::new(label)
            .size(body_text.size)
            .line_height(absolute_line_height(body_text.line_height))
            .style(text_style::surface),
        tokens::component::data_table::ROW_ITEM_CONTAINER_HEIGHT,
        align_x,
    )
}

fn cell<'a, Message, Renderer>(
    content: Text<'a, Theme, Renderer>,
    height: f32,
    align_x: alignment::Horizontal,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    Container::new(content)
        .width(Length::Shrink)
        .height(Length::Fixed(height))
        .padding(Padding::from([0.0, CELL_HORIZONTAL_PADDING]))
        .align_x(align_x)
        .align_y(alignment::Vertical::Center)
}
