//! Material 3 searchable select constructors with token-backed layout defaults.

use std::fmt;

use iced_widget::combo_box as iced_combo_box;
use iced_widget::core::text as core_text;
use iced_widget::core::{Element, Length, Padding, Pixels};
use iced_widget::overlay::menu as overlay_menu;
use iced_widget::text::{self, LineHeight};
use iced_widget::text_input::{self, Icon};

use super::absolute_line_height;
use crate::{
    Theme, menu as menu_style, text_input as text_input_style, tokens,
};

#[derive(Clone)]
enum DisplayValue<T> {
    Option(T),
    Input(String),
}

impl<T> fmt::Display for DisplayValue<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Option(option) => option.fmt(f),
            Self::Input(input) => input.fmt(f),
        }
    }
}

/// Searchable select state.
///
/// The wrapped iced combo box keeps the currently typed query internally. We
/// mirror the user-visible query as a display value so the text does not vanish
/// when focus leaves the input.
pub struct State<T> {
    options: Vec<T>,
    inner: iced_combo_box::State<DisplayValue<T>>,
}

impl<T> fmt::Debug for State<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl<T> State<T>
where
    T: fmt::Display + Clone,
{
    /// Creates a new [`State`] for a combo box with the given list of options.
    pub fn new(options: Vec<T>) -> Self {
        Self::with_selection(options, None)
    }

    /// Creates a new [`State`] for a combo box with the given list of options
    /// and selected value.
    pub fn with_selection(options: Vec<T>, selection: Option<&T>) -> Self {
        let inner_options = inner_options(&options);
        let inner_selection = selection.cloned().map(DisplayValue::Option);

        Self {
            options,
            inner: iced_combo_box::State::with_selection(
                inner_options,
                inner_selection.as_ref(),
            ),
        }
    }

    /// Returns the original options.
    pub fn options(&self) -> &[T] {
        &self.options
    }

    /// Pushes a new option.
    pub fn push(&mut self, new_option: T) {
        self.inner.push(DisplayValue::Option(new_option.clone()));
        self.options.push(new_option);
    }

    /// Returns ownership of the original options.
    pub fn into_options(self) -> Vec<T> {
        self.options
    }

    /// Synchronizes the internal query with the latest user input.
    pub fn set_input(&mut self, input: impl Into<String>) {
        let input = input.into();
        let inner_selection = if input.is_empty() {
            None
        } else {
            Some(DisplayValue::Input(input))
        };

        self.inner = iced_combo_box::State::with_selection(
            inner_options(&self.options),
            inner_selection.as_ref(),
        );
    }

    /// Synchronizes the internal query with the selected option.
    pub fn set_selection(&mut self, selection: Option<&T>) {
        let inner_selection = selection.cloned().map(DisplayValue::Option);

        self.inner = iced_combo_box::State::with_selection(
            inner_options(&self.options),
            inner_selection.as_ref(),
        );
    }

    fn inner(&self) -> &iced_combo_box::State<DisplayValue<T>> {
        &self.inner
    }
}

impl<T> Default for State<T>
where
    T: fmt::Display + Clone,
{
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

/// Material combo box.
pub struct ComboBox<'a, T, Message, Renderer>
where
    T: fmt::Display + Clone,
    Renderer: core_text::Renderer,
{
    inner: iced_widget::ComboBox<'a, DisplayValue<T>, Message, Theme, Renderer>,
}

impl<T, Message, Renderer> fmt::Debug for ComboBox<'_, T, Message, Renderer>
where
    T: fmt::Display + Clone,
    Renderer: core_text::Renderer,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ComboBox").finish_non_exhaustive()
    }
}

