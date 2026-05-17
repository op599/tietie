# Tietie (贴贴)

English · [简体中文](./README.zh-CN.md)

> A beautiful, lightweight, cross-platform clipboard manager.

Bottom-drawer popup, all-keyboard driven, Tauri 2 + React + Rust, ~10 MB installer. Does one thing well: **clipboard**.

## Features

- **Bottom drawer** — `⌘⇧V` slides up a full-width frosted-glass panel over the current app
- **Auto categorization** — text / link / image / code / color, detected by content heuristics
- **User folders** — colored chips for custom organization
- **Pin** — keep frequently-used items at the front
- **Inline edit** — `⌘E` edits in place, no new window
- **Keyboard-first** — `←/→` to navigate, `↵` to paste, `⌘1-9` to pick directly, `⌘P` to pin, `⌘D` to delete
- **History search** — `⌘F` full-text
- **Image support** — PNG thumbnails + full data, restored on paste
- **Tray mini panel** — click the tray icon for a search box + most recent 12 items; click to paste back

### Default shortcuts

| Shortcut | Action |
|---|---|
| `⌘⇧V` | Toggle drawer |
| `⌘F` | Focus search |
| `⌘E` | Edit current item |
| `⌘P` | Pin |
| `⌘D` | Delete |
| `⌘1`–`⌘9` | Paste the N-th item directly |
| `←` `→` `↵` | Select + paste |
| `Tab` | Switch type tabs |
| `Esc` | Close drawer |

## Download

[Releases](https://github.com/wangxiuwen/tietie/releases) ships:

| Platform | Format |
|---|---|
| macOS Apple Silicon | `Tietie_x.y.z_aarch64.dmg` |
| macOS Intel | `Tietie_x.y.z_x64.dmg` |
| Windows 10/11 | `Tietie_x.y.z_x64-setup.exe` / `.msi` |
| Linux | `tietie_x.y.z_amd64.deb` / `.AppImage` / `.rpm` |

> macOS: the app is unsigned. On first launch open *System Settings → Privacy & Security* and click *Open Anyway*.

## Local development

Requires `Node 20+`, `Rust stable`. Linux additionally needs `libwebkit2gtk-4.1-dev` and friends (see the CI files).

```bash
npm install
npm run tauri:dev     # dev with hot reload
npm run tauri:build   # build native installer
```

## Architecture

```
tietie/
├── src/                    # React frontend
│   ├── App.tsx             # drawer main panel
│   ├── TrayPopover.tsx     # tray mini panel
│   └── styles.css
├── src-tauri/              # Rust backend
│   └── src/
│       ├── lib.rs          # entry, tray, hotkey, IPC
│       ├── db.rs           # SQLite (rusqlite, bundled)
│       └── clipboard.rs    # 600ms polling + content classification
└── .github/workflows/      # tri-platform CI
```

**Performance trade-offs**
- Rust backend, React frontend (~170 KB gzipped)
- SQLite WAL, unique index for dedup
- 600 ms clipboard polling (macOS has no public change-notification API; polling is simpler and portable)
- `tauri.conf` lto + codegen=1 + opt-level=s + strip → release binary ~6–8 MB

## Roadmap

- [x] **v0.1** — drawer + auto-categorization + tray mini panel
- [ ] **v0.2** — full-screen Library window (mockup screen 4)
- [ ] **v0.2** — Settings panel (mockup screen 5) + long-press `⌘V` interception
- [ ] **v0.3** — iCloud / WebDAV sync

## License

MIT
