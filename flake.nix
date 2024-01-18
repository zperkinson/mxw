{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [pkg-config libudev-zero ];
          buildInputs = with pkgs; [ cargo rustc rustfmt pre-commit rustPackages.clippy ];
        };
        # devShell = with pkgs; mkShell {
        #   nativeBuildInputs = [pkg-config libudev-zero libgudev];
        #   buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy ];
        #   RUST_SRC_PATH = rustPlatform.rustLibSrc;
        # };
      });
}