impl<'a, T, Message, Renderer> ComboBox<'a, T, Message, Renderer>
where
    T: fmt::Display + Clone + 'static,
    Renderer: core_text::Renderer + 'a,
{
    /// Sets the message that should be produced when text is typed.
    pub fn on_input(
        mut self,
        on_input: impl Fn(String) -> Message + 'static,
    ) -> Self {
        self.inner = self.inner.on_input(on_input);
        self
    }

    /// Sets the message that will be produced when an option is hovered.
    pub fn on_option_hovered(
        mut self,
        on_option_hovered: impl Fn(T) -> Message + 'static,
    ) -> Self {
        self.inner =
            self.inner
                .on_option_hovered(move |value| match value {
                    DisplayValue::Option(option) => on_option_hovered(option),
                    DisplayValue::Input(_) => {
                        unreachable!("typed input is not a selectable option")
                    }
                });
        self
    }

    /// Sets the message that will be produced when the combo box opens.
    pub fn on_open(mut self, message: Message) -> Self {
        self.inner = self.inner.on_open(message);
        self
    }

    /// Sets the message that will be produced when the combo box closes.
    pub fn on_close(mut self, message: Message) -> Self {
        self.inner = self.inner.on_close(message);
        self
    }

    /// Sets the padding.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.inner = self.inner.padding(padding);
        self
    }

    /// Sets the font.
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.inner = self.inner.font(font);
        self
    }

    /// Sets the trailing icon.
    pub fn icon(mut self, icon: Icon<Renderer::Font>) -> Self {
        self.inner = self.inner.icon(icon);
        self
    }

    /// Sets the text size.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.inner = self.inner.size(size);
        self
    }

    /// Sets the text line height.
    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.inner = self.inner.line_height(line_height);
        self
    }

    /// Sets the width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.inner = self.inner.width(width);
        self
    }

    /// Sets the menu height.
    pub fn menu_height(mut self, menu_height: impl Into<Length>) -> Self {
        self.inner = self.inner.menu_height(menu_height);
        self
    }

    /// Sets the text shaping strategy.
    pub fn text_shaping(mut self, shaping: text::Shaping) -> Self {
        self.inner = self.inner.text_shaping(shaping);
        self
    }

    /// Sets the input style.
    pub fn input_style(
        mut self,
        style: impl Fn(&Theme, text_input::Status) -> text_input::Style + 'a,
    ) -> Self
    where
        <Theme as text_input::Catalog>::Class<'a>:
            From<text_input::StyleFn<'a, Theme>>,
    {
        self.inner = self.inner.input_style(style);
        self
    }

    /// Sets the menu style.
    pub fn menu_style(
        mut self,
        style: impl Fn(&Theme) -> overlay_menu::Style + 'a,
    ) -> Self
    where
        <Theme as overlay_menu::Catalog>::Class<'a>:
            From<overlay_menu::StyleFn<'a, Theme>>,
    {
        self.inner = self.inner.menu_style(style);
        self
    }
}

impl<'a, T, Message, Renderer> From<ComboBox<'a, T, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: fmt::Display + Clone + 'static,
    Message: Clone + 'a,
    Renderer: core_text::Renderer + 'a,
{
    fn from(combo_box: ComboBox<'a, T, Message, Renderer>) -> Self {
        Element::new(combo_box.inner)
    }
}

pub fn outlined<'a, T, Message, Renderer>(
    state: &'a State<T>,
    placeholder: &str,
    selection: Option<&T>,
    on_selected: impl Fn(T) -> Message + 'static,
) -> ComboBox<'a, T, Message, Renderer>
where
    T: fmt::Display + Clone + 'static,
    Renderer: core_text::Renderer + 'a,
{
    outlined_with_input(state, placeholder, "", selection, on_selected)
}

pub fn outlined_with_input<'a, T, Message, Renderer>(
    state: &'a State<T>,
    placeholder: &str,
    input: &str,
    selection: Option<&T>,
    on_selected: impl Fn(T) -> Message + 'static,
) -> ComboBox<'a, T, Message, Renderer>
where
    T: fmt::Display + Clone + 'static,
    Renderer: core_text::Renderer + 'a,
{
    let display_value = if input.is_empty() {
        selection.cloned().map(DisplayValue::Option)
    } else {
        Some(DisplayValue::Input(input.to_owned()))
    };

    let inner = iced_widget::ComboBox::new(
        state.inner(),
        placeholder,
        display_value.as_ref(),
        move |value| match value {
            DisplayValue::Option(option) => on_selected(option),
            DisplayValue::Input(_) => {
                unreachable!("typed input is not a selectable option")
            }
        },
    )
    .padding(Padding {
        top: tokens::component::text_field::TOP_SPACE,
        right: tokens::component::text_field::TRAILING_SPACE,
        bottom: tokens::component::text_field::BOTTOM_SPACE,
        left: tokens::component::text_field::LEADING_SPACE,
    })
    .size(tokens::component::text_field::INPUT_TEXT_SIZE)
    .line_height(absolute_line_height(
        tokens::component::text_field::INPUT_TEXT_LINE_HEIGHT,
    ))
    .width(Length::Fill)
    .input_style(text_input_style::default)
    .menu_style(menu_style::outlined_select);

    ComboBox { inner }
}

fn inner_options<T>(options: &[T]) -> Vec<DisplayValue<T>>
where
    T: Clone,
{
    options
        .iter()
        .cloned()
        .map(DisplayValue::Option)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_value_uses_typed_query_text() {
        assert_eq!(DisplayValue::<&str>::Input("xxx".into()).to_string(), "xxx");
    }

    #[test]
    fn state_preserves_original_options() {
        let mut state = State::new(vec!["Assist", "Suggestion"]);

        state.push("Filter");

        assert_eq!(state.options(), &["Assist", "Suggestion", "Filter"]);
    }
}
