import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import test from "node:test";

const core = readFileSync(
  new URL("fonts/NotoSansSC-Core-0a7ff25a.otf", import.meta.url),
);
const regional = readFileSync(
  new URL("fonts/NotoSansSC-faa6c9df.otf", import.meta.url),
);
const subsetTool = readFileSync(
  new URL("subset_cjk_font.py", import.meta.url),
  "utf8",
);
const showcase = readFileSync(
  new URL("../examples/showcase/app.rs", import.meta.url),
  "utf8",
);
const showcasePages = readFileSync(
  new URL("../examples/showcase/pages/mod.rs", import.meta.url),
  "utf8",
);
const html = readFileSync(new URL("index.html", import.meta.url), "utf8");
const headers = readFileSync(new URL("_headers", import.meta.url), "utf8");

const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

test("CJK font assets are valid, pinned OTF files", () => {
  assert.equal(core.subarray(0, 4).toString("ascii"), "OTTO");
  assert.equal(regional.subarray(0, 4).toString("ascii"), "OTTO");
  assert.equal(
    sha256(core),
    "0a7ff25a235072033a116b8094ec7fa5659edf9fafe243828618526e798b939f",
  );
  assert.equal(
    sha256(regional),
    "faa6c9df652116dde789d351359f3d7e5d2285a2b2a1f04a2d7244df706d5ea9",
  );
  assert.equal(
    subsetTool.match(/UPSTREAM_SHA256 = "([a-f0-9]+)"/)?.[1],
    "faa6c9df652116dde789d351359f3d7e5d2285a2b2a1f04a2d7244df706d5ea9",
  );
});

test("the automatic two-stage CJK payload stays below 10.4 MB", () => {
  assert.ok(core.byteLength < 2_000_000);
  assert.equal(regional.byteLength, 8_331_336);
  assert.ok(core.byteLength + regional.byteLength < 10_400_000);
});

test("the showcase uses same-origin versioned fonts with immutable caching", () => {
  assert.match(showcase, /fonts\/NotoSansSC-Core-0a7ff25a\.otf/);
  assert.match(showcase, /fonts\/NotoSansSC-faa6c9df\.otf/);
  assert.doesNotMatch(showcase, /cdn\.jsdelivr\.net/);
  assert.match(html, /rel="copy-dir" href="fonts"/);
  assert.match(headers, /\/fonts\/\*/);
  assert.match(headers, /max-age=31536000, immutable/);
});

test("font loading has no visible status or layout branch", () => {
  assert.doesNotMatch(showcase, /CjkFontStatus|contains_cjk/);
  assert.doesNotMatch(showcase, /NotoSansSC-Regular-2\.004/);
  assert.doesNotMatch(showcase, /result\.is_ok\(\)/);
  assert.match(showcase, /\.map\(\|_\| Message::CjkCoreFontFinished\)/);
  assert.doesNotMatch(showcasePages, /cjk_font|Noto Sans/);
});
