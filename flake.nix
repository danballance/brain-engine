{
  description = "DESCRIPTION";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
  flake-utils.lib.eachDefaultSystem (system:
    let
    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs {
      inherit system overlays;
    };

    # Rust toolchain
    rustToolchain = pkgs.rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" "rust-analyzer" "clippy" ];
    };

    # X11 dependencies
    x11BuildInputs = with pkgs; [
      xorg.libX11
      xorg.libXcursor
      xorg.libXi
      xorg.libXrandr
    ];

    # Wayland dependencies
    waylandBuildInputs = with pkgs; [
      libxkbcommon
      wayland
    ];

    # Common build inputs for Bevy
    buildInputs = with pkgs; [
      # Audio
      alsa-lib

      # Graphics
      vulkan-loader
      vulkan-headers

      # System
      systemd
      udev
    ] ++ x11BuildInputs ++ waylandBuildInputs;

    # Native build inputs (build tools)
    nativeBuildInputs = with pkgs; [
      rustToolchain
      pkg-config
			openssl

      # Optional development tools
      cargo-watch
      cargo-edit
      cargo-flamegraph
    ];

    in
    {
      devShells.default = pkgs.mkShell {
        inherit buildInputs nativeBuildInputs;

        shellHook = ''
          # Set up library path for dynamic linking
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath buildInputs}"

          # Set up Vulkan
          export VK_LAYER_PATH="${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d"

          echo "Bevy development environment loaded!"
          echo "Rust version: $(rustc --version)"
          echo "Cargo version: $(cargo --version)"
        '';
      };
    });
}
