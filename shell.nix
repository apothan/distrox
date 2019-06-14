{ pkgs ? (import <nixpkgs> {}) }:

let
  rust = pkgs.rustChannels.stable.rust.override {
    targets = [ "wasm32-unknown-unknown" ];
  };

  env = with pkgs.rustChannels.stable; [
    rust
    cargo
  ];

  dependencies = with pkgs; [
    openssl
    pkgconfig

    # node
    nodePackages.npm
    nodejs
  ];
in

pkgs.stdenv.mkDerivation rec {
  name    = "distrox";
  src     = /var/empty;
  version = "0.0.0";

  buildInputs = env ++ dependencies;

  shellHook = ''
    HISTFILE=${toString ./.history}
  '';
}

