{
  description = "Legacy of the Wizard — matching 6502 decompilation + C port";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = f:
        nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      # SDL3 from git main: nixpkgs' release predates the new Steam Controller
      # (2026, VID:PID 28DE:1302 "Triton") support merged 2026-05-14. Build from
      # source so the HIDAPI driver decodes the controller (works without Steam).
      sdl3gitFor = pkgs: pkgs.sdl3.overrideAttrs (old: {
        version = "git-20260531";
        src = pkgs.fetchFromGitHub {
          owner = "libsdl-org";
          repo = "SDL";
          rev = "96c03dc66e89765d5f81123c2056706dd6f28ea7";
          hash = "sha256-Fv22zkiAgI75TTiEfwTTUNg/jEtN+SxTvI3wr2kGAqM=";
        };
        patches = [ ];   # nixpkgs' release patches don't apply to git main
      });
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            # Matching disassembly toolchain (ca65/ld65/da65 round-trip to the ROM)
            cc65
            # Reference emulator: tracing, code/data coverage, replay capture
            fceux
            # C port build + tooling
            gcc
            gnumake
            cmake
            pkg-config
            (sdl3gitFor pkgs)
            # RE tooling
            cargo
            clippy
            rustc
            rustfmt
            python3
            # Utilities
            p7zip
            xxd
            xvfb-run
          ];
        };
      });

      # `nix build .#play` / `nix run .#play -- rom/lotw.nes`
      # Source is filtered to code only: the (gitignored) ROM never enters the
      # Nix store. The ROM is supplied at runtime as an argument.
      packages = forAllSystems (pkgs: rec {
        play = pkgs.stdenv.mkDerivation {
          pname = "lotw-play";
          version = "0.1.0";
          src = nixpkgs.lib.fileset.toSource {
            root = ./.;
            fileset = nixpkgs.lib.fileset.unions [
              ./CMakeLists.txt
              ./src
              ./test
            ];
          };
          nativeBuildInputs = [ pkgs.cmake pkgs.pkg-config ];
          buildInputs = [ (sdl3gitFor pkgs) ];
          installPhase = ''
            runHook preInstall
            install -Dm755 play $out/bin/lotw-play
            runHook postInstall
          '';
        };
        default = play;
      });

      apps = forAllSystems (pkgs: rec {
        play = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.play}/bin/lotw-play";
        };
        default = play;
      });
    };
}
