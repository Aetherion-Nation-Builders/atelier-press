# The brand layer

Atelier Press is DesignCraft with a different name on the door. We keep the change small on
purpose: upstream lands many changes a day, and every file we touch is a file that can conflict
when we pull them in. So:

**Renamed (what a person sees):** the window title, the macOS app menu, Help-menu items and links
(the Nation, Atelier, this repository), the About window (the Atelier mark, the other Atelier
apps), the start screen's welcome and links, the title-bar "Nation" button, the document type in
open and save dialogs, the PDF Creator and Producer, MCP server title and connection messages, CLI
`links` output, installer and package names, vendor and links, the browser page, the app icon,
and where settings and recovery files live (`~/Library/Application Support/Atelier Press`,
`%APPDATA%\Atelier Press`).

**Not renamed (machine names):** crate names (`designcraft-*`), the binaries (`designcraft`,
`designcraft-cli`), command ids (`help.discord` opens the Nation) and automation ids, the
`ai.storyteller.designcraft` app id and icon file names in packaging, the `.designcraft` format and
its `generator` field, the "DesignCraft Generic CMYK" colour profile name, the EPUB generator tag,
environment variables (`DESIGNCRAFT_*`), the Linux settings folder (`~/.config/designcraft`), the
macOS bundle executable, and the docs under `docs/`.

## Where the name lives

| Place | File |
|---|---|
| Links | `crates/engine/src/links.rs` (keys keep their upstream names, the URLs are ours) |
| About window, Atelier mark, other Atelier apps | `crates/ui-egui/src/about.rs` (the mark is drawn in code) |
| Title bar, Nation button, start screen | `crates/ui-egui/src/chrome.rs` |
| Help menu | `crates/ui-egui/src/menus.rs` (`help.*` rows) |
| Translations of our strings | rows in `crates/ui-egui/src/i18n.rs` and `i18n/{ar,it,ja,pt_br,uk}.rs` |
| macOS app menu, window title, settings folder, file dialogs | `apps/designcraft/src/{native_menu,main}.rs` |
| Recovery folder | `crates/engine/src/recovery.rs` |
| PDF Creator and Producer | `crates/pdf/src/export.rs` |
| MCP and CLI | `crates/mcp/src/{server,backend}.rs`, `apps/designcraft-cli/src/main.rs` |
| Windows file properties | `apps/designcraft/build.rs` |
| macOS bundle | `packaging/macos/Info.plist.in` (display name, copyright, document type; identifier unchanged) |
| Windows installer | `packaging/windows/designcraft.wxs` (names, manufacturer, links; upgrade code unchanged) |
| Linux desktop entry, AppStream, nfpm | `packaging/linux/ai.storyteller.designcraft.*`, `nfpm.yaml` |
| Browser build | `apps/designcraft-web/index.html`, `src/web.rs` |
| README and NOTICE | README: our header above upstream's feature sections; NOTICE: ours on top of theirs, kept on merge by `.gitattributes` |
| App icon | `assets/app-icon/` (ours; rows in `ASSETS.md`) |

Still upstream's name, for the packaging pass (real installers): the macOS `.app` bundle and DMG
volume name (`packaging/macos/package.sh`), the DMG window background
(`packaging/macos/dmg/background.tiff`), and the Windows and Linux package scripts' file names.
Also left for later passes: engine error messages that name DesignCraft, and the `docs/` manuals.
