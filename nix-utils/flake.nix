{
  inputs = {
    nixpkgs.url = "nixpkgs/f6db44a8daa5";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
        with pkgs; {
          devShells.default = mkShell {
            buildInputs = [
              bacon
              aider-chat
              trunk
            ];
          };
        }
    );
}
