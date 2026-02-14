# StackBuild - IconFlash

## Dev Stack
- **Frontend:** SvelteKit 2.50.1 + Svelte 5.48.2 + TypeScript 5.9.3
- **Backend:** Tauri v2.10.0 (Rust)
- **Build Tool:** Vite 7.3.1
- **Package Manager:** Bun
- **CSS:** Tailwind CSS 4.1.18 + PostCSS

## Target
- **Desktop** (Linux — GTK/WebKitGTK, bundle config supports all OS targets)

## Additional Key Libraries (UI)
- Tailwind CSS v4 with custom Catppuccin-like color theme (surface colors, accent blues)
- Tauri dialog plugin (@tauri-apps/plugin-dialog) for native file pickers
- Tauri fs plugin (@tauri-apps/plugin-fs) for filesystem access
- Custom SVG preview rendering in Svelte components
- File access scoped to: ~/.local/share/icons/, ~/.icons/, /usr/share/icons/, ~/.config/kitty/

## Key Features
Desktop GUI tool for NixOS that recolors SVG icon sets. Scans a directory of SVG icons, extracts dominant hex colors (1-5 per set), shows original palette alongside a representative icon, and lets you define replacement colors with live preview. Exports recolored icon set to ~/.local/share/icons/ with a custom name.

- Unlike manual sed/find-replace — visual two-panel UI with live SVG preview before and after
- Unlike theming tools that only handle GTK/Qt — works with any SVG icon set (Papirus, Tela, arcticons)
- Handles symlink dedup in icon directories (doesn't process the same file twice)
- Strips Inkscape metadata before color extraction for cleaner results

---

## Building Instructions

### Nix develop?
Yes — `flake.nix` with Rust + Bun + GTK/WebKitGTK + graphics libs (cairo, pango, librsvg).
```bash
nix develop                    # Or: direnv allow (auto via .envrc)
```

### Dev server?
```bash
bun install
bun run dev                    # SvelteKit dev server at localhost:5173
```

### Tauri dev server?
```bash
bun tauri dev                  # Full Tauri app with hot reload
```

---

## Android Build
N/A — desktop only. Mobile entry point markers exist in Rust but no Android config.

## Desktop Build
```bash
bun tauri build --no-bundle    # Release binary → src-tauri/target/release/icon-flash
bun tauri build                # Full release with OS-specific bundles
```

- **Release script?** No
- **Last build:** 2026-02-05

## Web Build
N/A — desktop application only.
