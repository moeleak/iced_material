use iced::Program;

use super::*;

#[derive(Debug, Clone)]
enum Message {}

fn update(_state: &mut (), _message: Message) {}

fn view(_state: &()) -> crate::Element<'_, Message> {
    iced::widget::text("").into()
}

#[test]
fn application_loads_bundled_material_fonts() {
    let application = application(|| (), update, view);
    let settings = Program::settings(&application);

    assert_eq!(settings.fonts.len(), fonts::all().len());
    assert_eq!(settings.default_font, fonts::ROBOTO);
}

#[test]
fn window_centers_without_min_size() {
    let size = Size::new(900.0, 640.0);
    let settings = window(size);

    assert_eq!(settings.size, size);
    assert_eq!(settings.min_size, None);
    assert!(matches!(settings.position, iced_window::Position::Centered));
}

#[test]
fn window_settings_centers_with_min_size() {
    let size = Size::new(1080.0, 980.0);
    let min_size = Size::new(420.0, 720.0);
    let settings = window_settings(size, Some(min_size));

    assert_eq!(settings.size, size);
    assert_eq!(settings.min_size, Some(min_size));
    assert!(matches!(settings.position, iced_window::Position::Centered));
}
