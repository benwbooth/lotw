{
  description = "Legacy of the Wizard — native Rust playable port";

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
      # nixpkgs splits Qt across outputs (qmltyperegistrar/qmlcachegen live in
      # qtdeclarative, not qtbase), but cxx-qt's qt-build-utils only looks in the
      # libexec qmake reports (qtbase). Merge the host tools and wrap qmake to
      # report the merged dir for *_LIBEXECS queries.
      qtMergedLibexecFor = pkgs: pkgs.runCommand "qt-merged-libexec" { } ''
        mkdir -p $out
        for d in ${pkgs.qt6.qtbase}/libexec ${pkgs.qt6.qtdeclarative}/libexec; do
          for f in "$d"/*; do ln -sf "$f" "$out/" 2>/dev/null || true; done
        done
        true
      '';
      qmakeWrapperFor = pkgs:
        let libexec = qtMergedLibexecFor pkgs;
        in pkgs.writeShellScriptBin "qmake" ''
          if [ "$1" = "-query" ]; then
            case "$2" in
              QT_HOST_LIBEXECS*|QT_INSTALL_LIBEXECS*) echo "${libexec}"; exit 0 ;;
            esac
          fi
          exec ${pkgs.qt6.qtbase}/bin/qmake "$@"
        '';
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            pkg-config
            (sdl3gitFor pkgs)
            cargo
            clippy
            rustc
            rustfmt
            # Python + numpy for the agent RL env (PyO3/maturin wrapper over
            # lotw::env::Env returns numpy frames). maturin builds the bindings;
            # use a venv for `maturin develop` (see below). Torch (ROCm) is
            # separate — run it from the rocm/pytorch container, not nix.
            (python3.withPackages (ps: with ps; [ numpy ]))
            maturin
            # Fast linker for the live-edit music JIT rebuild (see music-jit/bench.sh)
            mold
            # Utilities
            p7zip
            xxd
            xvfb-run
            # Emulator for lockstep byte-exact verification against the real ROM
            fceux
            # egui/winit runtime libs for the native asset editor (lotw-editor)
            libxkbcommon
            libGL
            wayland
            libx11
            libxcursor
            libxi
            libxrandr
            # Qt6 toolchain for the cxx-qt editor (qmake/moc, QML/QtQuick, native
            # Wayland platform plugin so touchpad pinch gestures are delivered)
            qt6.qtbase
            qt6.qtdeclarative
            qt6.qtwayland
            qt6.qtshadertools
            # KDE/Plasma integration so QtQuick Controls use the system theme:
            # qqc2-desktop-style provides the `org.kde.desktop` style, breeze the
            # widget style/colours, plasma-integration the KDE platform theme
            # plugin, kio the helpers org.kde.desktop expects.
            kdePackages.qqc2-desktop-style
            kdePackages.breeze
            kdePackages.plasma-integration
            kdePackages.kio
            cmake
            ninja
            watchexec # auto-rebuild/restart the Qt editor on change
          ];
          # winit/glow dlopen the windowing + GL libs at runtime, so they must be
          # on LD_LIBRARY_PATH (being build inputs alone is not enough). Qt needs
          # QMAKE for cxx-qt-build plus QML/plugin paths at runtime.
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
              pkgs.libxkbcommon
              pkgs.libGL
              pkgs.wayland
              pkgs.libx11
              pkgs.libxcursor
              pkgs.libxi
              pkgs.libxrandr
              pkgs.qt6.qtbase
              pkgs.qt6.qtdeclarative
            ]}:''${LD_LIBRARY_PATH:-}"
            export RUST_SRC_PATH="${pkgs.rustPlatform.rustLibSrc}/lib/rustlib/src/rust/library"
            export QMAKE="${qmakeWrapperFor pkgs}/bin/qmake"
            export QT_QPA_PLATFORM=wayland
            # QML import + plugin paths. Include qqc2-desktop-style (org.kde.desktop
            # QML style) and the KDE/breeze widget styles + plasma-integration
            # platform-theme plugin so the controls follow the system KDE theme,
            # not the flat "Basic" fallback.
            export QML2_IMPORT_PATH="${pkgs.qt6.qtdeclarative}/lib/qt-6/qml:${pkgs.kdePackages.qqc2-desktop-style}/lib/qt-6/qml:${pkgs.kdePackages.breeze}/lib/qt-6/qml"
            export QML_IMPORT_PATH="$QML2_IMPORT_PATH"
            export QT_PLUGIN_PATH="${pkgs.qt6.qtbase}/lib/qt-6/plugins:${pkgs.qt6.qtwayland}/lib/qt-6/plugins:${pkgs.qt6.qtdeclarative}/lib/qt-6/plugins:${pkgs.kdePackages.plasma-integration}/lib/qt-6/plugins:${pkgs.kdePackages.breeze}/lib/qt-6/plugins:${pkgs.kdePackages.kio}/lib/qt-6/plugins"
            # Default to the KDE desktop style + platform theme (matches the rest
            # of the Plasma session); both overridable. QT_QPA_PLATFORMTHEME is
            # left to KDE auto-detection (XDG_CURRENT_DESKTOP) unless preset.
            export QT_QUICK_CONTROLS_STYLE="''${QT_QUICK_CONTROLS_STYLE:-org.kde.desktop}"
          '';
        };
      });

      # `nix build .#play` / `nix run .#play -- rom/lotw.nes`
      # Source is filtered to code only: the (gitignored) ROM never enters the
      # Nix store. The ROM is supplied at runtime as an argument.
      packages = forAllSystems (pkgs: rec {
        play = pkgs.rustPlatform.buildRustPackage {
          pname = "lotw-play";
          version = "0.1.0";
          src = nixpkgs.lib.fileset.toSource {
            root = ./.;
            fileset = nixpkgs.lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              ./build.rs
              ./music-macros
              ./lotw-music
              ./music-jit
              ./src
              ./tests
              ./fixtures
              ./config
            ];
          };
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [ "--features" "sdl" "--bin" "play" ];
          cargoTestFlags = [ "--lib" "--tests" ];
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ (sdl3gitFor pkgs) ];
        };
        default = play;
      });

      apps = forAllSystems (pkgs: rec {
        play = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.play}/bin/play";
        };
        default = play;
      });
    };
}
