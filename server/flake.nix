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
        rustChannel = mozilla.rustChannelOf {
          date = "2020-12-12";
          channel = "nightly";
          sha256 = "sha256-9ly+eRpdhgiQ0047m/deeWfHNlDYF97x5vCxsNLje4s=";
        };

        # TODO: theoretically `rust-src` should already be included
        # https://github.com/mozilla/nixpkgs-mozilla/blob/8c007b60731c07dd7a052cce508de3bb1ae849b4/rust-overlay.nix#L228-L255
        rust = rustChannel.rust.override { extensions = [ "rust-src" ]; };

        # override the version used in naersk
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
        };
      in
        with pkgs; rec {
          # nix <command> .
          #
          # e.g. `nix run .` runs the `defaultApp`
          defaultApp = apps.server;
          defaultPackage = packages.server;

          # `nix run`
          apps.server = utils.lib.mkApp {
            drv = packages.server;

            # `naersk` builds packages whose derivations have names of the form
            # "{name}-{version}" but whose binaries exclude the version in the
            # filename. for `mkApp` to find the correct executable target
            # specify name w/o a version here.
            name = "sonar";
          };

          # `nix build`
          packages = {
            docker = pkgs.dockerTools.streamLayeredImage {
              name = "sonar";
              contents = [ self.packages.x86_64-linux.server ];
              config.Cmd = [ "sonar" ];
            };

            # version & name are parsed from Cargo.toml
            server = naersk-lib.buildPackage {
              src = ./.;

              # buildInputs = [] ++ lib.optionals stdenv.isLinux [
              #   pkg-config
              #   openssl
              # ];
            };
          };


          # `nix develop`
          devShell = pkgs.mkShell {
            # buildInputs = [ pkgs.openssl pkgs.cacert ];
            # buildInputs = with pkgs; [ pkgconfig openssl cmake zlib libgit2 ];

            # supply the specific rust version
            nativeBuildInputs = with pkgs; with stdenv; [ rust ]
            # ref - https://stackoverflow.com/a/51161923
            ++ lib.optionals isDarwin [ darwin.apple_sdk.frameworks.Security ];
            # ++ lib.optionals isLinux [ pkg-config openssl ];

            shellHook = ''
              # based on testing on my machine this may not be necessary but left
              # in for now biasing towards explicitness
              # see https://github.com/mozilla/nixpkgs-mozilla/issues/238
              export RUST_SRC_PATH="${rustChannel.rust-src}/lib/rustlib/src/rust/library"
              export SOURCE_CODE="${self.outPath}"
            '';
            # ++ lib.optionals isLinux ''
            # # CARGO_HTTP_CAINFO="/nix/store/gdgnc8r39yz1g74bw674flzdw759ml1c-nss-cacert-3.56/etc/ssl/certs/ca-bundle.crt"
            # export CARGO_HTTP_CAINFO="${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
            # export SSL_CERT_FILE="${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
            # '';
          };
        }
    );
}
