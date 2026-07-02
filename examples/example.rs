use iced::time::Instant;
use iced::widget::{column, row, scrollable, text};
use iced::{Alignment, Element, Length, Size, Subscription, window};
use iced_material as material;
use material::{ColorScheme, Theme, animation::ColorSchemeTransition};

pub fn main() -> iced::Result {
    let window_size = Size::new(1080.0, 980.0);

    iced::application(Demo::default, update, view)
        .title("iced_material example")
        .font(material::fonts::ROBOTO_REGULAR_BYTES)
        .font(material::fonts::ROBOTO_MEDIUM_BYTES)
        .font(material::fonts::ROBOTO_BOLD_BYTES)
        .font(material::fonts::NOTO_SANS_CJK_SC_REGULAR_BYTES)
        .font(material::fonts::NOTO_SANS_CJK_SC_MEDIUM_BYTES)
        .font(material::fonts::NOTO_SANS_CJK_SC_BOLD_BYTES)
        .font(material::fonts::MATERIAL_SYMBOLS_ROUNDED_BYTES)
        .font(material::fonts::MATERIAL_SYMBOLS_ROUNDED_FILLED_BYTES)
        .default_font(material::fonts::ROBOTO)
        .subscription(subscription)
        .theme(theme)
        .window(window::Settings {
            size: window_size,
            min_size: Some(Size::new(420.0, 720.0)),
            position: window::Position::Centered,
            ..window::Settings::default()
        })
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Navigate(DemoPage),
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
    ToggleDrawer,
    WindowResized(Size),
    Frame(Instant),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DemoPage {
    Inputs,
    Controls,
    Feedback,
    Surfaces,
    Navigation,
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

const NAV_DESTINATIONS: [material::widget::navigation::Destination<DemoPage>; 5] = [
    material::widget::navigation::Destination::new(DemoPage::Inputs, "input", "Inputs"),
    material::widget::navigation::Destination::new(DemoPage::Controls, "tune", "Controls"),
    material::widget::navigation::Destination::new(DemoPage::Feedback, "info", "Feedback")
        .badge("3"),
    material::widget::navigation::Destination::new(DemoPage::Surfaces, "layers", "Surfaces")
        .small_badge(),
    material::widget::navigation::Destination::new(
        DemoPage::Navigation,
        "navigation",
        "Navigation",
    ),
];

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
    navigation: material::widget::navigation::NavigationState<DemoPage>,
    rail_expansion: material::widget::navigation::NavigationRailExpansionState,
    window_size: Size,
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
    animation: Option<ColorSchemeTransition>,
}

impl Default for Demo {
    fn default() -> Self {
        let initial_theme = Theme::Dark;

        Self {
            navigation: material::widget::navigation::NavigationState::new(DemoPage::Inputs),
            rail_expansion: material::widget::navigation::NavigationRailExpansionState::new(false),
            window_size: Size::new(1080.0, 980.0),
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

    fn navigation_selection(&self) -> material::widget::navigation::Selection<DemoPage> {
        self.navigation.selection()
    }

    fn adaptive_navigation_layout(&self) -> material::widget::navigation::AdaptiveLayout {
        material::widget::navigation::adaptive_layout(
            self.window_size.width,
            self.window_size.height,
        )
    }
}

fn update(state: &mut Demo, message: Message) {
    match message {
        Message::Navigate(page) => {
            state
                .navigation
                .select(page, Instant::now(), state.adaptive_navigation_layout());
        }
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
        Message::ToggleDrawer => state.rail_expansion.toggle(Instant::now()),
        Message::WindowResized(size) => state.window_size = size,
        Message::DarkModeChanged(dark_mode) => {
            state.dark_mode = dark_mode;

            let target = if dark_mode {
                Theme::Dark.colors()
            } else {
                Theme::Light.colors()
            };

            state.animation = Some(ColorSchemeTransition::material_theme(
                state.visible_scheme,
                target,
                Instant::now(),
            ));
        }
        Message::Frame(now) => {
            if let Some(animation) = state.animation {
                state.visible_scheme = animation.value_at(now);

                if animation.is_finished_at(now) {
                    state.animation = None;
                }
            }

            let _ = state.navigation.advance(now);
            let _ = state.rail_expansion.advance(now);
        }
    }
}

fn theme(state: &Demo) -> Theme {
    state.theme()
}

fn subscription(state: &Demo) -> Subscription<Message> {
    let mut subscriptions =
        vec![iced::window::resize_events().map(|(_id, size)| Message::WindowResized(size))];

    if state.animation.is_some()
        || state.navigation.is_animating()
        || state.rail_expansion.is_animating()
    {
        subscriptions.push(iced::window::frames().map(Message::Frame));
    }

    Subscription::batch(subscriptions)
}

fn view(state: &Demo) -> Element<'_, Message, Theme> {
    let selection = state.navigation_selection();

    match state.adaptive_navigation_layout() {
        material::widget::navigation::AdaptiveLayout::NavigationBar => column![
            page_content(state),
            material::widget::navigation::navigation_bar(
                &NAV_DESTINATIONS,
                selection,
                Message::Navigate,
            )
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into(),
        material::widget::navigation::AdaptiveLayout::NavigationRail => {
            let navigation: Element<'_, Message, Theme> = if state.rail_expansion.is_visible() {
                let drawer_width =
                    material::widget::navigation::navigation_rail_expanded_width_for_progress(
                        state.rail_expansion.progress(),
                    );

                if drawer_width > 0.0 {
                    material::widget::navigation::navigation_rail_expanded_with_menu_at_width(
                        "Example",
                        &NAV_DESTINATIONS,
                        selection,
                        Message::Navigate,
                        Message::ToggleDrawer,
                        drawer_width,
                    )
                    .into()
                } else {
                    material::widget::navigation::navigation_rail_with_menu(
                        &NAV_DESTINATIONS,
                        selection,
                        Message::Navigate,
                        Message::ToggleDrawer,
                    )
                    .into()
                }
            } else {
                material::widget::navigation::navigation_rail_with_menu(
                    &NAV_DESTINATIONS,
                    selection,
                    Message::Navigate,
                    Message::ToggleDrawer,
                )
                .into()
            };

            row![navigation, page_content(state)]
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
    }
}

fn page_content(state: &Demo) -> Element<'_, Message, Theme> {
    let page = state.navigation.selected();
    let content = match page {
        DemoPage::Inputs => inputs_page(state),
        DemoPage::Controls => controls_page(state),
        DemoPage::Feedback => feedback_page(state),
        DemoPage::Surfaces => surfaces_page(state),
        DemoPage::Navigation => navigation_page(state),
    };

    let page = column![header(page), content]
        .spacing(28)
        .padding(28)
        .width(Length::Fill)
        .max_width(980);

    scrollable(
        material::widget::container::surface_container_high(page)
            .width(Length::Fill)
            .center_x(Length::Fill),
    )
    .height(Length::Fill)
    .into()
}

fn header(page: DemoPage) -> Element<'static, Message, Theme> {
    let body_large = material::tokens::typography::BODY_LARGE;
    let headline_large = material::tokens::typography::HEADLINE_LARGE;
    let chinese_sample = "中文字体 Noto Sans CJK";

    column![
        text("iced_material 0.14.2")
            .size(headline_large.size)
            .line_height(material::text::line_height(headline_large)),
        text(page_label(page))
            .size(body_large.size)
            .line_height(material::text::line_height(body_large)),
        text(chinese_sample)
            .font(material::fonts::font_for_content_type_scale(
                chinese_sample,
                body_large,
            ))
            .size(body_large.size)
            .line_height(material::text::line_height(body_large)),
    ]
    .spacing(6)
    .into()
}

fn inputs_page(state: &Demo) -> Element<'_, Message, Theme> {
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

    column![
        section("Text fields", column![input, editor].spacing(16).into()),
        material::widget::rule::horizontal_inset(),
        section(
            "Selection fields",
            column![select, combo_box].spacing(16).into()
        ),
        material::widget::rule::horizontal_inset(),
        section("Dividers", dividers()),
    ]
    .spacing(24)
    .width(Length::Fill)
    .into()
}

