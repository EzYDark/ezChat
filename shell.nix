# Run with `nix-shell shell.nix`
let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    gobject-introspection
    cargo 
    cargo-tauri # Optional, Only needed if Tauri doesn't work through the traditional way.
    nodejs # Optional, this is for if you have a js frontend
    gcc
    llvmPackages.lld

    fish
    pnpm
    rustup
    rust-analyzer
    sccache
  ];

  buildInputs = with pkgs;[
    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    pango
    webkitgtk_4_1
    openssl
  ];
  shellHook = ''
    export IN_NIX_SHELL=1

    export RUSTC_WRAPPER=${pkgs.sccache}/bin/sccache

    export SHELL=${pkgs.fish}/bin/fish
    if [ -z "$FISH_VERSION" ] && [ -n "$PS1" ]; then
      exec ${pkgs.fish}/bin/fish
    fi
  '';
}
