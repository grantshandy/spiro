{
  nixConfig.bash-prompt = "[nix-develop]$ ";

  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pname = "spiro";
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        libPath = with pkgs;
          lib.makeLibraryPath [
            libGL
            libxkbcommon
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ];
      in {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          doCheck = true;
          pname = pname;
          nativeBuildInputs = [ pkgs.makeWrapper ];
          buildInputs = with pkgs; [ xorg.libxcb ];
          postInstall = ''
            wrapProgram "$out/bin/${pname}" --prefix LD_LIBRARY_PATH : "${libPath}"
          '';
        };

        defaultApp = utils.lib.mkApp { drv = self.defaultPackage."${system}"; };

        devShell = with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
              rustfmt
              pre-commit
              rustPackages.clippy
              rust-analyzer
              xorg.libxcb
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            LD_LIBRARY_PATH = libPath;
          };
      });
}
