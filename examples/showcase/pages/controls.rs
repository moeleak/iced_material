use iced_material as material;
use material::widget::page;

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
    page::row([
        material::widget::button::outlined_action("Minus", Message::Decrement),
        material::text::headline_medium(state.count.to_string()).into(),
        material::widget::button::filled_action("Plus", Message::Increment),
    ])
    .into()
}

fn action_buttons(state: &Showcase) -> material::Element<'_, Message> {
    page::row([
        material::widget::button::filled("Filled")
            .on_press_maybe(state.enabled.then_some(Message::Increment))
            .into(),
        material::widget::button::filled_tonal("Tonal")
            .on_press_maybe(state.enabled.then_some(Message::Increment))
            .into(),
        material::widget::button::text("Text")
            .on_press_maybe(state.enabled.then_some(Message::Increment))
            .into(),
    ])
    .into()
}

fn fabs(state: &Showcase) -> material::Element<'_, Message> {
    page::stack([
        page::row([
            material::widget::button::surface_small_fab("+")
                .on_press_maybe(state.enabled.then_some(Message::Increment))
                .into(),
            material::widget::button::surface_fab("+")
                .on_press_maybe(state.enabled.then_some(Message::Increment))
                .into(),
            material::widget::button::surface_large_fab("+")
                .on_press_maybe(state.enabled.then_some(Message::Increment))
                .into(),
            material::widget::button::primary_fab("+")
                .on_press_maybe(state.enabled.then_some(Message::Increment))
                .into(),
            material::widget::button::secondary_fab("+")
                .on_press_maybe(state.enabled.then_some(Message::Increment))
                .into(),
            material::widget::button::tertiary_fab("+")
                .on_press_maybe(state.enabled.then_some(Message::Increment))
                .into(),
        ])
        .into(),
        page::row([
            material::widget::button::primary_extended_fab_with_icon("+", "Create")
                .on_press_maybe(state.enabled.then_some(Message::Increment))
                .into(),
            material::widget::button::secondary_extended_fab_with_icon("+", "Share")
                .on_press_maybe(state.enabled.then_some(Message::Increment))
                .into(),
            material::widget::button::tertiary_extended_fab_with_icon("+", "Add")
                .on_press_maybe(state.enabled.then_some(Message::Increment))
                .into(),
            material::widget::button::surface_extended_fab("Reroute")
                .on_press_maybe(state.enabled.then_some(Message::Increment))
                .into(),
        ])
        .into(),
    ])
    .into()
}

fn chips(state: &Showcase) -> material::Element<'_, Message> {
    page::compact_row([
        material::widget::button::assist_chip("Assist")
            .on_press_maybe(state.enabled.then_some(Message::Increment))
            .into(),
        material::widget::button::suggestion_chip("Suggestion")
            .on_press_maybe(state.enabled.then_some(Message::Increment))
            .into(),
        material::widget::button::filter_chip("Filter")
            .on_press_maybe(state.enabled.then_some(Message::Increment))
            .into(),
        material::widget::button::selected_filter_chip("Selected")
            .on_press_maybe(state.enabled.then_some(Message::Increment))
            .into(),
    ])
    .into()
}

fn segmented_buttons(state: &Showcase) -> material::Element<'_, Message> {
    use material::widget::segmented_button::{self, SegmentPosition};

    segmented_button::group([
        segmented_button::animated_selectable_label(
            "List",
            state
                .segment_state
                .progress_for(SegmentChoice::List.index()),
            SegmentPosition::First,
        )
        .on_press(Message::SegmentSelected(SegmentChoice::List))
        .into(),
        segmented_button::animated_selectable_label(
            "Grid",
            state
                .segment_state
                .progress_for(SegmentChoice::Grid.index()),
            SegmentPosition::Middle,
        )
        .on_press(Message::SegmentSelected(SegmentChoice::Grid))
        .into(),
        segmented_button::animated_selectable_label(
            "Map",
            state.segment_state.progress_for(SegmentChoice::Map.index()),
            SegmentPosition::Last,
        )
        .on_press(Message::SegmentSelected(SegmentChoice::Map))
        .into(),
    ])
    .into()
}

fn selection_controls(state: &Showcase) -> material::Element<'_, Message> {
    let switches = page::stack([
        material::widget::checkbox::standard(
            state.enabled,
            "Enable actions",
            Message::EnabledChanged,
        )
        .into(),
        material::widget::toggler::standard(
            state.dark_mode,
            "Dark theme",
            Message::DarkModeChanged,
        )
        .into(),
    ])
    .spacing(12);

    let radios = page::row([
        material::widget::radio::standard(
            "Standard",
            RadioChoice::Standard,
            state.radio_choice,
            Message::ChoiceSelected,
        )
        .into(),
        material::widget::radio::standard(
            "Expressive",
            RadioChoice::Expressive,
            state.radio_choice,
            Message::ChoiceSelected,
        )
        .into(),
        material::widget::radio::standard(
            "Dense",
            RadioChoice::Dense,
            state.radio_choice,
            Message::ChoiceSelected,
        )
        .into(),
    ]);

    page::stack([switches.into(), radios.into()])
        .spacing(18)
        .into()
}
