use iced::Length;
use iced_material as material;
use material::widget::page;

use super::super::{Message, NAV_DESTINATIONS, Showcase, TabChoice};

pub(super) fn view(state: &Showcase) -> material::Element<'_, Message> {
    let selection = state.navigation_selection();
    let rail_height = showcase_rail_height();
    let bar = material::widget::navigation::navigation_bar(
        &NAV_DESTINATIONS,
        selection,
        Message::Navigate,
    );
    let rail = material::widget::navigation::navigation_rail_with_menu(
        &NAV_DESTINATIONS,
        selection,
        Message::Navigate,
        Message::MenuPressed,
    )
    .height(Length::Fixed(rail_height));
    let expanded_rail = material::widget::navigation::navigation_rail_expanded_with_menu(
        "Showcase",
        &NAV_DESTINATIONS,
        selection,
        Message::Navigate,
        Message::MenuPressed,
    )
    .height(Length::Fixed(rail_height));

    page::sections([
        page::section("Tabs", tabs(state)).into(),
        page::section("Navigation bar", bar).into(),
        page::section("Navigation rail with menu", rail).into(),
        page::section("Expanded navigation rail", expanded_rail).into(),
    ])
    .into()
}

fn tabs(state: &Showcase) -> material::Element<'_, Message> {
    page::stack([
        material::widget::tabs::animated_bar(
            material::widget::tabs::Variant::Primary,
            3,
            &state.primary_tab_state,
            [
                material::widget::tabs::primary_icon_label_for_animated_bar(
                    "input",
                    "Inputs",
                    state.primary_tab == TabChoice::Inputs,
                )
                .on_press(Message::PrimaryTabSelected(TabChoice::Inputs))
                .into(),
                material::widget::tabs::primary_icon_label_for_animated_bar(
                    "tune",
                    "Controls",
                    state.primary_tab == TabChoice::Controls,
                )
                .on_press(Message::PrimaryTabSelected(TabChoice::Controls))
                .into(),
                material::widget::tabs::primary_icon_label_for_animated_bar(
                    "info",
                    "Feedback",
                    state.primary_tab == TabChoice::Feedback,
                )
                .on_press(Message::PrimaryTabSelected(TabChoice::Feedback))
                .into(),
            ],
        )
        .into(),
        material::widget::tabs::animated_bar(
            material::widget::tabs::Variant::Secondary,
            3,
            &state.secondary_tab_state,
            [
                material::widget::tabs::secondary_label_for_animated_bar(
                    "Overview",
                    state.secondary_tab == TabChoice::Inputs,
                )
                .on_press(Message::SecondaryTabSelected(TabChoice::Inputs))
                .into(),
                material::widget::tabs::secondary_label_for_animated_bar(
                    "Details",
                    state.secondary_tab == TabChoice::Controls,
                )
                .on_press(Message::SecondaryTabSelected(TabChoice::Controls))
                .into(),
                material::widget::tabs::secondary_label_for_animated_bar(
                    "History",
                    state.secondary_tab == TabChoice::Feedback,
                )
                .on_press(Message::SecondaryTabSelected(TabChoice::Feedback))
                .into(),
            ],
        )
        .into(),
    ])
    .spacing(12)
    .into()
}

pub(super) fn showcase_rail_height() -> f32 {
    material::widget::navigation::navigation_rail_min_height(NAV_DESTINATIONS.len(), true)
}
