# Native and WebAssembly

The same widget APIs are intended to work on native and WebAssembly targets, but
the integration points differ.

## Native

Native examples run through Cargo:

```sh
cargo run --example quickstart
cargo run --example showcase
```

Use `material::window`, `material::window_with_min_size`, or
`material::window_settings` to keep window sizing consistent with the examples.

## WebAssembly

The web showcase is built by Trunk:

```sh
trunk build web/index.html --release --dist dist --public-url /
```

Serve the generated `dist/` directory over HTTP. Direct file loading does not
provide reliable MIME types for JavaScript modules and WASM.

On browsers, iced/winit does not attach an editable DOM control to its canvas
and its Web IME hooks are not implemented. `material-ui-rs` therefore ships an
internal wasm-bindgen JavaScript adapter that keeps a visually hidden text input
focused while a Material text field is active. The adapter is included and
initialized lazily by the crate; host pages do not need a script tag, hidden
input, or JavaScript hooks.

The adapter uses composition and `beforeinput` events so macOS desktop input
methods and Android or iOS keyboards can commit CJK text without leaking
candidate-navigation keys into the application. Desktop command and navigation
keys are forwarded with their modifiers, while input-source shortcuts and
standard IME mode keys (including Japanese and Korean keyboard keys) remain
native to the browser. Composition-owned keys also stay inside the IME. The
pointer-region gesture bridge that opens a soft keyboard remains limited to
touch or coarse-pointer environments.

This bridge provides committed-text input. The current sentinel model does not
mirror surrounding application text or selection into the DOM, so browser IME
preedit, reconversion, native selection handles, and surrounding-text
autocorrection are not exposed through iced 0.14.

The showcase keeps CJK font delivery separate from IME handling. A small common
Simplified Chinese subset loads silently after application startup, followed by
the complete official regional font automatically when the core request
finishes. Core failure does not block the complete regional font, and no
text-input message starts or retries either request. The core makes common
glyphs available first when available, and the full regional face completes the
repertoire. These versioned OTF assets stay outside WASM and are cached by the
same-origin static asset service. Font completion never adds a status row or an
application-driven layout branch, although text can naturally reflow when the
new glyph metrics become available.

## Platform-Specific Behavior

Platform-specific behavior should stay behind narrow adapters or `cfg`
sections. For example, input method normalization is isolated so native and WASM
behavior remain predictable for text fields and overlays.

When changing shared widgets, verify both native examples and the WebAssembly
showcase if the change touches navigation, overlays, input handling, or
animation behavior.
