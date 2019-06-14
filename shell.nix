{ pkgs ? (import <nixpkgs> {}) }:

let
  # we need unstable nixpkgs for electron 5
  unstable = let
    rev    = "168d1031af4916354491a1b1dd103258cfd26c0e";
    sha256 = "1mzir6zlf1lxcl7l1pz82nziwi68gf7zjpwbqrj7sv3vn6jknyxw";

    tarball = builtins.fetchTarball {
      inherit sha256;
      url = "https://github.com/NixOS/nixpkgs-channels/archive/${rev}.tar.gz";
    };
  in import tarball {
    config.allowUnfree = false;
  };

  rust = pkgs.rustChannels.stable.rust.override {
    targets = [ "wasm32-unknown-unknown" ];
  };

  env = with pkgs.rustChannels.stable; [
    rust
    cargo
  ];

  dependencies = with pkgs; [
    # node
    nodePackages.npm
    nodePackages.node2nix
    nodejs

  ] ++ (with unstable; [
    electron_5
  ]);

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

