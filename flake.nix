{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    naersk.url = "github:nix-community/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    naersk,
    rust-overlay,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [rust-overlay.overlays.default];
    };

    rustToolchain = pkgs.rust-bin.nightly.latest.default;
    naerskLib = pkgs.callPackage naersk {inherit rustToolchain;};
  in {
    packages.${system}.default = naerskLib.buildPackage {
      src = ./.;
      cargoLock = ./Cargo.lock;
      buildInputs = [pkgs.glib pkgs.systemd.dev];
      nativeBuildInputs = [pkgs.pkg-config];
      pname = "antumbra";
    };

    devShells.${system}.default = pkgs.mkShell {
      packages = [
        rustToolchain
        pkgs.rust-analyzer
      ];

      buildInputs = with pkgs; [
        glib

        systemd.dev
      ];

      nativeBuildInputs = [pkgs.pkg-config pkgs.libusb1];
    };
  };
}
