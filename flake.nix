{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };
      in {
        devShell = with pkgs;
          mkShell {
            buildInputs = [
              just
              bash
              rustup
              pnpm
              nodejs
              wasm-pack
              zip
              pandoc
            ];
          };
      }
    );
}
