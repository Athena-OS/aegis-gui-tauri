{
  # main
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  # dev
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , ...
    } @ inputs:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = with inputs; [
            fenix.overlays.default
          ];
        };
        aegis-gui-tauri = pkgs.callPackage ./nix/aegis-gui-tauri.nix { };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "development-environment-aegis-gui-tauri";
          WEBKIT_DISABLE_COMPOSITING_MODE = "1"; # essential in NVIDIA + compositor https://github.com/NixOS/nixpkgs/issues/212064#issuecomment-1400202079
          XDG_DATA_DIRS = "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS";
          packages = with pkgs;
            [
              (fenix.stable.withComponents [
                "cargo"
                "rustc"
                "rustfmt"
                "clippy"
                "rust-src"
              ])
              nodePackages.yarn
              glib
              dbus
              cairo
              atk
              openssl
              libsoup
              pango
              gdk-pixbuf
              gtk3
              harfbuzz
              zlib
              gcc
              pkg-config
              webkitgtk
              appimagekit
              cargo-tauri
              nixpkgs-fmt
              gsettings-desktop-schemas
              sqlite
              just
              haskellPackages.udev
              libudev-zero
            ];
        };
        packages.default = aegis-gui-tauri;
      }
    );
}
