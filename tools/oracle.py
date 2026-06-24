#!/usr/bin/env python3
"""Differential accuracy oracle: replay an input fixture through both the Rust
port and FCEUX (the real-NES oracle), then compare curated game-state RAM to
find where the port diverges.

Usage: python3 tools/oracle.py inputs/<file>.replay [--rom rom/lotw.nes]

Run from the repo root inside the nix devshell (needs cargo + fceux + xvfb).

Only a curated set of game-state zero-page bytes is compared — these were
verified to align frame-for-frame between the port and FCEUX (shift 0). The
6502 stack ($0100-$01FF), the APU/sound shadows, OAM, and volatile scratch are
NOT compared (they carry harmless rendering/timing/alignment noise). A field
that diverges for more than JITTER consecutive frames is reported as a real
port bug, pinned to the first frame and the buttons held around it.
"""
import os
import subprocess
import sys

# Curated game-state fields (name -> zero-page offset). Verified clean: these
# match FCEUX every frame on a known-good replay (modulo <=2 frames of
# transition jitter). Add fields here as more are verified stable.
FIELDS = {
    0x40: "character_index", 0x43: "player_x_fine", 0x44: "player_x_tile",
    0x45: "player_y", 0x46: "landing_timer", 0x47: "map_screen_x",
    0x48: "map_screen_y", 0x4E: "fall_frames", 0x4F: "jump_timer",
    0x50: "pose_state", 0x56: "player_pose", 0x57: "player_facing",
    0x58: "player_health", 0x59: "player_magic", 0x5A: "coins",
    0x7B: "scroll_fine_x", 0x7C: "scroll_tile_x",
}
W = 0x800       # bytes dumped per frame ($0000-$07FF)
JITTER = 3      # consecutive-frame mismatches to tolerate (transition jitter)


def expand(replay):
    """Expand a `frame <count> <buttons>` replay into a per-frame button list."""
    bits = {"A": 1, "B": 2, "select": 4, "start": 8,
            "up": 16, "down": 32, "left": 64, "right": 128}
    out = []
    for line in open(replay):
        toks = line.split("#")[0].split()
        if not toks or toks[0] != "frame":
            continue
        b = 0
        for t in toks[2:]:
            b |= bits.get(t, 0)
        out += [b] * int(toks[1])
    return out


def main():
    replay = sys.argv[1]
    rom = "rom/lotw.nes"
    if "--rom" in sys.argv:
        rom = sys.argv[sys.argv.index("--rom") + 1]
    port_bin, fceux_bin = "/tmp/oracle_port.bin", "/tmp/oracle_fceux.bin"

    print(f"[oracle] replay: {replay}")
    subprocess.run(["cargo", "build", "--quiet", "--bin", "replay_oracle"], check=True)
    subprocess.run(["./target/debug/replay_oracle", replay, port_bin, rom], check=True)
    env = dict(os.environ, LOTW_REPLAY=os.path.abspath(replay), LOTW_ORACLE_OUT=fceux_bin)
    subprocess.run(
        ["xvfb-run", "-a", "fceux", "--no-config", "1",
         "--loadlua", "tools/oracle_fceux.lua", rom],
        env=env, check=False,
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
    )

    a = open(fceux_bin, "rb").read()   # oracle (FCEUX)
    b = open(port_bin, "rb").read()    # port
    n = min(len(a), len(b)) // W
    buttons = expand(replay)
    print(f"[oracle] comparing {n} frames across {len(FIELDS)} curated fields\n")

    # For each field, find the first run of > JITTER consecutive mismatches.
    findings = []
    for off, name in sorted(FIELDS.items()):
        run, run_start = 0, None
        for fr in range(2, n):  # skip boot frames
            if a[fr * W + off] != b[fr * W + off]:
                if run == 0:
                    run_start = fr
                run += 1
                if run > JITTER:
                    findings.append((run_start, off, name,
                                     a[run_start * W + off], b[run_start * W + off]))
                    break
            else:
                run = 0

    if not findings:
        print("[oracle] PASS — no curated field diverges beyond transition jitter.")
        return
    findings.sort()
    print(f"[oracle] {len(findings)} field(s) diverge. First divergences:\n")
    for fr, off, name, fceux_v, port_v in findings:
        held = buttons[fr] if fr < len(buttons) else 0
        names = [n for n, m in [("A", 1), ("B", 2), ("sel", 4), ("start", 8),
                                 ("up", 16), ("dn", 32), ("L", 64), ("R", 128)] if held & m]
        print(f"  frame {fr:5}  ${off:02X} {name:16} fceux={fceux_v:3} port={port_v:3}"
              f"  buttons=[{'+'.join(names) or 'none'}]")
    print(f"\n[oracle] earliest: frame {findings[0][0]} in {findings[0][2]} "
          f"-> investigate the routine that writes ${findings[0][1]:02X} there.")


if __name__ == "__main__":
    main()
