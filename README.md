# Atelier Press

**Page layout for the Nation.** Magazines, books and print documents: spreads and parent pages,
threaded text frames, paragraph and character styles, swatches, text wrap and a real paragraph
composer, with IDML import and export and PDF/X output. A native app written in Rust. Runs on
macOS, Windows, Linux and in the browser. Offline, no account, no cloud.

Press is one of the Atelier apps, Aetherion Nation's own creative suite. It is built on
[DesignCraft](https://github.com/storytold/designcraft) by the ArtCraft team (MIT OR Apache-2.0),
and keeps pulling their work in: DesignCraft is the engine, Atelier is the house it lives in.

| What | Where |
|---|---|
| Run it | `cargo run --release -p designcraft -- --sample` (desktop) · `apps/designcraft-web` (browser) · `designcraft-cli` (headless) |
| How it is built | `AGENTS.md`, `CLAUDE.md`, `docs/architecture.md` |
| The brand layer, and what we change versus upstream | [`docs/brand.md`](docs/brand.md) |
| Pulling in upstream work | [`docs/upstream.md`](docs/upstream.md) |

## Names

The product is **Atelier Press**. Crates, binaries, command ids and the `.designcraft` document
format keep their upstream `designcraft` names on purpose, so that upstream changes merge cleanly;
see `docs/brand.md` before renaming anything.

---

## What's in the box

The sections below are DesignCraft's own notes, kept as upstream writes them (they still say
"DesignCraft"; in Atelier the app is Press). They are the best description of what the app does.

## The sample magazine

Every page below was laid out by DesignCraft from code (`crates/engine/src/sample.rs`) and exported
by its own renderer. Try it yourself with `File → New → Sample Document`, or start the app with
`--sample`.

<table>
<tr>
<td width="50%" valign="top"><img src="docs/images/page-cover.png" alt="Magazine cover for The Spring Issue, No. 01: a dusk landscape of layered purple hills under a large pale sun, with the white serif headline The Quiet Art of Layout and an italic deck below" width="100%"><p align="center"><sub><b>Cover.</b> A full-bleed graphic frame, display type and an italic deck.</sub></p></td>
<td width="50%" valign="top"><img src="docs/images/page-2.png" alt="Feature opener titled Notes on the Grid, with an orange FEATURE · DESIGN kicker over a rule, an italic deck, a wide landscape picture with a small caption, and two columns of justified body text above a QUARTERLY folio" width="100%"><p align="center"><sub><b>Styles.</b> Kicker rule, headline, deck, caption and justified two-column body.</sub></p></td>
</tr>
<tr>
<td width="50%" valign="top"><img src="docs/images/page-3.png" alt="Three-column page of justified, hyphenated body text that wraps around a shaded pull quote reading The best layouts disappear. What remains is the story., with a gradient rule and SPRING ISSUE folio at the foot" width="100%"><p align="center"><sub><b>Threading, columns and text wrap.</b> One story flows through three columns and around the pull quote.</sub></p></td>
<td width="50%" valign="top"><img src="docs/images/page-4.png" alt="Coming Next page titled Color, Ink and Paper on a plum background, with four swatch circles labeled Ink Plum, Sunset, C=100 M=0 Y=0 K=0 and Paper Warm, above a short paragraph of body text" width="100%"><p align="center"><sub><b>Swatches.</b> Named colors and a CMYK process swatch, laid out as a palette.</sub></p></td>
</tr>
</table>

## Why DesignCraft

- **Familiar.** InDesign's layout, tools, menus, panels and shortcuts: spreads and parent pages,
  frames and threaded stories, the Control panel, paragraph and character styles, swatches, text
  wrap and more. If you know InDesign, you already know how to use it.
- **Beautiful type.** A Knuth–Plass paragraph composer (plus single-line), dictionary hyphenation
  (the public-domain Moby word list plus our own trained patterns), word, letter and glyph-scaling
  justification, keeps, optical margin alignment, columns, baseline grid, tabs, rules and shading.
  Line breaks are identical on screen and in PDF.
- **Fast.** Multithreaded SIMD rendering (vello_cpu), copy-on-write documents with O(1) undo
  snapshots, and cached composition.
- **Open.** A documented native format, IDML import and export, and PNG, PDF (PDF/X-4, PDF/A-2b)
  and EPUB export.
  No subscription, no licence server, no telemetry.
- **Agent-native.** Every menu item, tool gesture, panel control and dialog can be driven over a
  JSON control channel and an **MCP server**, so Claude and other agents can lay out and edit
  documents like a designer. See [`docs/control-protocol.md`](docs/control-protocol.md) and
  [`docs/mcp.md`](docs/mcp.md).
- **Everywhere.** One Rust codebase for desktop and the web.

## Quick start

The egui frontend requires Rust 1.95 or newer (including its shared `craft-ui` components).

```sh
cargo run --release -p designcraft                         # desktop app (start screen)
cargo run --release -p designcraft -- --sample             # open the sample magazine
cargo run --release -p designcraft -- --sample --control 7979   # + JSON control channel
cargo run --release -p designcraft-cli -- run --sample --all-pages out/       # headless: render every page to PNG
cargo run --release -p designcraft-cli -- commands         # list every command
cargo run --release -p designcraft-cli -- script --help    # command script syntax (docs/agents.md)
cargo xtask ci                                             # fmt, clippy, tests, assets, layering, wasm
```

