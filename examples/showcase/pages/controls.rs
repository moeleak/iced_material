use material::widget::page;
use material_ui_rs as material;

use super::super::{Message, RadioChoice, SegmentChoice, Showcase};

pub(super) fn view(state: &Showcase) -> material::Element<'_, Message> {
    page::sections([
        page::section("Counter", counter_controls(state)).into(),
        page::section("Actions", action_buttons(state)).into(),
        page::section("FABs", fabs(state)).into(),
        page::section("Chips", chips(state)).into(),
        page::section("Segmented buttons", segmented_buttons(state)).into(),
        page::section("Selection controls", selection_controls(state)).into(),
    ])
    .into()
}

fn counter_controls(state: &Showcase) -> material::Element<'_, Message> {
    use material::widget::button::{self, ButtonVariant};

    page::row([
        button::action(
            button::button("Minus", ButtonVariant::Outlined),
            Message::Decrement,
        ),
        material::text::headline_medium(state.count.to_string()).into(),
        button::action(
            button::button("Plus", ButtonVariant::Filled),
            Message::Increment,
        ),
    ])
    .into()
}

fn action_buttons(state: &Showcase) -> material::Element<'_, Message> {
    use material::widget::button::{self, ButtonVariant};

    page::row(button::enabled_actions(
        state.enabled,
        Message::Increment,
        [
            button::button("Filled", ButtonVariant::Filled),
            button::button("Tonal", ButtonVariant::FilledTonal),
            button::button("Text", ButtonVariant::Text),
        ],
    ))
    .into()
}

fn fabs(state: &Showcase) -> material::Element<'_, Message> {
    use material::widget::button::{self, FabSize, FabVariant};

    page::stack([
        page::row(button::enabled_actions(
            state.enabled,
            Message::Increment,
            [
                button::fab("add", FabVariant::Surface, FabSize::Small),
                button::fab("add", FabVariant::Surface, FabSize::Standard),
                button::fab("add", FabVariant::Surface, FabSize::Large),
                button::fab("add", FabVariant::Primary, FabSize::Standard),
                button::fab("add", FabVariant::Secondary, FabSize::Standard),
                button::fab("add", FabVariant::Tertiary, FabSize::Standard),
            ],
        ))
        .into(),
        page::row(button::enabled_actions(
            state.enabled,
            Message::Increment,
            [
                button::extended_fab_with_icon("add", "Create", FabVariant::Primary),
                button::extended_fab_with_icon("share", "Share", FabVariant::Secondary),
                button::extended_fab_with_icon("add", "Add", FabVariant::Tertiary),
                button::extended_fab("Reroute", FabVariant::Surface),
            ],
        ))
        .into(),
    ])
    .into()
}

fn chips(state: &Showcase) -> material::Element<'_, Message> {
    use material::widget::button::{self, ChipVariant};

    page::compact_row(button::enabled_actions(
        state.enabled,
        Message::Increment,
        [
            button::chip("Assist", ChipVariant::Assist),
            button::chip("Suggestion", ChipVariant::Suggestion),
            button::chip("Filter", ChipVariant::Filter),
            button::chip("Selected", ChipVariant::SelectedFilter),
        ],
    ))
    .into()
}

fn segmented_buttons(state: &Showcase) -> material::Element<'_, Message> {
    use material::widget::segmented_button;

    segmented_button::group(segmented_button::animated_selectable_label_actions(
        &state.segment_state,
        [
            ("List", Message::SegmentSelected(SegmentChoice::List)),
            ("Grid", Message::SegmentSelected(SegmentChoice::Grid)),
            ("Map", Message::SegmentSelected(SegmentChoice::Map)),
        ],
    ))
    .into()
}

fn selection_controls(state: &Showcase) -> material::Element<'_, Message> {
    let switches = page::component_stack([
        material::widget::checkbox::standard(
            state.enabled,
            "Enable actions",
            Message::EnabledChanged,
        ),
        state
            .theme_controller
            .dark_mode_switch("Dark theme", Message::ThemeChanged),
    ]);

    let radios = page::row([
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
    ]);

    page::spacious_stack([switches.into(), radios.into()]).into()
}
