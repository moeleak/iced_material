#[path = "pages/mod.rs"]
mod pages;

use iced::time::Instant;
use iced::{Size, Subscription};
use iced_material as material;
use material::widget::navigation;
use material::{ColorScheme, Theme, animation::ColorSchemeTransition};

pub fn main() -> iced::Result {
    let window_size = Size::new(1080.0, 980.0);

    material::application(Showcase::default, update, view)
        .title("iced_material showcase")
        .subscription(subscription)
        .theme(theme)
        .window(material::window_with_min_size(
            window_size,
            Size::new(420.0, 720.0),
        ))
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Navigate(ShowcasePage),
    Increment,
    Decrement,
    TextChanged(String),
    EditorAction(material::widget::text_editor::Action),
    SelectChanged(&'static str),
    ComboSelected(&'static str),
    ComboInputChanged(String),
    SearchChanged(String),
    SliderChanged(f32),
    EnabledChanged(bool),
    DarkModeChanged(bool),
    ChoiceSelected(RadioChoice),
    SegmentSelected(SegmentChoice),
    PrimaryTabSelected(TabChoice),
    SecondaryTabSelected(TabChoice),
    MenuPressed,
    DialogOpened,
    DialogDismissed,
    DialogConfirmed,
    ShowSnackbar,
    SnackbarUndo,
    WindowResized(Size),
    Frame(Instant),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShowcasePage {
    Inputs,
    Controls,
    Feedback,
    Surfaces,
    Navigation,
    Structure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RadioChoice {
    Standard,
    Expressive,
    Dense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SegmentChoice {
    List,
    Grid,
    Map,
}

impl SegmentChoice {
    const fn index(self) -> usize {
        match self {
            Self::List => 0,
            Self::Grid => 1,
            Self::Map => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TabChoice {
    Inputs,
    Controls,
    Feedback,
}

impl TabChoice {
    const fn index(self) -> usize {
        match self {
            Self::Inputs => 0,
            Self::Controls => 1,
            Self::Feedback => 2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct InventoryRow {
    component: &'static str,
    status: &'static str,
    count: u32,
}

const NAV_DESTINATIONS: [navigation::Destination<ShowcasePage>; 6] = [
    navigation::Destination::new(ShowcasePage::Inputs, "input", "Inputs"),
    navigation::Destination::new(ShowcasePage::Controls, "tune", "Controls"),
    navigation::Destination::new(ShowcasePage::Feedback, "info", "Feedback").badge("3"),
    navigation::Destination::new(ShowcasePage::Surfaces, "layers", "Surfaces").small_badge(),
    navigation::Destination::new(ShowcasePage::Navigation, "navigation", "Navigation"),
    navigation::Destination::new(ShowcasePage::Structure, "layers", "Structure"),
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
struct Showcase {
    navigation: navigation::NavigationState<ShowcasePage>,
    window_size: Size,
    count: i32,
    note: String,
    editor_content: material::widget::text_editor::Content,
    select_choice: Option<&'static str>,
    combo_options: material::widget::combo_box::State<&'static str>,
    combo_choice: Option<&'static str>,
    combo_input: String,
    search_query: String,
    progress: f32,
    enabled: bool,
    dark_mode: bool,
    radio_choice: Option<RadioChoice>,
    segment_choice: SegmentChoice,
    segment_state: material::widget::segmented_button::State,
    primary_tab: TabChoice,
    primary_tab_state: material::widget::tabs::State,
    secondary_tab: TabChoice,
    secondary_tab_state: material::widget::tabs::State,
    progress_animation: material::widget::progress_bar::IndeterminateState,
    alert_dialog_open: bool,
    snackbar: material::widget::snackbar::Transition,
    visible_scheme: ColorScheme,
    animation: Option<ColorSchemeTransition>,
}

impl Default for Showcase {
    fn default() -> Self {
        let initial_theme = Theme::Dark;

        Self {
            navigation: navigation::NavigationState::new(ShowcasePage::Inputs),
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
            search_query: String::new(),
            progress: 42.0,
            enabled: true,
            dark_mode: true,
            radio_choice: Some(RadioChoice::Standard),
            segment_choice: SegmentChoice::List,
            segment_state: material::widget::segmented_button::State::new(
                SegmentChoice::List.index(),
            ),
            primary_tab: TabChoice::Inputs,
            primary_tab_state: material::widget::tabs::State::new(TabChoice::Inputs.index()),
            secondary_tab: TabChoice::Controls,
            secondary_tab_state: material::widget::tabs::State::new(TabChoice::Controls.index()),
            progress_animation: material::widget::progress_bar::IndeterminateState::new(
                Instant::now(),
            ),
            alert_dialog_open: false,
            snackbar: material::widget::snackbar::Transition::default(),
            visible_scheme: initial_theme.colors(),
            animation: None,
        }
    }
}

impl Showcase {
    fn theme(&self) -> Theme {
        Theme::new("Material 3 animated", self.visible_scheme)
    }

    fn navigation_selection(&self) -> navigation::Selection<ShowcasePage> {
        self.navigation.selection()
    }

    fn adaptive_navigation_layout(&self) -> navigation::AdaptiveLayout {
        navigation::adaptive_layout(self.window_size.width, self.window_size.height)
    }
}

fn update(state: &mut Showcase, message: Message) {
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
        Message::SearchChanged(query) => state.search_query = query,
        Message::SliderChanged(progress) => state.progress = progress,
        Message::EnabledChanged(enabled) => state.enabled = enabled,
        Message::ChoiceSelected(choice) => state.radio_choice = Some(choice),
        Message::SegmentSelected(choice) => {
            state.segment_choice = choice;
            state.segment_state.select(choice.index(), Instant::now());
        }
        Message::PrimaryTabSelected(choice) => {
            state.primary_tab = choice;
            state.primary_tab_state.select(
                choice.index(),
                Instant::now(),
                material::widget::tabs::Variant::Primary,
            );
        }
        Message::SecondaryTabSelected(choice) => {
            state.secondary_tab = choice;
            state.secondary_tab_state.select(
                choice.index(),
                Instant::now(),
                material::widget::tabs::Variant::Secondary,
            );
        }
        Message::MenuPressed => state.navigation.toggle_menu_now(),
        Message::DialogOpened => state.alert_dialog_open = true,
        Message::DialogDismissed => state.alert_dialog_open = false,
        Message::DialogConfirmed => {
            state.alert_dialog_open = false;
            state.count += 1;
        }
        Message::ShowSnackbar => state.snackbar.show(Instant::now()),
        Message::SnackbarUndo => {
            state.count -= 1;
            state.snackbar.dismiss(Instant::now());
        }
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
            let _ = state.segment_state.advance(now);
            let _ = state.primary_tab_state.advance(now);
            let _ = state.secondary_tab_state.advance(now);
            state.progress_animation.advance(now);
            let _ = state.snackbar.advance(now);
        }
    }
}

fn theme(state: &Showcase) -> Theme {
    state.theme()
}

fn subscription(state: &Showcase) -> Subscription<Message> {
    let mut subscriptions =
        vec![iced::window::resize_events().map(|(_id, size)| Message::WindowResized(size))];

    if state.animation.is_some()
        || state.navigation.is_animating()
        || state.segment_state.is_animating()
        || state.primary_tab_state.is_animating()
        || state.secondary_tab_state.is_animating()
        || state.snackbar.is_active()
        || (state.navigation.selected() == ShowcasePage::Feedback
            && state.progress_animation.is_animating())
    {
        subscriptions.push(iced::window::frames().map(Message::Frame));
    }

    Subscription::batch(subscriptions)
}

fn view(state: &Showcase) -> material::Element<'_, Message> {
    let page_content = material::widget::snackbar::host_single_line_with_action(
        pages::view(state),
        &state.snackbar,
        Instant::now(),
        "Photo archived",
        "Undo",
        Message::SnackbarUndo,
    );

    let content = navigation::suite(&NAV_DESTINATIONS, &state.navigation)
        .layout(state.adaptive_navigation_layout())
        .with_menu("Showcase", Message::MenuPressed)
        .view(Message::Navigate, page_content);

    if state.alert_dialog_open {
        material::widget::dialog::modal(content, alert_dialog())
    } else {
        content
    }
}

fn alert_dialog() -> material::Element<'static, Message> {
    material::widget::dialog::alert_with_icon(
        "info",
        "Discard draft?",
        "Your current changes will be removed from this device.",
        material::widget::dialog::actions([
            material::widget::dialog::action_button("Cancel", Message::DialogDismissed),
            material::widget::dialog::action_button("Discard", Message::DialogConfirmed),
        ]),
    )
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combo_input_preserves_typed_query_and_clears_stale_selection() {
        let mut showcase = Showcase::default();

        update(&mut showcase, Message::ComboInputChanged("xxx".into()));

        assert_eq!(showcase.combo_choice, None);
        assert_eq!(showcase.combo_input, "xxx");

        update(&mut showcase, Message::ComboSelected("Assist"));

        assert_eq!(showcase.combo_choice, Some("Assist"));
        assert_eq!(showcase.combo_input, "");
    }

    #[test]
    fn navigation_starts_selection_animation() {
        let mut showcase = Showcase::default();

        update(&mut showcase, Message::Navigate(ShowcasePage::Controls));

        assert_eq!(showcase.navigation.selected(), ShowcasePage::Controls);
        assert!(showcase.navigation.is_animating());
        assert_eq!(
            showcase
                .navigation
                .selection()
                .progress(ShowcasePage::Controls),
            0.0
        );
        assert_eq!(
            showcase
                .navigation
                .selection()
                .progress(ShowcasePage::Inputs),
            1.0
        );
    }

    #[test]
    fn alert_dialog_messages_toggle_modal_state() {
        let mut showcase = Showcase::default();

        update(&mut showcase, Message::DialogOpened);
        assert!(showcase.alert_dialog_open);

        update(&mut showcase, Message::DialogDismissed);
        assert!(!showcase.alert_dialog_open);

        update(&mut showcase, Message::DialogOpened);
        update(&mut showcase, Message::DialogConfirmed);
        assert!(!showcase.alert_dialog_open);
        assert_eq!(showcase.count, 1);
    }

    #[test]
    fn snackbar_button_starts_android_transition() {
        let mut showcase = Showcase::default();

        update(&mut showcase, Message::ShowSnackbar);

        assert_eq!(
            showcase.snackbar.phase(),
            material::widget::snackbar::TransitionPhase::Showing
        );
        assert!(showcase.snackbar.is_active());
    }

    #[test]
    fn snackbar_action_dismisses_with_exit_transition() {
        let mut showcase = Showcase::default();

        update(&mut showcase, Message::ShowSnackbar);
        update(&mut showcase, Message::SnackbarUndo);

        assert_eq!(showcase.count, -1);
        assert_eq!(
            showcase.snackbar.phase(),
            material::widget::snackbar::TransitionPhase::Dismissing
        );
    }

    #[test]
    fn navigation_uses_material_symbol_icon_names() {
        assert_eq!(material::fonts::all().len(), 5);
        assert_eq!(
            NAV_DESTINATIONS.map(|destination| destination.icon),
            ["input", "tune", "info", "layers", "navigation", "layers"]
        );

        for destination in NAV_DESTINATIONS {
            assert!(material::fonts::material_symbol_codepoint(destination.icon).is_some());
        }
    }

    #[test]
    fn resize_updates_adaptive_layout_inputs() {
        let mut showcase = Showcase::default();

        update(
            &mut showcase,
            Message::WindowResized(Size::new(500.0, 900.0)),
        );

        assert_eq!(
            showcase.adaptive_navigation_layout(),
            material::widget::navigation::AdaptiveLayout::NavigationBar
        );

        update(
            &mut showcase,
            Message::WindowResized(Size::new(900.0, 900.0)),
        );

        assert_eq!(
            showcase.adaptive_navigation_layout(),
            material::widget::navigation::AdaptiveLayout::NavigationRail
        );
    }
}
