use iced::time::{Duration, Instant};
use iced::widget::{column, row, text};
use iced::{Alignment, Color, Element, Length, Size, Subscription, window};
use iced_material as material;
use material::{ColorQuartet, ColorScheme, Inverse, Outline, Surface, SurfaceContainer, Theme};

fn type_scale_line_height(
    scale: material::tokens::typography::TypeScale,
) -> iced::widget::text::LineHeight {
    iced::widget::text::LineHeight::Absolute(scale.line_height.into())
}

pub fn main() -> iced::Result {
    let window_size = Size::new(1080.0, 980.0);

    iced::application(Demo::default, update, view)
        .title("iced_material demo")
        .subscription(subscription)
        .theme(theme)
        .window(window::Settings {
            size: window_size,
            min_size: Some(window_size),
            position: window::Position::Centered,
            ..window::Settings::default()
        })
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    TextChanged(String),
    EditorAction(material::widget::text_editor::Action),
    SelectChanged(&'static str),
    ComboSelected(&'static str),
    ComboInputChanged(String),
    SliderChanged(f32),
    EnabledChanged(bool),
    DarkModeChanged(bool),
    ChoiceSelected(RadioChoice),
    Frame(Instant),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RadioChoice {
    Standard,
    Expressive,
    Dense,
}

#[derive(Debug, Clone, Copy)]
struct InventoryRow {
    component: &'static str,
    status: &'static str,
    count: u32,
}

const INVENTORY_ROWS: [InventoryRow; 3] = [
    InventoryRow {
        component: "Buttons",
        status: "Enabled",
        count: 4,
    },
    InventoryRow {
        component: "Selection",
        status: "Animated",
        count: 3,
    },
    InventoryRow {
        component: "Inputs",
        status: "Focused",
        count: 5,
    },
];

#[derive(Debug)]
struct Demo {
    count: i32,
    note: String,
    editor_content: material::widget::text_editor::Content,
    select_choice: Option<&'static str>,
    combo_options: material::widget::combo_box::State<&'static str>,
    combo_choice: Option<&'static str>,
    combo_input: String,
    progress: f32,
    enabled: bool,
    dark_mode: bool,
    radio_choice: Option<RadioChoice>,
    visible_scheme: ColorScheme,
    animation: Option<ThemeAnimation>,
}

#[derive(Debug, Clone, Copy)]
struct ThemeAnimation {
    from: ColorScheme,
    to: ColorScheme,
    started_at: Instant,
}

impl Default for Demo {
    fn default() -> Self {
        let initial_theme = Theme::Dark;

        Self {
            count: 0,
            note: String::new(),
            editor_content: material::widget::text_editor::Content::with_text(
                "Material 3 multi-line text editor",
            ),
            select_choice: Some("Assist"),
            combo_options: material::widget::combo_box::State::with_selection(
                vec!["Assist", "Suggestion", "Filter"],
                Some(&"Suggestion"),
            ),
            combo_choice: Some("Suggestion"),
            combo_input: String::new(),
            progress: 42.0,
            enabled: true,
            dark_mode: true,
            radio_choice: Some(RadioChoice::Standard),
            visible_scheme: initial_theme.colors(),
            animation: None,
        }
    }
}

impl Demo {
    fn theme(&self) -> Theme {
        Theme::new("Material 3 animated", self.visible_scheme)
    }
}

fn update(state: &mut Demo, message: Message) {
    match message {
        Message::Increment => state.count += 1,
        Message::Decrement => state.count -= 1,
        Message::TextChanged(note) => state.note = note,
        Message::EditorAction(action) => state.editor_content.perform(action),
        Message::SelectChanged(choice) => state.select_choice = Some(choice),
        Message::ComboSelected(choice) => {
            state.combo_choice = Some(choice);
            state.combo_input.clear();
            state.combo_options.set_selection(Some(&choice));
        }
        Message::ComboInputChanged(input) => {
            state.combo_options.set_input(input.clone());
            state.combo_input = input;
            state.combo_choice = None;
        }
        Message::SliderChanged(progress) => state.progress = progress,
        Message::EnabledChanged(enabled) => state.enabled = enabled,
        Message::ChoiceSelected(choice) => state.radio_choice = Some(choice),
        Message::DarkModeChanged(dark_mode) => {
            state.dark_mode = dark_mode;

            let target = if dark_mode {
                Theme::Dark.colors()
            } else {
                Theme::Light.colors()
            };

            state.animation = Some(ThemeAnimation {
                from: state.visible_scheme,
                to: target,
                started_at: Instant::now(),
            });
        }
        Message::Frame(now) => {
            if let Some(animation) = state.animation {
                let duration =
                    Duration::from_millis(u64::from(material::tokens::motion::DURATION_MEDIUM4_MS));
                let progress = now
                    .saturating_duration_since(animation.started_at)
                    .as_secs_f32()
                    / duration.as_secs_f32();

                if progress >= 1.0 {
                    state.visible_scheme = animation.to;
                    state.animation = None;
                } else {
                    state.visible_scheme = lerp_color_scheme(
                        animation.from,
                        animation.to,
                        emphasized_decelerate(progress),
                    );
                }
            }
        }
    }
}

fn theme(state: &Demo) -> Theme {
    state.theme()
}

fn subscription(state: &Demo) -> Subscription<Message> {
    if state.animation.is_some() {
        iced::window::frames().map(Message::Frame)
    } else {
        Subscription::none()
    }
}

fn view(state: &Demo) -> Element<'_, Message, Theme> {
    let body_large = material::tokens::typography::BODY_LARGE;
    let body_medium = material::tokens::typography::BODY_MEDIUM;
    let headline_large = material::tokens::typography::HEADLINE_LARGE;
    let headline_medium = material::tokens::typography::HEADLINE_MEDIUM;
    let title_medium = material::tokens::typography::TITLE_MEDIUM;

    let header = column![
        text("iced_material 0.14.2")
            .size(headline_large.size)
            .line_height(type_scale_line_height(headline_large)),
        text("Material 3 inspired widgets running on iced 0.14")
            .size(body_large.size)
            .line_height(type_scale_line_height(body_large)),
    ]
    .spacing(6);

    let controls = row![
        material::widget::button::outlined("Minus").on_press(Message::Decrement),
        text(state.count)
            .size(headline_medium.size)
            .line_height(type_scale_line_height(headline_medium)),
        material::widget::button::filled("Plus").on_press(Message::Increment),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let input = material::widget::text_input::outlined("Write a note", &state.note)
        .on_input(Message::TextChanged);

    let editor = material::widget::text_editor::outlined(&state.editor_content)
        .placeholder("Write details")
        .on_action(Message::EditorAction)
        .height(Length::Fixed(112.0));

    let select_options = ["Assist", "Suggestion", "Filter"];
    let select = material::widget::pick_list::outlined(
        select_options,
        state.select_choice,
        Message::SelectChanged,
    )
    .placeholder("Choose a chip")
    .width(Length::Fill);

    let combo_box = material::widget::combo_box::outlined_with_input(
        &state.combo_options,
        "Search a chip",
        &state.combo_input,
        state.combo_choice.as_ref(),
        Message::ComboSelected,
    )
    .on_input(Message::ComboInputChanged);

    let dividers = column![
        material::widget::rule::horizontal_full_width(),
        row![
            text("Full")
                .size(body_large.size)
                .line_height(type_scale_line_height(body_large)),
            material::widget::rule::vertical_full_height(),
            text("Inset")
                .size(body_large.size)
                .line_height(type_scale_line_height(body_large)),
        ]
        .height(Length::Fixed(32.0))
        .spacing(16)
        .align_y(Alignment::Center),
        material::widget::rule::horizontal_inset(),
    ]
    .spacing(8);

    let progress = column![
        row![
            text("Progress")
                .size(body_large.size)
                .line_height(type_scale_line_height(body_large))
                .width(Length::Fill),
            text(format!("{:.0}%", state.progress))
                .size(body_large.size)
                .line_height(type_scale_line_height(body_large)),
        ]
        .spacing(12),
        material::widget::slider::continuous(0.0..=100.0, state.progress, Message::SliderChanged)
            .step(1.0),
        material::widget::progress_bar::linear(0.0..=100.0, state.progress),
    ]
    .spacing(10);

    let switches = column![
        material::widget::checkbox::standard(
            state.enabled,
            "Enable actions",
            Message::EnabledChanged,
        ),
        material::widget::toggler::standard(
            state.dark_mode,
            "Dark theme",
            Message::DarkModeChanged,
        ),
    ]
    .spacing(12);

    let radios = row![
        material::widget::radio::standard(
            "Standard",
            RadioChoice::Standard,
            state.radio_choice,
            Message::ChoiceSelected,
        ),
        material::widget::radio::standard(
            "Expressive",
            RadioChoice::Expressive,
            state.radio_choice,
            Message::ChoiceSelected,
        ),
        material::widget::radio::standard(
            "Dense",
            RadioChoice::Dense,
            state.radio_choice,
            Message::ChoiceSelected,
        ),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let actions = row![
        material::widget::button::filled("Filled")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
        material::widget::button::filled_tonal("Tonal")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
        material::widget::button::text("Text")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
        material::widget::button::primary_fab("+")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
    ]
    .spacing(12);

    let chips = row![
        material::widget::button::assist_chip("Assist")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
        material::widget::button::suggestion_chip("Suggestion")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
        material::widget::button::filter_chip("Filter")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
        material::widget::button::selected_filter_chip("Selected")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
        material::widget::tooltip::plain(
            material::widget::button::assist_chip("Hint")
                .on_press_maybe(state.enabled.then_some(Message::Increment)),
            "Material 3 plain tooltip",
            material::widget::tooltip::Position::Top,
        ),
    ]
    .spacing(8);

    let badges = row![
        text("Badges")
            .size(body_large.size)
            .line_height(type_scale_line_height(body_large)),
        material::widget::badge::small(),
        material::widget::badge::large("3"),
        material::widget::badge::large("99+"),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let elevated_card = material::widget::card::elevated(
        column![
            text("Elevated")
                .size(title_medium.size)
                .line_height(type_scale_line_height(title_medium)),
            text("Level 1")
                .size(body_medium.size)
                .line_height(type_scale_line_height(body_medium)),
        ]
        .spacing(2),
    )
    .padding(12)
    .height(Length::Fixed(78.0))
    .width(Length::FillPortion(1));

    let filled_card = material::widget::card::filled(
        column![
            text("Filled")
                .size(title_medium.size)
                .line_height(type_scale_line_height(title_medium)),
            text("Container")
                .size(body_medium.size)
                .line_height(type_scale_line_height(body_medium)),
        ]
        .spacing(2),
    )
    .padding(12)
    .height(Length::Fixed(78.0))
    .width(Length::FillPortion(1));

    let outlined_card = material::widget::card::outlined(
        column![
            text("Outlined")
                .size(title_medium.size)
                .line_height(type_scale_line_height(title_medium)),
            text("1px stroke")
                .size(body_medium.size)
                .line_height(type_scale_line_height(body_medium)),
        ]
        .spacing(2),
    )
    .padding(12)
    .height(Length::Fixed(78.0))
    .width(Length::FillPortion(1));

    let cards = row![elevated_card, filled_card, outlined_card]
        .spacing(8)
        .width(Length::Fill);

    let lists = column![
        material::widget::list::one_line_with_leading_icon("*", "One-line list item"),
        material::widget::list::two_line_with_trailing("Messages", "Supporting text", "24"),
        material::widget::list::three_line("Three-line item", "Supporting text", "Second line"),
    ]
    .spacing(0)
    .width(Length::Fill);

    let data_table = material::widget::data_table::standard(
        [
            material::widget::data_table::column("Component", |row: InventoryRow| row.component)
                .width(Length::FillPortion(2)),
            material::widget::data_table::column("State", |row: InventoryRow| row.status),
            material::widget::data_table::numeric_column("Count", |row: InventoryRow| {
                row.count.to_string()
            })
            .width(Length::Fixed(88.0)),
        ],
        INVENTORY_ROWS,
    )
    .width(Length::Fill);

    let fields = column![
        controls, input, editor, select, combo_box, dividers, data_table
    ]
        .spacing(20)
        .width(Length::Fill);

    let components = column![progress, switches, radios, actions, chips, badges, cards, lists]
        .spacing(20)
        .width(Length::Fill);

    let panel = column![
        header,
        row![fields, components]
            .spacing(32)
            .align_y(Alignment::Start),
    ]
    .spacing(28)
    .padding(28)
    .max_width(980);

    material::widget::container::surface_container_high(panel)
        .center(Length::Fill)
        .into()
}

fn emphasized_decelerate(progress: f32) -> f32 {
    if progress <= 0.0 {
        return 0.0;
    }

    if progress >= 1.0 {
        return 1.0;
    }

    let easing = material::tokens::motion::EASING_EMPHASIZED_DECELERATE;

    cubic_bezier(progress, easing.x1, easing.y1, easing.x2, easing.y2)
}

fn cubic_bezier(progress: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let progress = progress.clamp(0.0, 1.0);
    let mut low = 0.0;
    let mut high = 1.0;

    for _ in 0..20 {
        let mid = (low + high) * 0.5;

        if bezier_axis(mid, x1, x2) < progress {
            low = mid;
        } else {
            high = mid;
        }
    }

    bezier_axis((low + high) * 0.5, y1, y2).clamp(0.0, 1.0)
}

fn bezier_axis(t: f32, p1: f32, p2: f32) -> f32 {
    let inverse = 1.0 - t;

    3.0 * inverse * inverse * t * p1 + 3.0 * inverse * t * t * p2 + t * t * t
}

fn lerp_color_scheme(from: ColorScheme, to: ColorScheme, amount: f32) -> ColorScheme {
    ColorScheme {
        primary: lerp_color_quartet(from.primary, to.primary, amount),
        secondary: lerp_color_quartet(from.secondary, to.secondary, amount),
        tertiary: lerp_color_quartet(from.tertiary, to.tertiary, amount),
        error: lerp_color_quartet(from.error, to.error, amount),
        surface: lerp_surface(from.surface, to.surface, amount),
        inverse: lerp_inverse(from.inverse, to.inverse, amount),
        outline: lerp_outline(from.outline, to.outline, amount),
        shadow: lerp_color(from.shadow, to.shadow, amount),
        scrim: lerp_color(from.scrim, to.scrim, amount),
    }
}

fn lerp_color_quartet(from: ColorQuartet, to: ColorQuartet, amount: f32) -> ColorQuartet {
    ColorQuartet {
        color: lerp_color(from.color, to.color, amount),
        text: lerp_color(from.text, to.text, amount),
        container: lerp_color(from.container, to.container, amount),
        container_text: lerp_color(from.container_text, to.container_text, amount),
    }
}

fn lerp_surface(from: Surface, to: Surface, amount: f32) -> Surface {
    Surface {
        color: lerp_color(from.color, to.color, amount),
        text: lerp_color(from.text, to.text, amount),
        text_variant: lerp_color(from.text_variant, to.text_variant, amount),
        container: lerp_surface_container(from.container, to.container, amount),
    }
}

fn lerp_surface_container(
    from: SurfaceContainer,
    to: SurfaceContainer,
    amount: f32,
) -> SurfaceContainer {
    SurfaceContainer {
        lowest: lerp_color(from.lowest, to.lowest, amount),
        low: lerp_color(from.low, to.low, amount),
        base: lerp_color(from.base, to.base, amount),
        high: lerp_color(from.high, to.high, amount),
        highest: lerp_color(from.highest, to.highest, amount),
    }
}

fn lerp_inverse(from: Inverse, to: Inverse, amount: f32) -> Inverse {
    Inverse {
        inverse_surface: lerp_color(from.inverse_surface, to.inverse_surface, amount),
        inverse_surface_text: lerp_color(
            from.inverse_surface_text,
            to.inverse_surface_text,
            amount,
        ),
        inverse_primary: lerp_color(from.inverse_primary, to.inverse_primary, amount),
    }
}

fn lerp_outline(from: Outline, to: Outline, amount: f32) -> Outline {
    Outline {
        color: lerp_color(from.color, to.color, amount),
        variant: lerp_color(from.variant, to.variant, amount),
    }
}

fn lerp_color(from: Color, to: Color, amount: f32) -> Color {
    let amount = amount.clamp(0.0, 1.0);

    Color {
        r: lerp_component(from.r, to.r, amount),
        g: lerp_component(from.g, to.g, amount),
        b: lerp_component(from.b, to.b, amount),
        a: lerp_component(from.a, to.a, amount),
    }
}

fn lerp_component(from: f32, to: f32, amount: f32) -> f32 {
    from + (to - from) * amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emphasized_decelerate_has_expected_endpoints() {
        assert_eq!(emphasized_decelerate(0.0), 0.0);
        assert!((emphasized_decelerate(1.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn theme_lerp_reaches_target_color_scheme() {
        let target = Theme::Light.colors();

        assert_eq!(
            lerp_color_scheme(Theme::Dark.colors(), target, 1.0)
                .surface
                .color,
            target.surface.color
        );
    }

    #[test]
    fn combo_input_preserves_typed_query_and_clears_stale_selection() {
        let mut demo = Demo::default();

        update(&mut demo, Message::ComboInputChanged("xxx".into()));

        assert_eq!(demo.combo_choice, None);
        assert_eq!(demo.combo_input, "xxx");

        update(&mut demo, Message::ComboSelected("Assist"));

        assert_eq!(demo.combo_choice, Some("Assist"));
        assert_eq!(demo.combo_input, "");
    }
}