fn controls_page(state: &Demo) -> Element<'_, Message, Theme> {
    column![
        section("Counter", counter_controls(state)),
        material::widget::rule::horizontal_inset(),
        section("Actions", action_buttons(state)),
        material::widget::rule::horizontal_inset(),
        section("Chips", chips(state)),
        material::widget::rule::horizontal_inset(),
        section("Selection controls", selection_controls(state)),
    ]
    .spacing(24)
    .width(Length::Fill)
    .into()
}

fn feedback_page(state: &Demo) -> Element<'_, Message, Theme> {
    column![
        section("Progress", progress_indicators(state)),
        material::widget::rule::horizontal_inset(),
        section("Badges", badges()),
        material::widget::rule::horizontal_inset(),
        section(
            "Tooltip",
            material::widget::tooltip::plain(
                material::widget::button::assist_chip("Hint")
                    .on_press_maybe(state.enabled.then_some(Message::Increment)),
                "Material 3 plain tooltip",
                material::widget::tooltip::Position::Top,
            )
            .into(),
        ),
    ]
    .spacing(24)
    .width(Length::Fill)
    .into()
}

fn surfaces_page(_state: &Demo) -> Element<'static, Message, Theme> {
    column![
        section("Cards", cards()),
        material::widget::rule::horizontal_inset(),
        section("Lists", lists()),
        material::widget::rule::horizontal_inset(),
        section("Data table", data_table()),
    ]
    .spacing(24)
    .width(Length::Fill)
    .into()
}

