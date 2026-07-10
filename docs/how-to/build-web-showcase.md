# Build a WebAssembly App with Trunk

`material-ui-rs` works with a normal Trunk host page. Mobile Android and iOS
IME support is shipped inside the crate as a wasm-bindgen JavaScript module, so
you do not need to copy a bridge script, add a hidden input, or define browser
globals. [Trunk includes wasm-bindgen JavaScript snippets automatically][trunk-js].

## Use a Minimal Host Page

For an application whose `Cargo.toml` is one directory above `web/`, create
`web/index.html`:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <base data-trunk-public-url />
    <title>My app</title>
    <style>
      html, body { width: 100%; height: 100%; margin: 0; }
      body { overflow: hidden; }
      canvas { display: block; }
    </style>
    <link
      data-trunk
      rel="rust"
      href="../Cargo.toml"
      data-bin="my_app"
      data-wasm-opt="z"
    />
  </head>
  <body></body>
</html>
```

Omit `data-bin` when Cargo has only one unambiguous binary target. The
`data-trunk-public-url` base keeps generated assets working when the site is
deployed below a path prefix.

## Build and Serve

During development:

```sh
trunk serve web/index.html
```

For deployment:

```sh
trunk build web/index.html --release --dist dist --public-url /
python3 -m http.server 4173 --directory dist
```

Serve `dist/` over HTTP instead of opening `index.html` directly, so JavaScript
and WASM receive the correct MIME types.

For a small release binary, configure the application crate too:

```toml
[profile.release]
lto = "fat"
opt-level = "z"
codegen-units = 1
strip = true
panic = "abort"
```

This repository also passes `--converge` to `wasm-opt` in `web/index.html` so
the size-oriented optimization passes repeat until the output stabilizes. This
slightly increases release build time, not download or startup time.

## Add CJK Fonts Without Embedding Them

The mobile IME bridge and CJK fonts are separate concerns. IME input works
automatically. The showcase starts its font tasks quietly from application
startup with `fonts::load_web_font`; it never waits for a text control to receive
CJK input. The downloaded fonts remain outside the `.wasm` file. See
[Use bundled and CJK fonts](use-fonts.md).

This repository's showcase uses a silent two-stage strategy:

- After the application starts, it loads a 1.99 MB GB2312 core in the
  background without inserting a visible loading-status element.
- When the core request finishes, the complete official 8.33 MB Simplified
  Chinese regional font starts automatically in the background. The common
  core becomes usable first when available, and the official face then supplies
  its full repertoire. Completion stays silent. Core failure does not block the
  complete regional font; input events never start or retry either request.

The normal raw font payload is 10.32 MB, down from the previous 16.44 MB font,
and none of it is embedded in WASM. Versioned files are served from the same
Cloudflare Worker with a one-year immutable browser cache. Noto's own download
guide recommends [region-specific subset OTFs][noto-subsets] when only one
region is needed.

The showcase assets target Simplified Chinese (`Noto Sans SC`). Applications
for Traditional Chinese, Japanese, or Korean should build the corresponding
core and load the corresponding official regional font instead of reusing SC
glyph forms.

## Build This Repository's Showcase

The repository has multiple targets, so its host page includes showcase-only
feature flags:

```sh
trunk build web/index.html --release --dist dist --public-url /
```

Run the mobile input and font-asset regression tests with Node.js 24 or newer:

```sh
node --test web/*.test.mjs
```

The minimal downstream page above should not copy the showcase-only
`__showcase_web` feature configuration.

[trunk-js]: https://trunk-rs.github.io/trunk/guide/assets/index.html#js-snippets
[noto-subsets]: https://github.com/notofonts/noto-cjk/blob/main/Sans/README.md#region-specific-subset-otfs
