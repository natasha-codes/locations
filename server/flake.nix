{
  description = "sonar - find your friends";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    mozillapkgs = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk, mozillapkgs }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";

        # get our specific rust version
        mozilla = pkgs.callPackage (mozillapkgs + "/package-set.nix") {};
        rust = (
          mozilla.rustChannelOf {
            date = "2020-12-07";
            channel = "nightly";
            sha256 = "sha256-MXTXVVKZ1H1LG4Y+SlKMHjVv/HI6U8bkCooNYI7Hhl0=";
          }
        ).rust;

        # override the version used in naersk
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
        };
      in
        rec {
          # `nix build`
          packages.server = naersk-lib.buildPackage {
            pname = "sonar";
            root = ./.;
          };
          defaultPackage = packages.server;

          # `nix run`
          apps.server = utils.lib.mkApp {
            drv = packages.server;
          };
          defaultApp = apps.server;

          # `nix develop`
          devShell = pkgs.mkShell {
            # supply the specific rust version
            nativeBuildInputs = with pkgs; [ rust ];
          };
        }
    );
}