fn navigation_page(state: &Demo) -> Element<'_, Message, Theme> {
    let selection = state.navigation_selection();
    let rail_height = navigation_demo_rail_height();
    let bar = material::widget::navigation::navigation_bar(
        &NAV_DESTINATIONS,
        selection,
        Message::Navigate,
    );
    let rail = material::widget::navigation::navigation_rail_with_menu(
        &NAV_DESTINATIONS,
        selection,
        Message::Navigate,
        Message::ToggleDrawer,
    )
    .height(Length::Fixed(rail_height));
    let expanded_rail = material::widget::navigation::navigation_rail_expanded_with_menu(
        "Example",
        &NAV_DESTINATIONS,
        selection,
        Message::Navigate,
        Message::ToggleDrawer,
    )
    .height(Length::Fixed(rail_height));

    column![
        section("Navigation bar", bar.into()),
        material::widget::rule::horizontal_inset(),
        section("Navigation rail with menu", rail.into()),
        material::widget::rule::horizontal_inset(),
        section("Expanded navigation rail", expanded_rail.into()),
    ]
    .spacing(24)
    .width(Length::Fill)
    .into()
}

fn navigation_demo_rail_height() -> f32 {
    material::widget::navigation::navigation_rail_min_height(NAV_DESTINATIONS.len(), true)
}

fn section<'a>(
    title: &'static str,
    body: Element<'a, Message, Theme>,
) -> Element<'a, Message, Theme> {
    let title_medium = material::tokens::typography::TITLE_MEDIUM;

    column![
        text(title)
            .size(title_medium.size)
            .line_height(material::text::line_height(title_medium)),
        body
    ]
    .spacing(12)
    .width(Length::Fill)
    .into()
}

