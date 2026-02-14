# Claude Instructions for IconFlash

## Project Overview
A desktop GUI tool for NixOS that recolors SVG icon sets by replacing hex color values.
Built with Tauri v2 (Rust backend) + SvelteKit (TypeScript frontend) + Tailwind CSS 4.
Deployed locally on NixOS via Home Manager integration. Uses system GTK theme.

## Key Paths
| Path | Purpose |
|------|---------|
| `src/routes/` | SvelteKit pages |
| `src/lib/` | Shared components, stores, types |
| `src-tauri/src/` | Rust backend (Tauri commands) |
| `src-tauri/Cargo.toml` | Rust dependencies |
| `src-tauri/capabilities/` | Tauri permission scopes |
| `flake.nix` | Nix dev shell |

## Owner
- GitHub: jaycee1285
- User: john

---

## Documentation Requirements

**Update Obsidian after completing work.**

### Project File Location
`~/Sync/JMC/SideProjects/IconFlash/IconFlash.md`

### After Completing Tasks
Update the frontmatter:
- `last-completed`: What you just finished
- `next-tasks`: Remove done items, add new ones
- `blockers`: Set if you need human input, otherwise "None"

### Creating New Docs
Put reference docs in `~/Sync/JMC/SideProjects/IconFlash/` with:
type: reference, parent: "[[IconFlash]]", created date, and tags.

---

## Build/Run Commands
| Command | Purpose |
|---------|---------|
| `bun run dev` | SvelteKit dev server only |
| `bun tauri dev` | Full Tauri dev (frontend + Rust) |
| `bun tauri build --no-bundle` | Build release binary |
| `bun run check` | Svelte type checking |
| `cargo clippy --manifest-path src-tauri/Cargo.toml` | Rust linting |

---

## Current State
**Read the Obsidian project file before starting work:**
`~/Sync/JMC/SideProjects/IconFlash/IconFlash.md`

That file has the canonical task list, known issues, blockers,
and PORTFOLIO-tagged features. Do not duplicate state here.
