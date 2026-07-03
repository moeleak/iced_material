use iced::alignment;
use iced_material as material;
use material::widget::page;

use super::super::{Message, Showcase};

pub(super) fn view(state: &Showcase) -> material::Element<'_, Message> {
    page::sections([
        page::section("Top app bars", top_app_bars()).into(),
        page::section("Search view", search_view(state)).into(),
        page::section("Bottom app bar", bottom_app_bar()).into(),
        page::section("Bottom sheets", bottom_sheets(state)).into(),
        page::section("Side sheets", side_sheets(state)).into(),
    ])
    .into()
}

fn top_app_bars() -> material::Element<'static, Message> {
    page::stack([
        material::widget::app_bar::with_status_bar(material::widget::app_bar::small(
            "Small",
            Some(app_bar_action("menu", Message::MenuPressed)),
            [
                app_bar_action("search", Message::Increment),
                app_bar_action("info", Message::Increment),
            ],
        ))
        .into(),
        material::widget::app_bar::with_status_bar(material::widget::app_bar::medium(
            "Medium",
            Some(app_bar_action("menu", Message::MenuPressed)),
            [
                app_bar_action("search", Message::Increment),
                app_bar_action("info", Message::Increment),
            ],
        ))
        .into(),
        material::widget::app_bar::with_status_bar(material::widget::app_bar::large(
            "Large",
            Some(app_bar_action("menu", Message::MenuPressed)),
            [
                app_bar_action("search", Message::Increment),
                app_bar_action("info", Message::Increment),
            ],
        ))
        .into(),
    ])
    .spacing(12)
    .into()
}

fn search_view(state: &Showcase) -> material::Element<'_, Message> {
    let results = page::stack([
        material::widget::list::one_line_with_leading_icon("input", "Inputs").into(),
        material::widget::list::one_line_with_leading_icon("tune", "Controls").into(),
        material::widget::list::one_line_with_leading_icon("info", "Feedback").into(),
    ])
    .spacing(0);

    material::widget::search::docked_view(
        "Search components",
        &state.search_query,
        Message::SearchChanged,
        results,
    )
    .into()
}

fn bottom_app_bar() -> material::Element<'static, Message> {
    material::widget::app_bar::bottom(
        [
            app_bar_action("menu", Message::MenuPressed),
            app_bar_action("search", Message::Increment),
            app_bar_action("info", Message::Increment),
        ],
        Some(material::widget::button::primary_fab_action(
            "+",
            Message::Increment,
        )),
    )
    .into()
}

fn app_bar_action(icon: &'static str, on_press: Message) -> material::Element<'static, Message> {
    material::widget::app_bar::icon_action(icon, on_press)
}

fn bottom_sheets(state: &Showcase) -> material::Element<'static, Message> {
    let width = page::preview_width(state.window_size.width);
    let standard = material::widget::sheet::standard_bottom(sheet_content(
        "Standard bottom sheet",
        "Coexists with the page and keeps secondary content available.",
    ));

    let modal_preview = page::preview_pane(material::widget::sheet::modal_overlay(sheet_content(
        "Modal bottom sheet",
        "Uses a scrim and blocks interaction behind the sheet.",
    )));

    page::stack([
        page::centered_preview(width, standard).into(),
        page::centered_preview(width, modal_preview).into(),
    ])
    .spacing(12)
    .into()
}

fn side_sheets(state: &Showcase) -> material::Element<'static, Message> {
    let width = page::preview_width(state.window_size.width);
    let standard = page::aligned_preview_pane(
        alignment::Horizontal::Right,
        material::widget::sheet::standard_side(side_sheet_content(
            "Standard side sheet",
            "Coexists with the page while supporting content remains visible.",
        )),
    );

    let modal = page::preview_pane(material::widget::sheet::modal_side_overlay(
        material::widget::sheet::Side::Right,
        side_sheet_content(
            "Modal side sheet",
            "Uses a scrim and keeps focus on a temporary side task.",
        ),
    ));

    page::stack([
        page::centered_preview(width, standard).into(),
        page::centered_preview(width, modal).into(),
    ])
    .spacing(12)
    .into()
}

fn sheet_content(
    title: &'static str,
    supporting: &'static str,
) -> material::Element<'static, Message> {
    material::widget::sheet::bottom_content(
        page::stack([
            material::text::title_medium(title).into(),
            material::text::body_medium(supporting).into(),
            page::row([
                material::widget::button::text_action("Dismiss", Message::Decrement),
                material::widget::button::filled_action("Apply", Message::Increment),
            ])
            .into(),
        ])
        .spacing(8),
    )
    .into()
}

fn side_sheet_content(
    title: &'static str,
    supporting: &'static str,
) -> material::Element<'static, Message> {
    material::widget::sheet::side_content(
        page::stack([
            material::text::title_medium(title).into(),
            material::text::body_medium(supporting).into(),
            page::row([
                material::widget::button::text_action("Dismiss", Message::Decrement),
                material::widget::button::filled_action("Apply", Message::Increment),
            ])
            .into(),
        ])
        .spacing(8),
    )
    .into()
}