fn counter_controls(state: &Demo) -> Element<'_, Message, Theme> {
    let headline_medium = material::tokens::typography::HEADLINE_MEDIUM;

    row![
        material::widget::button::outlined("Minus").on_press(Message::Decrement),
        text(state.count)
            .size(headline_medium.size)
            .line_height(material::text::line_height(headline_medium)),
        material::widget::button::filled("Plus").on_press(Message::Increment),
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .into()
}

fn action_buttons(state: &Demo) -> Element<'_, Message, Theme> {
    row![
        material::widget::button::filled("Filled")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
        material::widget::button::filled_tonal("Tonal")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
        material::widget::button::text("Text")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
        material::widget::button::primary_fab("+")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .into()
}

fn chips(state: &Demo) -> Element<'_, Message, Theme> {
    row![
        material::widget::button::assist_chip("Assist")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
        material::widget::button::suggestion_chip("Suggestion")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
        material::widget::button::filter_chip("Filter")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
        material::widget::button::selected_filter_chip("Selected")
            .on_press_maybe(state.enabled.then_some(Message::Increment)),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn selection_controls(state: &Demo) -> Element<'_, Message, Theme> {
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

    column![switches, radios].spacing(18).into()
}

fn progress_indicators(state: &Demo) -> Element<'_, Message, Theme> {
    let body_large = material::tokens::typography::BODY_LARGE;

    column![
        row![
            text("Progress")
                .size(body_large.size)
                .line_height(material::text::line_height(body_large))
                .width(Length::Fill),
            text(format!("{:.0}%", state.progress))
                .size(body_large.size)
                .line_height(material::text::line_height(body_large)),
        ]
        .spacing(12),
        material::widget::slider::continuous(0.0..=100.0, state.progress, Message::SliderChanged)
            .step(1.0),
        material::widget::progress_bar::linear(0.0..=100.0, state.progress),
    ]
    .spacing(10)
    .into()
}

fn badges() -> Element<'static, Message, Theme> {
    let body_large = material::tokens::typography::BODY_LARGE;

    row![
        text("Badges")
            .size(body_large.size)
            .line_height(material::text::line_height(body_large)),
        material::widget::badge::small(),
        material::widget::badge::large("3"),
        material::widget::badge::large("99+"),
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .into()
}

fn cards() -> Element<'static, Message, Theme> {
    let body_medium = material::tokens::typography::BODY_MEDIUM;
    let title_medium = material::tokens::typography::TITLE_MEDIUM;

    let elevated_card = material::widget::card::elevated(
        column![
            text("Elevated")
                .size(title_medium.size)
                .line_height(material::text::line_height(title_medium)),
            text("Level 1")
                .size(body_medium.size)
                .line_height(material::text::line_height(body_medium)),
        ]
        .spacing(2),
    )
    .padding(12)
    .height(Length::Fixed(78.0))
    .width(Length::Fill);

    let filled_card = material::widget::card::filled(
        column![
            text("Filled")
                .size(title_medium.size)
                .line_height(material::text::line_height(title_medium)),
            text("Container")
                .size(body_medium.size)
                .line_height(material::text::line_height(body_medium)),
        ]
        .spacing(2),
    )
    .padding(12)
    .height(Length::Fixed(78.0))
    .width(Length::Fill);

    let outlined_card = material::widget::card::outlined(
        column![
            text("Outlined")
                .size(title_medium.size)
                .line_height(material::text::line_height(title_medium)),
            text("1px stroke")
                .size(body_medium.size)
                .line_height(material::text::line_height(body_medium)),
        ]
        .spacing(2),
    )
    .padding(12)
    .height(Length::Fixed(78.0))
    .width(Length::Fill);

    column![elevated_card, filled_card, outlined_card]
        .spacing(8)
        .width(Length::Fill)
        .into()
}

fn lists() -> Element<'static, Message, Theme> {
    column![
        material::widget::list::one_line_with_leading_icon("*", "One-line list item"),
        material::widget::list::two_line_with_trailing("Messages", "Supporting text", "24"),
        material::widget::list::three_line("Three-line item", "Supporting text", "Second line"),
    ]
    .spacing(0)
    .width(Length::Fill)
    .into()
}

