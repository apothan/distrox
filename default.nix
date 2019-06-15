{ pkgs ? (import <nixpkgs> {}) }:

let
  env = with pkgs.rustChannels.stable; [
    rust
    cargo
  ];

  dependencies = with pkgs; [
    pkgconfig
    glib
    cairo
    atk
    pango
    gdk_pixbuf
    gtk3

    gnome3.glade
  ];
in

pkgs.stdenv.mkDerivation rec {
    name = "distrox";
    src = /var/empty;
    version = "0.0.0";

    buildInputs = env ++ dependencies;

}

