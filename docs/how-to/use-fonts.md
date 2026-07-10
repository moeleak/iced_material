# Use Bundled and CJK Fonts

`material-ui-rs` bundles Roboto and Material Symbols Rounded font files. The
application helper loads them for you.

## Use the Default Path

For a Material-themed app, use `material::application`:

```rust
use material_ui_rs as material;

material::application(boot, update, view)
    .title("My app")
    .run()
```

This is equivalent to creating an `iced` application and then calling
`material::with_material_fonts`.

## Add Material Fonts to an Existing Application

If an app is already created through `iced::application`, wrap it before
running:

```rust
let app = iced::application(boot, update, view);

material::with_material_fonts(app).run()
```

Use this when you need the regular `iced` application builder first but still
want the bundled fonts.

## Use Material Symbols

Use `material::fonts::icon` or the widget constructors that accept icon names:

```rust
let icon = material::fonts::icon("menu", 24.0);
```

Several common symbol names are mapped to codepoints. Other names are rendered
through the Material Symbols Rounded font, so the exact output depends on the
font's ligature support in the renderer.

## Load CJK Fonts on WebAssembly

Full CJK fonts are intentionally not embedded in the crate because they add
many megabytes to the WASM module. Do not wait for the first CJK keystroke.
Start a small common-glyph subset from `boot`, then start its official regional
font automatically as soon as the subset request finishes:

```rust
const CJK_CORE: &str = "fonts/NotoSansSC-Core-0a7ff25a.otf";
const CJK_REGIONAL: &str = "fonts/NotoSansSC-faa6c9df.otf";

#[derive(Debug, Clone)]
enum Message {
    CjkCoreFontFinished,
    CjkRegionalFontFinished,
    TextChanged(String),
}

fn boot() -> (State, iced::Task<Message>) {
    let load_core = material::fonts::load_web_font(CJK_CORE)
        .map(|_| Message::CjkCoreFontFinished);

    (State::default(), load_core)
}

fn update(state: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        Message::CjkCoreFontFinished => {
            material::fonts::load_web_font(CJK_REGIONAL)
                .map(|_| Message::CjkRegionalFontFinished)
        }
        Message::CjkRegionalFontFinished => iced::Task::none(),
        Message::TextChanged(value) => {
            state.text = value;
            iced::Task::none()
        }
    }
}
```

Neither `TextChanged` nor any other input message participates in font loading.
The first internal completion message starts the regional font even if the core
request failed; the complete official font can work independently. The second
message causes a redraw and ends the chain without adding visible state or
retrying on every keystroke.

The second stage is the byte-for-byte official 8.33 MB Simplified Chinese
regional OTF. Together with the 1.99 MB core, the automatic background sequence
transfers 10.32 MB. The full regional face intentionally overlaps the core: the
core makes common glyphs available earlier, while the official face completes
the repertoire. Both files stay outside the `.wasm` binary and can be cached
independently by the browser.

Keep these URLs same-origin and serve them with long-lived cache headers. A
cross-origin URL must allow CORS. Use raw `.ttf`, `.otf`, or `.ttc` files;
browser-oriented WOFF2 files and CSS `@font-face` rules cannot populate iced's
renderer font database. Do not preload either multi-megabyte font alongside
WASM, because that would move font traffic onto the initial critical path.

To let Trunk copy a self-hosted `web/fonts/` directory, add this optional host
page asset next to the Rust link:

```html
<link data-trunk rel="copy-dir" href="fonts" />
```

## Use CJK Font Constants

The crate exposes `Noto Sans CJK SC` font constants for applications that load
that font family themselves:

```rust
let scale = material::tokens::typography::BODY_LARGE;
let font = material::fonts::noto_sans_cjk_sc_for_type_scale(scale);
```

`material::fonts::font_for_content_type_scale` chooses Roboto or Noto Sans CJK
SC from the text content. Load a face whose internal family name is
`Noto Sans CJK SC` before relying on those constants. Applications targeting
Traditional Chinese, Japanese, or Korean should load and select the matching
regional Noto Sans CJK family so shared Han characters use locale-appropriate
glyph forms.

The official region-specific file named `NotoSansSC-Regular.otf` uses the
family name `Noto Sans SC`, not `Noto Sans CJK SC`. It works as a renderer
fallback, but does not directly match the named constants above.
