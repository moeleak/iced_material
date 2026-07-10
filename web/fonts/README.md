# Showcase CJK Fonts

These files stay outside the WASM binary and are copied into `dist/fonts/` by
Trunk.

At runtime the showcase starts the core from `boot`, then automatically starts
the complete official regional font when the core request finishes. Core
failure does not block the complete font, and text-input events never initiate
either request.

- `NotoSansSC-Core-0a7ff25a.otf` keeps Basic Latin and the GB2312 repertoire so
  common Simplified Chinese text can render after a small background download.
- `NotoSansSC-faa6c9df.otf` is the byte-for-byte official Simplified Chinese
  regional font. Its full repertoire completes the small core.

Both faces retain the `Noto Sans SC` family name so iced/cosmic-text can move
between them when a glyph is missing. The core is derived from the official Noto
Sans SC 2.004 [regional subset][upstream], and both files are distributed under
the SIL Open Font License 1.1 in `OFL-1.1.txt`. The hash suffixes make the
one-year immutable cache safe when assets change.

The pinned upstream font has SHA-256
`faa6c9df652116dde789d351359f3d7e5d2285a2b2a1f04a2d7244df706d5ea9`.
Download the official runtime asset and rebuild the core with the locked
FontTools:

```sh
nix develop -c curl --fail --location \
  --output web/fonts/NotoSansSC-faa6c9df.otf \
  https://raw.githubusercontent.com/notofonts/noto-cjk/Sans2.004/Sans/SubsetOTF/SC/NotoSansSC-Regular.otf

nix develop -c python3 web/subset_cjk_font.py \
  web/fonts/NotoSansSC-faa6c9df.otf \
  web/fonts/NotoSansSC-Core-0a7ff25a.otf
```

[upstream]: https://github.com/notofonts/noto-cjk/blob/Sans2.004/Sans/SubsetOTF/SC/NotoSansSC-Regular.otf
