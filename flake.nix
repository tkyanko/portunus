{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          name = "portunus";
          version = "0.1.0";
          src = ./.;
          nativeBuildInputs = with pkgs; [ sqlite ];
        };

        dockerImage = pkgs.dockerTools.buildImage {
          name = "portunus";
          tag = "v0.1.0";
          config = {
            Cmd = [ "${defaultPackage}/bin/portunus" ];
            WorkingDir = "/data";
          };
        };

      in rec {
        packages = {
          rustPackage = defaultPackage;
          docker = dockerImage;
        };
        defaultPackage = dockerImage;
        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo ];
        };
      }
    );
}
