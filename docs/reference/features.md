# Feature Flags

The crate features control optional integrations and compatibility flags.

## Default Features

`default = ["svg", "animate", "canvas"]`

The default set enables:

- SVG support.
- Theme and widget animation support through `iced_anim`.
- Canvas drawing support for components that need custom geometry.

## Optional Features

- `serde`: enables serialization support for theme data.
- `animate`: enables integration with `iced_anim`.
- `canvas`: compatibility feature for existing manifests; canvas drawing is
  always enabled.
- `crisp`: enables pixel snapping for crisp edges by default. This can cause
  jitter in animated layouts.
- `dialog`: enables `iced_dialog` support.
- `selection`: enables `iced_selection` support.
- `markdown`: enables the markdown widget.
- `svg`: enables the SVG widget.
- `qr_code`: enables the QR code widget.

## Internal Feature

- `__showcase_web`: builds the WebAssembly showcase binary used by Trunk.

Do not depend on internal features from downstream applications.
