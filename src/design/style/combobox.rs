use iced_widget::combo_box::Catalog;
use iced_widget::overlay::menu as overlay_menu;
use iced_widget::text_input as iced_text_input;

use crate::Theme;

impl Catalog for Theme {
    fn default_input<'a>() -> <Self as iced_text_input::Catalog>::Class<'a> {
        Box::new(crate::style::text_input::default)
    }

    fn default_menu<'a>() -> <Self as overlay_menu::Catalog>::Class<'a> {
        Box::new(crate::style::menu::outlined_select)
    }
}

#[cfg(test)]
#[path = "../../tests/design/style/combobox.rs"]
mod tests;