Japanese text (UI and documents) uses fonts from
[storytold/craft-fonts](https://github.com/storytold/craft-fonts), an optional build input (font
files are never committed here; see craftrules
[`standards/fonts.md`](https://github.com/storytold/craftrules/blob/main/standards/fonts.md)):

```sh
git clone https://github.com/storytold/craft-fonts ../craft-fonts
CRAFT_FONTS_DIR="$PWD/../craft-fonts" cargo run --release -p designcraft   # absolute path
```

Without it, Japanese falls back to the system's fonts (none on the web). Release builds always
include it.

To drive a running app, send JSON lines to `127.0.0.1:7979`. The protocol is described in
[`docs/control-protocol.md`](docs/control-protocol.md).

### Logs

The desktop app writes its `log` records to standard error and to `logs/designcraft.log` in the
settings directory, beside `ui.json` and `prefs.json` (Linux `~/.config/designcraft/logs/`, or
`$XDG_CONFIG_HOME/designcraft/logs/`; macOS `~/Library/Application Support/DesignCraft/logs/`;
Windows `%APPDATA%\DesignCraft\logs\`). A start launched from a desktop menu or the Dock has no
terminal, so this file is what to attach to a bug report: the crash guard's panic report and a
failed crash-recovery save land there. Each launch moves the previous log to `designcraft.1.log`
(and that one to `designcraft.2.log`), so the log of a run that crashed survives the next start.
The file stops growing at 16 MiB. `--version` writes no file.

| Variable | Effect |
|---|---|
| `RUST_LOG` | Log levels for standard error and the log file. Default: `info` for DesignCraft's own crates, `warn` for everything else. env_logger-style directives replace that, e.g. `RUST_LOG=debug`, `RUST_LOG=warn,designcraft_render=trace` or `RUST_LOG=info,wgpu_core=warn`; a directive ending in `*` covers every target starting with it (`designcraft*=debug`). |
| `WGPU_BACKEND` | The graphics backend that composites the window: `dx12`, `vulkan`, `gl` or `metal` (a comma-separated list lets wgpu choose among them). Default: DirectX 12 on Windows, Vulkan on Linux, Metal on macOS, with OpenGL as the fallback. Setting it also turns off the start-up fallback described below. |

The logger is `apps/designcraft/src/logging.rs`; the web build logs to the browser console instead.

### Graphics backend

The canvas is rendered on the CPU; the GPU only composites the interface, through wgpu. A
graphics driver that faults takes the process down before any Rust code can catch it, so the
backend is chosen before the window exists, and a start that never showed a frame is remembered:
`gpu.json` in the settings directory records the backend being tried until the first frame has
been presented. A record left behind means that backend crashed (or the app was killed) at
start-up, so the next start tries the next one (DirectX 12, Vulkan, then OpenGL on Windows;
Vulkan, then OpenGL on Linux) and says so in the status bar. The list starts over once every
backend has failed, or with a new DesignCraft version; delete the file or set `WGPU_BACKEND` to
start afresh. Seen in the wild: on an AMD hybrid-graphics laptop (a Radeon RX 6800M beside an
integrated Radeon) the AMD Vulkan driver faulted at the first present, which is why DirectX 12 is
the Windows default. The code is `apps/designcraft/src/gpu.rs`.

### Web

```sh
cd apps/designcraft-web && trunk build --release          # → dist/web (serve it with any static server)
cd apps/designcraft-web && trunk serve --release          # http://127.0.0.1:8767
```

You need [trunk](https://trunkrs.dev) and the `wasm32-unknown-unknown` target. The same app runs
through eframe's web runner on WebGPU, falling back to WebGL2 (`?webgl` forces it; `?sample` opens
the sample magazine). Open and Place use the browser's file picker, and dropping files works too.
Save and Export download the file. The web build has no control channel.

## Architecture

DesignCraft is an engine-first Cargo workspace with enforced layering (`cargo xtask layers`). The
egui frontend is a separate crate, so the UI can be swapped without touching the engine.

| Layer | Crates |
|---|---|
| L0 | `geom` (paths, units & measurement parsing, corner options) · `color` (CMYK/RGB/Lab, swatches, tints, gradients) |
| L1 | `doc` (spreads, pages, parents, layers, frames, stories, styles) · `fonts` (font DB, shaping, outlines) |
| L2 | `compose` (the text engine) |
| L3 | `render` (vello_cpu) |
| L4 | `tools` (pointer events → commands + overlays) |
| L5 | `engine` (session, history, command registry) |
| L6 | `ui-egui` (InDesign-style UI, control channel) |
| L7 | `apps/designcraft`, `apps/designcraft-cli`, `apps/designcraft-web` |

- **Status and milestones:** [ROADMAP.md](ROADMAP.md) (stage: pre-alpha); how close to InDesign: [`docs/target-app-parity.md`](docs/target-app-parity.md); known gaps: [`docs/gaps.md`](docs/gaps.md)
- **Contributor and agent rules** (clean-room, asset policy, quality gates): [`AGENTS.md`](AGENTS.md)
- **Bundled assets:** every one is listed with its licence in [`ASSETS.md`](ASSETS.md)
- **App icon and colour:** a calico cat in a polka-dot scarf on DesignCraft green `#7bb51c`; see [`assets/app-icon/`](assets/app-icon/README.md)

## License

MIT OR Apache-2.0, like DesignCraft. Third-party material is listed in `ASSETS.md` and `NOTICE`.
The Atelier and Aetherion names and marks are not covered by the license.

<sub>Adobe and InDesign are trademarks or registered trademarks of Adobe Inc. in the United States
and/or other countries. Atelier Press is an independent, open-source project and is not affiliated
with, sponsored by or endorsed by Adobe Inc.; these names are used only to describe the workflows
it is compatible with.</sub>
