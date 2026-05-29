{
  description = "Legacy of the Wizard — matching 6502 decompilation + C port";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = f:
        nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
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
            SDL2
            # RE tooling
            cargo
            clippy
            rustc
            rustfmt
            python3
            # Utilities
            p7zip
            xxd
            xvfb
          ];
        };
      });
    };
}
