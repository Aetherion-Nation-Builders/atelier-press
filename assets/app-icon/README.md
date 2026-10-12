# Atelier Press app icon

**The hidden sun behind a printed page**: an ivory disc rising behind an ochre page set in two
columns under a headline, on a volcanic black field. Three colours, from the Nation's palette:
volcanic black `#141210`, ivory `#efe6d2`, ochre `#d9a441`.

## Geometry

A 512 × 512 viewBox, clipped to a rounded square with `rx=112`, as all the Atelier icons. Windows
and Linux use the full tile. The macOS icons (`.icns`, `designcraft-macos-512.png`) sit on Apple's
grid: an 824/1024 body with a transparent margin.

## Provenance

Drawn for Atelier Press by hand as SVG (2026). It replaces DesignCraft's cat icon, which stays in
DesignCraft's own repository. Licence: `LICENSE.txt` (MIT OR Apache-2.0). The file names keep
their upstream `designcraft` names so packaging stays untouched.

## Files

| File | Use |
|---|---|
| `designcraft.svg` | Canonical artwork |
| `designcraft-small.svg` | The same artwork (upstream kept a lighter trace here); also `hicolor/scalable` |
| `designcraft-1024.png` | 1024 px render of the full tile |
| `designcraft-macos-512.png` | Runtime Dock icon on macOS (embedded by `apps/designcraft`) |
| `designcraft.icns` | macOS bundle icon (`CFBundleIconFile`) |
| `designcraft.ico` | Windows exe icon, 16–256 px (embedded by `apps/designcraft/build.rs`) |
| `hicolor/<size>/apps/ai.storyteller.designcraft.png` | Linux icon theme, 16–512 px; the 256 px one is also the runtime window icon on Windows and Linux |

`packaging/icons.sh` regenerates the renders (needs resvg; iconutil on macOS for the `.icns`). The
committed renders were made from the same SVG with cairosvg and Pillow.
