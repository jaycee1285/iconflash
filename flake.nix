{
  description = "IconFlash - Recolor SVG icon sets with a visual preview";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Node/Bun
            bun
            nodejs

            # Rust
            rustc
            cargo
            rustfmt
            clippy

            # Tauri build deps
            pkg-config
            openssl

            # GTK/WebKit (Tauri v2)
            webkitgtk_4_1
            gtk3
            glib
            glib-networking
            libsoup_3

            # Text/graphics rendering
            cairo
            pango
            gdk-pixbuf
            harfbuzz
            librsvg

            # Accessibility
            atk

            # IPC
            dbus

            # Wayland
            wayland
            wayland-protocols
            libxkbcommon

            # X11 fallback
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libxcb
          ];

          shellHook = ''
            export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules"
            export WEBKIT_DISABLE_COMPOSITING_MODE=1
          '';
        };
      }
    );
}
