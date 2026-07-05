use super::*;

fn is_font_asset(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0x00, 0x01, 0x00, 0x00]) || bytes.starts_with(b"OTTO")
}

#[test]
fn bundled_fonts_are_font_assets() {
    assert!(is_font_asset(ROBOTO_REGULAR_BYTES));
    assert!(is_font_asset(ROBOTO_MEDIUM_BYTES));
    assert!(is_font_asset(ROBOTO_BOLD_BYTES));
    assert!(is_font_asset(MATERIAL_SYMBOLS_ROUNDED_BYTES));
    assert!(is_font_asset(MATERIAL_SYMBOLS_ROUNDED_FILLED_BYTES));
    assert_eq!(all().len(), 5);
}

#[test]
fn material_fonts_expose_expected_families_and_weights() {
    assert_eq!(ROBOTO.family, Family::Name(ROBOTO_FAMILY));
    assert_eq!(ROBOTO.weight, Weight::Normal);
    assert_eq!(ROBOTO_MEDIUM.weight, Weight::Medium);
    assert_eq!(ROBOTO_BOLD.weight, Weight::Bold);
    assert_eq!(
        NOTO_SANS_CJK_SC.family,
        Family::Name(NOTO_SANS_CJK_SC_FAMILY)
    );
    assert_eq!(NOTO_SANS_CJK_SC.weight, Weight::Normal);
    assert_eq!(NOTO_SANS_CJK_SC_MEDIUM.weight, Weight::Medium);
    assert_eq!(NOTO_SANS_CJK_SC_BOLD.weight, Weight::Bold);
    assert_eq!(
        MATERIAL_SYMBOLS_ROUNDED.family,
        Family::Name(MATERIAL_SYMBOLS_ROUNDED_FAMILY)
    );
    assert_eq!(
        MATERIAL_SYMBOLS_ROUNDED_FILLED.family,
        Family::Name(MATERIAL_SYMBOLS_ROUNDED_FILLED_FAMILY)
    );
}

#[test]
fn material_symbol_names_resolve_to_google_codepoints() {
    assert_eq!(material_symbol_codepoint("input"), Some('\u{e890}'));
    assert_eq!(material_symbol_codepoint("tune"), Some('\u{e429}'));
    assert_eq!(material_symbol_codepoint("info"), Some('\u{e88e}'));
    assert_eq!(material_symbol_codepoint("layers"), Some('\u{e53b}'));
    assert_eq!(material_symbol_codepoint("navigation"), Some('\u{e55d}'));
    assert_eq!(material_symbol_codepoint("menu"), Some('\u{e5d2}'));
    assert_eq!(material_symbol_codepoint("unknown_symbol"), None);
}

#[test]
fn material_symbol_fragment_falls_back_to_ligature_text_for_unknown_names() {
    assert_eq!(material_symbol_fragment("input").as_ref(), "\u{e890}");
    assert_eq!(
        material_symbol_fragment("unknown_symbol").as_ref(),
        "unknown_symbol"
    );
}

#[test]
fn type_scale_weights_select_roboto_faces() {
    assert_eq!(
        roboto_for_type_scale(tokens::typography::BODY_LARGE),
        ROBOTO
    );
    assert_eq!(
        roboto_for_type_scale(tokens::typography::LABEL_LARGE),
        ROBOTO_MEDIUM
    );
    assert_eq!(
        roboto_for_weight(tokens::typography::WEIGHT_BOLD),
        ROBOTO_BOLD
    );
}

#[test]
fn cjk_content_selects_noto_sans_cjk_sc() {
    assert!(!contains_cjk("Material 3 typography"));
    assert!(contains_cjk("中文字体"));
    assert!(contains_cjk("かな"));
    assert!(contains_cjk("한글"));
    assert_eq!(
        font_for_content_type_scale("中文字体", tokens::typography::BODY_LARGE),
        NOTO_SANS_CJK_SC
    );
    assert_eq!(
        font_for_content_type_scale("English", tokens::typography::LABEL_LARGE),
        ROBOTO_MEDIUM
    );
}
