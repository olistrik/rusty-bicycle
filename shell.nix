{pkgs ? import <nixpkgs> {}}:

pkgs.mkShell rec {
  propagatedBuildInputs = with pkgs; [
    cargo cargo-edit rustc rustfmt dbus pkgconfig
  ];
}
