# Bundled Fonts

These font files are bundled so `iced_material` can render Material 3 typography
and Material Symbols without depending on system-installed fonts.

Sources:

- `Roboto-Regular.ttf`, `Roboto-Medium.ttf`, and `Roboto-Bold.ttf` were
  downloaded through Google Fonts CSS for `Roboto` weights 400, 500, and 700.
- `NotoSansCJKsc-Regular.otf`, `NotoSansCJKsc-Medium.otf`, and
  `NotoSansCJKsc-Bold.otf` were downloaded from the official Noto CJK
  repository under `Sans/OTF/SimplifiedChinese`.
- `MaterialSymbolsRounded-Regular.ttf` was downloaded through Google Fonts CSS
  for `Material Symbols Rounded` with `opsz=24`, `wght=400`, `FILL=0`, and
  `GRAD=0`.
- `MaterialSymbolsRounded-Filled.ttf` was downloaded through Google Fonts CSS
  for `Material Symbols Rounded` with `opsz=24`, `wght=400`, `FILL=1`, and
  `GRAD=0`, then renamed internally to `Material Symbols Rounded Filled` so the
  renderer can select it independently from the outline face.

Roboto and Material Symbols are Google font assets distributed under the Apache
License, Version 2.0. The license text is included in
`LICENSE-APACHE-2.0.txt`.

Noto Sans CJK SC is distributed under the SIL Open Font License, Version 1.1.
The license text is included in `LICENSE-OFL-1.1.txt`.
