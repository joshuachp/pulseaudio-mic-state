{
  description = "Get the status of the microphone with pulseaudio ";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nmattia/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, fenix, flake-utils, naersk }:
    let
      supportedSystems = with flake-utils.lib.system; [
        x86_64-linux
        x86_64-darwin
        aarch64-linux
        aarch64-darwin
      ];
      eachSystem = flake-utils.lib.eachSystem supportedSystems;
    in
    eachSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        toolchain = fenix.packages.${system}.stable;
        naersk' = pkgs.callPackage naersk {
          inherit (toolchain) cargo rustc;
        };
      in
      rec {
        packages.default = naersk'.buildPackage {
          pname = "pulseaudio-mic-state";
          src = ./.;
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          buildInputs = with pkgs; [
            libpulseaudio
          ];
        };
        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };
        devShells.default = pkgs.mkShell {
          inputsFrom = [ packages.default ];
          packages = with pkgs; [
            pre-commit
            rust-analyzer
          ];
        };
      }
    );
}
