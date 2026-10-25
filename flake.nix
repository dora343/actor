{
  description = "A reproducible Rust development environment and package";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      naersk,
      rust-overlay,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default;
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in
      {

        packages.default =
          (naersk.lib.${system}.override {
            cargo = rustToolchain;
            rustc = rustToolchain;
          }).buildPackage
            {
              src = ./.;
            };

        devShells.default = pkgs.mkShell {
          name = "${cargoToml.package.name}-dev-shell";

          buildInputs = [ rustToolchain ];

        };
      }
    );
}
