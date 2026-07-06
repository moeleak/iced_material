//! Helpers for bootstrapping `iced` applications with Material defaults.

use iced::{Size, window as iced_window};
use iced_winit::program;

use crate::{Theme, fonts};

/// Creates an `iced` application with the bundled Material fonts preloaded.
///
/// This is equivalent to [`iced::application()`] followed by
/// [`with_material_fonts`].
pub fn application<State, Message, Renderer>(
    boot: impl iced::application::BootFn<State, Message>,
    update: impl iced::application::UpdateFn<State, Message>,
    view: impl for<'a> iced::application::ViewFn<'a, State, Message, Theme, Renderer>,
) -> iced::Application<impl iced::Program<State = State, Message = Message, Theme = Theme>>
where
    State: 'static,
    Message: Send + 'static,
    Renderer: program::Renderer,
{
    with_material_fonts(iced::application(boot, update, view))
}

/// Adds the bundled Material fonts to an existing `iced` application.
pub fn with_material_fonts<P>(application: iced::Application<P>) -> iced::Application<P>
where
    P: iced::Program,
{
    fonts::all()
        .into_iter()
        .fold(application, iced::Application::font)
        .default_font(fonts::ROBOTO)
}

/// Returns centered window settings for the provided size.
pub fn window(size: Size) -> iced_window::Settings {
    window_settings(size, None)
}

/// Returns centered window settings with the provided minimum size.
pub fn window_with_min_size(size: Size, min_size: Size) -> iced_window::Settings {
    window_settings(size, Some(min_size))
}

/// Returns centered window settings with the provided size constraints.
pub fn window_settings(size: Size, min_size: Option<Size>) -> iced_window::Settings {
    iced_window::Settings {
        size,
        min_size,
        position: iced_window::Position::Centered,
        ..iced_window::Settings::default()
    }
}

#[cfg(test)]
#[path = "../tests/application.rs"]
mod tests;
