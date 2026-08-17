{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      inherit (nixpkgs) lib;

      forEachSystem =
        fn:
        lib.genAttrs lib.systems.flakeExposed (
          system:
          fn {
            inherit system;
            pkgs = nixpkgs.legacyPackages.${system};
          }
        );
    in
    {
      formatter = forEachSystem (
        { pkgs, ... }:
        let
          rustfmt' = pkgs.rustfmt.override { asNightly = true; };
        in
        pkgs.writeShellApplication {
          name = "fmt";
          runtimeInputs = builtins.attrValues {
            inherit (pkgs) nixfmt taplo fd;
            inherit rustfmt';
          };
          text = ''
            fd "$@" -t f -e nix -X nixfmt '{}'
            fd "$@" -t f -e rs -X rustfmt '{}'
            fd "$@" -t f -e toml -X taplo fmt '{}'
          '';
        }
      );

      devShells = forEachSystem (
        { pkgs, ... }:
        let
          rustfmt' = pkgs.rustfmt.override { asNightly = true; };
        in
        {
          default = pkgs.mkShell {
            packages = builtins.attrValues {
              inherit (pkgs) rustc cargo clippy;
              inherit rustfmt';
            };
          };
        }
      );
    };
}
