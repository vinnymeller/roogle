{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        naersk-lib = pkgs.callPackage naersk { };
        rustVersion = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
      in
      {

        formatter = pkgs.nixpkgs-fmt;

        defaultPackage = naersk-lib.buildPackage ./.;

        devShell = with pkgs; mkShell {
          buildInputs = [
            rustVersion
            rust-analyzer
            ];
        };

      });
}