fn dividers() -> Element<'static, Message, Theme> {
    let body_large = material::tokens::typography::BODY_LARGE;

    column![
        material::widget::rule::horizontal_full_width(),
        row![
            text("Full")
                .size(body_large.size)
                .line_height(material::text::line_height(body_large)),
            material::widget::rule::vertical_full_height(),
            text("Inset")
                .size(body_large.size)
                .line_height(material::text::line_height(body_large)),
        ]
        .height(Length::Fixed(32.0))
        .spacing(16)
        .align_y(Alignment::Center),
        material::widget::rule::horizontal_inset(),
    ]
    .spacing(8)
    .into()
}

fn data_table() -> Element<'static, Message, Theme> {
    material::widget::data_table::standard(
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
    .width(Length::Fill)
    .into()
}

fn page_label(page: DemoPage) -> &'static str {
    match page {
        DemoPage::Inputs => "Inputs",
        DemoPage::Controls => "Controls",
        DemoPage::Feedback => "Feedback",
        DemoPage::Surfaces => "Surfaces",
        DemoPage::Navigation => "Navigation",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn navigation_starts_selection_animation() {
        let mut demo = Demo::default();

        update(&mut demo, Message::Navigate(DemoPage::Controls));

        assert_eq!(demo.navigation.selected(), DemoPage::Controls);
        assert!(demo.navigation.is_animating());
        assert_eq!(
            demo.navigation.selection().progress(DemoPage::Controls),
            0.0
        );
        assert_eq!(demo.navigation.selection().progress(DemoPage::Inputs), 1.0);
    }

    #[test]
    fn menu_toggles_expanded_navigation_rail() {
        let mut demo = Demo::default();

        update(&mut demo, Message::ToggleDrawer);

        assert!(demo.rail_expansion.is_open());
        assert!(demo.rail_expansion.is_animating());

        update(
            &mut demo,
            Message::Frame(Instant::now() + iced::time::Duration::from_millis(500)),
        );

        assert!(demo.rail_expansion.is_visible());

        update(&mut demo, Message::ToggleDrawer);

        assert!(!demo.rail_expansion.is_open());
    }

    #[test]
    fn navigation_uses_material_symbol_icon_names() {
        assert_eq!(material::fonts::all().len(), 8);
        assert_eq!(
            NAV_DESTINATIONS.map(|destination| destination.icon),
            ["input", "tune", "info", "layers", "navigation"]
        );

        for destination in NAV_DESTINATIONS {
            assert!(material::fonts::material_symbol_codepoint(destination.icon).is_some());
        }
    }

    #[test]
    fn navigation_demo_rail_height_fits_all_destinations() {
        assert_eq!(navigation_demo_rail_height(), 468.0);
        assert!(
            navigation_demo_rail_height()
                > material::widget::navigation::navigation_rail_min_height(4, true)
        );
    }

    #[test]
    fn chinese_sample_uses_bundled_noto_sans_cjk() {
        assert_eq!(
            material::fonts::font_for_content_type_scale(
                "中文字体 Noto Sans CJK",
                material::tokens::typography::BODY_LARGE,
            ),
            material::fonts::NOTO_SANS_CJK_SC
        );
    }

    #[test]
    fn resize_updates_adaptive_layout_inputs() {
        let mut demo = Demo::default();

        update(&mut demo, Message::WindowResized(Size::new(500.0, 900.0)));

        assert_eq!(
            demo.adaptive_navigation_layout(),
            material::widget::navigation::AdaptiveLayout::NavigationBar
        );

        update(&mut demo, Message::WindowResized(Size::new(900.0, 900.0)));

        assert_eq!(
            demo.adaptive_navigation_layout(),
            material::widget::navigation::AdaptiveLayout::NavigationRail
        );
    }
}
